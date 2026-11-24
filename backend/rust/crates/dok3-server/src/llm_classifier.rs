//! Air-gapped local/LAN OpenAI-compatible LLM classifier with strict validation and rule fallback.

use std::io::{Read, Write};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream};
use std::time::Duration;

use dok3_domain::{
    Document, normalize_classification_terms, validate_classification_category,
    validate_classification_summary,
};
use serde::Deserialize;

use crate::classification::RuleOutput;

const DEFAULT_CLASSIFIER_MODEL: &str = "qwen2.5-3b-instruct";
const LLM_TIMEOUT: Duration = Duration::from_secs(3);
const MAX_PROMPT_BODY_CHARS: usize = 2000;

/// Validated air-gapped HTTP endpoint (loopback or RFC1918 IPv4 literal only).
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ValidatedLocalEndpoint {
    pub host_header: String,
    pub addrs: Vec<SocketAddr>,
    pub path: String,
}

/// Validates that `raw` is an `http://` URL pointing strictly to loopback (`localhost`,
/// `127.0.0.0/8`, `[::1]`) or an RFC1918 private IPv4 literal (`10.0.0.0/8`, `172.16.0.0/12`,
/// `192.168.0.0/16`). Rejects `https://`, link-local metadata IPs, public IPs, and external DNS.
pub(crate) fn validate_classifier_endpoint(raw: &str) -> Result<ValidatedLocalEndpoint, String> {
    let trimmed = raw.trim();
    let rest = trimmed
        .strip_prefix("http://")
        .ok_or_else(|| "DOK3_CLASSIFIER_ENDPOINT must use http:// (air-gap guard)".to_string())?;
    let (authority, path) = match rest.find('/') {
        Some(idx) => (&rest[..idx], &rest[idx..]),
        None => (rest, "/v1/chat/completions"),
    };
    if authority.is_empty() || authority.contains('@') {
        return Err("invalid DOK3_CLASSIFIER_ENDPOINT authority".to_string());
    }

    let (host, port) = if let Some(after_bracket) = authority.strip_prefix('[') {
        let end = after_bracket
            .find(']')
            .ok_or_else(|| "invalid IPv6 host in DOK3_CLASSIFIER_ENDPOINT".to_string())?;
        let host = &after_bracket[..end];
        let port_part = &after_bracket[end + 1..];
        let port = if port_part.is_empty() {
            80
        } else if let Some(p) = port_part.strip_prefix(':') {
            p.parse::<u16>()
                .map_err(|_| "invalid port in DOK3_CLASSIFIER_ENDPOINT".to_string())?
        } else {
            return Err("invalid port syntax in DOK3_CLASSIFIER_ENDPOINT".to_string());
        };
        (host, port)
    } else if let Some(colon) = authority.rfind(':') {
        let host = &authority[..colon];
        let port = authority[colon + 1..]
            .parse::<u16>()
            .map_err(|_| "invalid port in DOK3_CLASSIFIER_ENDPOINT".to_string())?;
        (host, port)
    } else {
        (authority, 80)
    };

    let addrs = if host.eq_ignore_ascii_case("localhost") {
        vec![SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), port)]
    } else if host == "::1" {
        vec![SocketAddr::new(
            IpAddr::V6(std::net::Ipv6Addr::LOCALHOST),
            port,
        )]
    } else if let Ok(v4) = host.parse::<Ipv4Addr>() {
        if !(v4.is_loopback() || v4.is_private()) {
            return Err(format!(
                "DOK3_CLASSIFIER_ENDPOINT IP '{v4}' is not loopback or RFC1918 private"
            ));
        }
        vec![SocketAddr::new(IpAddr::V4(v4), port)]
    } else {
        return Err(format!(
            "DOK3_CLASSIFIER_ENDPOINT host '{host}' rejected by air-gap guard (use localhost, ::1, 127.x, or RFC1918 IP)"
        ));
    };

    Ok(ValidatedLocalEndpoint {
        host_header: authority.to_string(),
        addrs,
        path: if path.is_empty() {
            "/v1/chat/completions".to_string()
        } else {
            path.to_string()
        },
    })
}

/// Offline LLM classifier calling a validated local/LAN OpenAI-compatible endpoint.
#[derive(Debug, Clone)]
pub(crate) struct OfflineLlmClassifier {
    endpoint: ValidatedLocalEndpoint,
    model: String,
}

