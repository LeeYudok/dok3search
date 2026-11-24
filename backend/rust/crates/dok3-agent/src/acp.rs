//! Agent Communication Protocol (ACP) adapter delegating to [`SearchService`].

use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::sync::Arc;

use dok3_core::Dok3Error;
use dok3_domain::{
    DocumentDetail, DocumentId, Principal, SearchRequest, SearchResponse, SectionId,
    SectionReadRequest, SectionReadResponse,
};
use dok3_search::SearchService;
use serde::{Deserialize, Serialize};

use crate::server::MAX_JSONRPC_LINE_BYTES;

/// ACP operation envelope routed through [`AcpAdapter`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "action", rename_all = "snake_case")]
pub enum AcpRequest {
    Search {
        query: String,
        #[serde(default = "default_limit")]
        limit: usize,
    },
    GetDocument {
        document_id: String,
        #[serde(default)]
        toc_only: bool,
        #[serde(default)]
        max_chars: Option<usize>,
    },
    GetSection {
        document_id: String,
        section_id: String,
    },
}

const fn default_limit() -> usize {
    10
}

/// ACP operation response payload.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum AcpResponse {
    Search(SearchResponse),
    Document(Box<DocumentDetail>),
    Section(SectionReadResponse),
}

/// Stateless ACP adapter enforcing per-call [`Principal`] ACL through [`SearchService`].
#[derive(Clone)]
pub struct AcpAdapter {
    search: Arc<SearchService>,
}

impl AcpAdapter {
    #[must_use]
    pub fn new(search: Arc<SearchService>) -> Self {
        Self { search }
    }

    pub fn handle(&self, principal: &Principal, req: AcpRequest) -> Result<AcpResponse, Dok3Error> {
        match req {
            AcpRequest::Search { query, limit } => {
                let resp = self.search.search(
                    principal,
                    &SearchRequest {
                        query,
                        limit,
                        ..SearchRequest::default()
                    },
                )?;
                Ok(AcpResponse::Search(resp))
            }
            AcpRequest::GetDocument {
                document_id,
                toc_only,
                max_chars,
            } => {
                let mut doc = self
                    .search
                    .get_document(principal, &DocumentId::new(document_id))?;
                if doc.body.trim().is_empty()
                    && let Some(md) = doc.markdown.take()
                {
                    doc.body = md;
                }
                doc.markdown = None;
                let cap = max_chars
                    .unwrap_or(16_000)
                    .clamp(1, dok3_domain::MAX_SECTION_MAX_CHARS);
                if toc_only {
                    doc.body.clear();
                } else if doc.body.chars().count() > cap {
                    doc.body = doc.body.chars().take(cap).collect();
                }
                Ok(AcpResponse::Document(Box::new(doc)))
            }
            AcpRequest::GetSection {
                document_id,
                section_id,
            } => {
                let section = self.search.get_section(
                    principal,
                    &SectionReadRequest {
                        document_id: DocumentId::new(document_id),
                        section_id: SectionId::new(section_id),
                        cursor: None,
                        max_chars: 4000,
                    },
                )?;
                Ok(AcpResponse::Section(section))
            }
        }
    }

    /// Serves newline-delimited ACP JSON requests over stdin/stdout until EOF.
    pub fn serve_stdio(&self, principal: &Principal) -> io::Result<()> {
        let stdin = io::stdin();
        let stdout = io::stdout();
        self.serve_io(
            principal,
            BufReader::new(stdin.lock()),
            BufWriter::new(stdout.lock()),
        )
    }

    /// Serves newline-delimited ACP JSON requests over arbitrary buffered streams.
    pub fn serve_io<R: BufRead, W: Write>(
        &self,
        principal: &Principal,
        mut reader: R,
        mut writer: W,
    ) -> io::Result<()> {
        let mut buf = Vec::new();
        loop {
            buf.clear();
            let n = reader.read_until(b'\n', &mut buf)?;
            if n == 0 {
                break;
            }
            if buf.len() > MAX_JSONRPC_LINE_BYTES {
                let err = serde_json::json!({
                    "ok": false,
                    "error": { "code": "invalid_input", "message": "ACP request line exceeds maximum byte limit" }
                });
                writeln!(writer, "{err}")?;
                writer.flush()?;
                continue;
            }
            let line = match std::str::from_utf8(&buf) {
                Ok(s) => s.trim(),
                Err(_) => {
                    let err = serde_json::json!({
                        "ok": false,
                        "error": { "code": "invalid_input", "message": "ACP request must be valid UTF-8" }
                    });
                    writeln!(writer, "{err}")?;
                    writer.flush()?;
                    continue;
                }
            };
            if line.is_empty() {
                continue;
            }
            let payload = match serde_json::from_str::<AcpRequest>(line) {
                Ok(req) => match self.handle(principal, req) {
                    Ok(result) => serde_json::json!({ "ok": true, "result": result }),
                    Err(err) => serde_json::json!({
                        "ok": false,
                        "error": { "code": err.code(), "message": err.to_string() }
                    }),
                },
                Err(err) => serde_json::json!({
                    "ok": false,
                    "error": { "code": "invalid_input", "message": format!("invalid ACP request JSON: {err}") }
                }),
            };
            writeln!(writer, "{payload}")?;
            writer.flush()?;
        }
        Ok(())
    }
}