impl OfflineLlmClassifier {
    pub(crate) fn new(endpoint_raw: &str, model: Option<&str>) -> Result<Self, String> {
        let endpoint = validate_classifier_endpoint(endpoint_raw)?;
        let model = model
            .map(str::trim)
            .filter(|m| !m.is_empty())
            .unwrap_or(DEFAULT_CLASSIFIER_MODEL)
            .to_string();
        Ok(Self { endpoint, model })
    }

    pub(crate) fn classify(&self, doc: &Document) -> Result<RuleOutput, String> {
        let excerpt: String = doc.body.chars().take(MAX_PROMPT_BODY_CHARS).collect();
        let req_body = serde_json::json!({
            "model": self.model,
            "temperature": 0,
            "response_format": { "type": "json_object" },
            "messages": [
                {
                    "role": "system",
                    "content": "Classify the document into JSON with keys: category (one of guide, manual, code, policy, design), summary (<=200 chars), tags (array of strings), keywords (array of strings)."
                },
                {
                    "role": "user",
                    "content": format!("Title: {}\n\n{}", doc.title, excerpt)
                }
            ]
        });
        let body_bytes =
            serde_json::to_vec(&req_body).map_err(|err| format!("serialize LLM request: {err}"))?;

        let mut stream = None;
        for addr in &self.endpoint.addrs {
            if let Ok(s) = TcpStream::connect_timeout(addr, LLM_TIMEOUT) {
                stream = Some(s);
                break;
            }
        }
        let mut stream =
            stream.ok_or_else(|| "failed to connect to local classifier endpoint".to_string())?;
        let _ = stream.set_read_timeout(Some(LLM_TIMEOUT));
        let _ = stream.set_write_timeout(Some(LLM_TIMEOUT));

        let http_req = format!(
            "POST {} HTTP/1.1\r\nHost: {}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
            self.endpoint.path,
            self.endpoint.host_header,
            body_bytes.len()
        );
        stream
            .write_all(http_req.as_bytes())
            .and_then(|()| stream.write_all(&body_bytes))
            .map_err(|err| format!("write LLM HTTP request: {err}"))?;

        let mut raw_resp = Vec::new();
        stream
            .read_to_end(&mut raw_resp)
            .map_err(|err| format!("read LLM HTTP response: {err}"))?;

        let body = extract_http_body(&raw_resp)?;
        parse_and_validate_llm_json(&body)
    }
}

fn extract_http_body(raw: &[u8]) -> Result<Vec<u8>, String> {
    let sep = raw
        .windows(4)
        .position(|w| w == b"\r\n\r\n")
        .ok_or_else(|| "malformed HTTP response from classifier".to_string())?;
    let header_str = std::str::from_utf8(&raw[..sep])
        .map_err(|_| "non-UTF-8 HTTP headers from classifier".to_string())?;
    let status_line = header_str.lines().next().unwrap_or("");
    if !status_line.contains(" 200 ") {
        return Err(format!("classifier HTTP status error: {status_line}"));
    }
    let body_raw = &raw[sep + 4..];
    if header_str
        .to_ascii_lowercase()
        .contains("transfer-encoding: chunked")
    {
        decode_chunked(body_raw)
    } else {
        Ok(body_raw.to_vec())
    }
}

fn decode_chunked(mut input: &[u8]) -> Result<Vec<u8>, String> {
    let mut out = Vec::new();
    while let Some(crlf) = input.windows(2).position(|w| w == b"\r\n") {
        let size_str = std::str::from_utf8(&input[..crlf])
            .map_err(|_| "invalid chunk size".to_string())?
            .split(';')
            .next()
            .unwrap_or("")
            .trim();
        let size = usize::from_str_radix(size_str, 16)
            .map_err(|_| "invalid hex chunk size".to_string())?;
        input = &input[crlf + 2..];
        if size == 0 {
            break;
        }
        if input.len() < size + 2 {
            return Err("truncated chunked body".to_string());
        }
        out.extend_from_slice(&input[..size]);
        input = &input[size + 2..];
    }
    Ok(out)
}

#[derive(Deserialize)]
struct ChatCompletionResponse {
    choices: Vec<ChatChoice>,
}

#[derive(Deserialize)]
struct ChatChoice {
    message: ChatMessage,
}

#[derive(Deserialize)]
struct ChatMessage {
    content: String,
}

#[derive(Deserialize)]
struct RawClassifierOutput {
    category: String,
    summary: String,
    #[serde(default)]
    tags: Vec<String>,
    #[serde(default)]
    keywords: Vec<String>,
}

fn parse_and_validate_llm_json(body: &[u8]) -> Result<RuleOutput, String> {
    let envelope: ChatCompletionResponse =
        serde_json::from_slice(body).map_err(|err| format!("decode chat completion: {err}"))?;
    let content = envelope
        .choices
        .first()
        .map(|c| c.message.content.as_str())
        .ok_or_else(|| "classifier returned no choices".to_string())?;
    let raw: RawClassifierOutput = serde_json::from_str(content)
        .map_err(|err| format!("decode classifier JSON content: {err}"))?;

    let category = validate_classification_category(&raw.category).map_err(|e| e.to_string())?;
    let summary = validate_classification_summary(&raw.summary).map_err(|e| e.to_string())?;
    let tags = normalize_classification_terms("tags", &raw.tags).map_err(|e| e.to_string())?;
    let keywords =
        normalize_classification_terms("keywords", &raw.keywords).map_err(|e| e.to_string())?;
    Ok(RuleOutput {
        category,
        summary,
        tags,
        keywords,
    })
}

#[cfg(test)]
mod tests {
    use std::io::{Read, Write};
    use std::net::TcpListener;
    use std::thread;

    use dok3_domain::{AclScope, Document, DocumentId, RevisionId, SourceId};

    use super::{OfflineLlmClassifier, validate_classifier_endpoint};

    fn sample_doc() -> Document {
        Document {
            id: DocumentId::new("llm-1"),
            revision_id: RevisionId::new("rev-1"),
            source_id: SourceId::new("src"),
            source: "files".into(),
            title: "테스트 문서".into(),
            summary: String::new(),
            category: "guide".into(),
            topic: None,
            tag: "Rust".into(),
            tags: vec![],
            keywords: vec![],
            language: "ko".into(),
            version: "1.0".into(),
            date: None,
            source_uri: "file:///doc.md".into(),
            sample_path: None,
            acl_scope: AclScope::Public,
            content_hash: "abc".into(),
            body: "본문 내용입니다.".into(),
            markdown: None,
            sections: vec![],
        }
    }

    #[test]
    fn air_gap_guard_accepts_loopback_and_rfc1918_and_rejects_external() {
        assert!(validate_classifier_endpoint("http://127.0.0.1:11434/v1/chat/completions").is_ok());
        assert!(validate_classifier_endpoint("http://localhost:8000/v1/chat/completions").is_ok());
        assert!(
            validate_classifier_endpoint("http://192.168.1.50:8080/v1/chat/completions").is_ok()
        );
        assert!(validate_classifier_endpoint("http://10.0.0.5:8080").is_ok());

        assert!(validate_classifier_endpoint("https://127.0.0.1:8080").is_err());
        assert!(validate_classifier_endpoint("http://169.254.169.254/latest").is_err());
        assert!(validate_classifier_endpoint("http://8.8.8.8:8080").is_err());
        assert!(validate_classifier_endpoint("http://api.openai.com/v1/chat/completions").is_err());
    }

    #[test]
    fn mock_local_llm_server_classifies_and_rejects_invalid_category() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();
        let handle = thread::spawn(move || {
            for idx in 0..2 {
                let (mut sock, _) = listener.accept().unwrap();
                let mut buf = [0u8; 4096];
                let _ = sock.read(&mut buf);
                let content = if idx == 0 {
                    r#"{"category":"policy","summary":"AI 요약","tags":["보안"],"keywords":["정책"]}"#
                } else {
                    r#"{"category":"invalid-cat","summary":"bad","tags":[],"keywords":[]}"#
                };
                let json = serde_json::json!({
                    "choices": [{ "message": { "content": content } }]
                })
                .to_string();
                let resp = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                    json.len(),
                    json
                );
                sock.write_all(resp.as_bytes()).unwrap();
                sock.flush().unwrap();
                let _ = sock.shutdown(std::net::Shutdown::Write);
            }
        });

        let classifier = OfflineLlmClassifier::new(
            &format!("http://127.0.0.1:{port}/v1/chat/completions"),
            Some("qwen2.5-3b-instruct"),
        )
        .unwrap();

        let ok = classifier.classify(&sample_doc()).unwrap();
        assert_eq!(ok.category, "policy");
        assert_eq!(ok.summary, "AI 요약");
        assert_eq!(ok.tags, vec!["보안"]);

        assert!(classifier.classify(&sample_doc()).is_err());
        handle.join().unwrap();
    }
}
