//! Built-in Korean sample documents and optional `dok3search-design` loader.

use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

use dok3_domain::{
    AclScope, Document, DocumentFormatLinks, DocumentId, DocumentSection, RevisionId, SectionId,
    SourceId,
};

#[derive(Debug, Default)]
pub struct SeedDataset {
    pub documents: Vec<Document>,
    pub pairs: Vec<DocumentFormatLinks>,
}

#[derive(Debug, Clone)]
struct DesignPairEntry {
    title: String,
    markdown: String,
    yaml: String,
}

#[derive(Debug, Default)]
struct DesignManifest {
    entrypoints: Vec<DesignPairEntry>,
    tree_order: HashMap<String, usize>,
}

/// Loads `dok3search-design` files (if `design_dir` exists) followed by the 20 built-in samples.
#[must_use]
pub fn load_seed_dataset(design_dir: Option<&Path>) -> SeedDataset {
    let mut dataset = SeedDataset::default();

    if let Some(dir) = design_dir.filter(|path| path.is_dir()) {
        let (design_docs, pairs) = load_design_documents(dir);
        dataset.documents.extend(design_docs);
        dataset.pairs.extend(pairs);
    }

    dataset.documents.extend(builtin_sample_documents());
    dataset
}

fn load_design_documents(dir: &Path) -> (Vec<Document>, Vec<DocumentFormatLinks>) {
    let manifest = fs::read_to_string(dir.join("index.yaml"))
        .map(|raw| parse_design_index_yaml(&raw))
        .unwrap_or_default();

    let mut files = Vec::new();
    collect_design_files(dir, dir, &mut files);

    let mut docs: Vec<Document> = files
        .into_iter()
        .filter_map(|(sample_path, content)| {
            build_design_document(&sample_path, &content, &manifest)
        })
        .collect();

    let fallback_rank = manifest.tree_order.len();
    docs.sort_by(|a, b| {
        let rank_a = a
            .sample_path
            .as_deref()
            .and_then(|p| manifest.tree_order.get(p).copied())
            .unwrap_or(fallback_rank);
        let rank_b = b
            .sample_path
            .as_deref()
            .and_then(|p| manifest.tree_order.get(p).copied())
            .unwrap_or(fallback_rank);
        rank_a
            .cmp(&rank_b)
            .then_with(|| a.sample_path.cmp(&b.sample_path))
    });

    let mut id_by_sample_path = HashMap::new();
    for doc in &docs {
        if let Some(sample_path) = &doc.sample_path {
            id_by_sample_path.insert(sample_path.as_str(), doc.id.clone());
        }
    }

    let pairs = manifest
        .entrypoints
        .iter()
        .filter_map(|entry| {
            let markdown_id = id_by_sample_path.get(entry.markdown.as_str())?.clone();
            let yaml_id = id_by_sample_path.get(entry.yaml.as_str())?.clone();
            Some(DocumentFormatLinks {
                markdown_id,
                yaml_id,
            })
        })
        .collect();

    (docs, pairs)
}

fn collect_design_files(root: &Path, current: &Path, out: &mut Vec<(String, String)>) {
    let Ok(entries) = fs::read_dir(current) else {
        return;
    };
    let mut paths: Vec<_> = entries.filter_map(Result::ok).map(|e| e.path()).collect();
    paths.sort();

    for path in paths {
        let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
            continue;
        };
        if name.starts_with('.') || name == "LICENSE" {
            continue;
        }
        if path.is_dir() {
            collect_design_files(root, &path, out);
            continue;
        }
        let is_supported =
            name.ends_with(".md") || name.ends_with(".yaml") || name.ends_with(".yml");
        if !is_supported {
            continue;
        }
        let Ok(rel) = path.strip_prefix(root) else {
            continue;
        };
        let Ok(content) = fs::read_to_string(&path) else {
            continue;
        };
        let normalized_rel = rel.to_string_lossy().replace('\\', "/");
        out.push((normalized_rel, content));
    }
}

fn parse_design_index_yaml(raw: &str) -> DesignManifest {
    let mut manifest = DesignManifest::default();
    let mut section = "";
    let mut in_files = false;

    for raw_line in raw.lines() {
        let trimmed = raw_line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        if !raw_line.starts_with(' ') && !raw_line.starts_with('\t') {
            in_files = false;
            if trimmed.starts_with("entrypoints:") {
                section = "entrypoints";
            } else if trimmed.starts_with("document_tree:") {
                section = "document_tree";
            } else {
                section = "";
            }
            continue;
        }

        match section {
            "entrypoints" => {
                if let Some(flow) = trimmed
                    .strip_prefix('-')
                    .map(str::trim)
                    .and_then(|s| s.strip_prefix('{'))
                    .and_then(|s| s.strip_suffix('}'))
                {
                    let fields = parse_inline_mapping(flow);
                    if let (Some(title), Some(markdown), Some(yaml)) = (
                        fields.get("title"),
                        fields.get("markdown"),
                        fields.get("yaml"),
                    ) {
                        manifest.entrypoints.push(DesignPairEntry {
                            title: title.clone(),
                            markdown: markdown.clone(),
                            yaml: yaml.clone(),
                        });
                    }
                }
            }
            "document_tree" => {
                if trimmed == "files:" {
                    in_files = true;
                } else if trimmed.starts_with("id:") || trimmed.starts_with("title:") {
                    in_files = false;
                } else if in_files && let Some(item) = trimmed.strip_prefix("- ") {
                    let path = unquote_yaml_scalar(item.trim());
                    if !path.is_empty() && !manifest.tree_order.contains_key(&path) {
                        let next_idx = manifest.tree_order.len();
                        manifest.tree_order.insert(path, next_idx);
                    }
                }
            }
            _ => {}
        }
    }

    manifest
}

fn parse_inline_mapping(flow: &str) -> HashMap<String, String> {
    let mut out = HashMap::new();
    for pair in flow.split(',') {
        let Some((key, value)) = pair.split_once(':') else {
            continue;
        };
        let key = key.trim();
        let value = unquote_yaml_scalar(value.trim());
        if !key.is_empty() && !value.is_empty() {
            out.insert(key.to_string(), value);
        }
    }
    out
}

fn unquote_yaml_scalar(raw: &str) -> String {
    let trimmed = raw.trim();
    if trimmed.len() >= 2
        && ((trimmed.starts_with('\'') && trimmed.ends_with('\''))
            || (trimmed.starts_with('"') && trimmed.ends_with('"')))
    {
        trimmed[1..trimmed.len() - 1].to_string()
    } else {
        trimmed.to_string()
    }
}

fn build_design_document(
    sample_path: &str,
    content: &str,
    manifest: &DesignManifest,
) -> Option<Document> {
    let is_markdown = sample_path.ends_with(".md");
    let is_architecture = sample_path == "dok3search-final-architecture.md";
    let pair = manifest
        .entrypoints
        .iter()
        .find(|entry| entry.markdown == sample_path || entry.yaml == sample_path);

    let first_heading = content
        .lines()
        .next()
        .and_then(|line| line.strip_prefix("# "))
        .map(str::trim_end);

    let markdown = if is_markdown {
        strip_leading_h1(content)
    } else {
        format!("```yaml\n{}\n```", content.trim_end())
    };

    let summary = design_summary(sample_path, &markdown, is_markdown);
    let doc_id = if is_architecture {
        "final-architecture".to_string()
    } else {
        format!("design-{}", slug_non_alnum(sample_path))
    };

    let base_title = pair
        .map(|entry| entry.title.clone())
        .or_else(|| first_heading.map(|h| h.replace(['`', '*'], "")))
        .unwrap_or_else(|| sample_path.to_string());
    let title = if sample_path.ends_with("-download.md") {
        format!("{base_title} (다운로드본)")
    } else {
        base_title
    };

    let tag = if is_architecture {
        "아키텍처"
    } else if is_markdown {
        "Markdown"
    } else {
        "YAML"
    };

    let sections = parse_h2_sections(&markdown);
    let revision_hash = fnv1a_64_hex(content.as_bytes());

    Some(Document {
        id: DocumentId::new(doc_id),
        revision_id: RevisionId::new(format!("fnv1a64:{revision_hash}")),
        source_id: SourceId::new("dok3search-design"),
        source: "dok3search-design".to_string(),
        title,
        summary,
        category: "design".to_string(),
        tag: tag.to_string(),
        tags: vec![tag.to_string()],
        keywords: vec!["dok3search-design".to_string()],
        language: if is_markdown { "markdown" } else { "yaml" }.to_string(),
        version: "v1.3".to_string(),
        date: is_architecture.then(|| "2026-11-20".to_string()),
        source_uri: format!("design://{sample_path}"),
        sample_path: Some(sample_path.to_string()),
        acl_scope: AclScope::Public,
        content_hash: revision_hash,
        body: String::new(),
        markdown: Some(markdown),
        sections,
    })
}

fn strip_leading_h1(content: &str) -> String {
    let mut lines = content.lines();
    let Some(first) = lines.next() else {
        return String::new();
    };
    if !first.starts_with("# ") {
        return content.to_string();
    }
    let rest = lines.collect::<Vec<_>>().join("\n");
    rest.trim_start_matches(['\r', '\n']).to_string()
}

fn design_summary(sample_path: &str, markdown: &str, is_markdown: bool) -> String {
    let fixed = match sample_path {
        "index.md" => Some("설계 개요와 자료를 읽는 순서입니다."),
        "index.yaml" => Some("설계 문서의 경로, 그룹, 진입점을 기록한 파일입니다."),
        "map.md" => {
            Some("화면, 구성요소, 계약, 운영 절차가 어떻게 연결되는지 보여 주는 설계 문서입니다.")
        }
        "map.yaml" => Some("항목 ID, 관계 유형, 근거 문서와 절을 구조화한 설계 맵입니다."),
        "dok3search-final-architecture.md" => {
            Some("색인과 검색, 문서 열람, API 연동, 장애 복구를 다루는 dok3search 최종 설계입니다.")
        }
        _ => None,
    };
    if let Some(summary) = fixed {
        return summary.to_string();
    }

    let first_paragraph = split_paragraphs(markdown)
        .into_iter()
        .find(|part| part.chars().next().is_some_and(char::is_alphanumeric));

    if let Some(paragraph) = first_paragraph {
        let cleaned = paragraph.replace(['`', '*'], "");
        let collapsed: String = cleaned.split_whitespace().collect::<Vec<_>>().join(" ");
        return collapsed.chars().take(180).collect();
    }

    if is_markdown {
        String::new()
    } else {
        "설계 저장소의 YAML 예시입니다.".to_string()
    }
}

fn split_paragraphs(text: &str) -> Vec<String> {
    let mut paragraphs = Vec::new();
    let mut current = Vec::new();
    for line in text.lines() {
        if line.trim().is_empty() {
            if !current.is_empty() {
                paragraphs.push(current.join("\n"));
                current.clear();
            }
        } else {
            current.push(line);
        }
    }
    if !current.is_empty() {
        paragraphs.push(current.join("\n"));
    }
    paragraphs
}

fn slug_non_alnum(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut in_sep = false;
    for ch in input.chars() {
        if ch.is_ascii_alphanumeric() {
            out.push(ch);
            in_sep = false;
        } else if !in_sep {
            out.push('-');
            in_sep = true;
        }
    }
    out
}

/// Matches frontend `documentHeadingId`: `'section-' + label.toLowerCase().replace(/[^\p{L}\p{N}]+/gu, '-').replace(/-$/, '')`.
#[must_use]
pub fn document_heading_id(label: &str) -> String {
    let mut slug = String::new();
    let mut in_sep = false;
    for ch in label.to_lowercase().chars() {
        if ch.is_alphabetic() || ch.is_numeric() {
            slug.push(ch);
            in_sep = false;
        } else if !in_sep {
            slug.push('-');
            in_sep = true;
        }
    }
    let trimmed = slug.strip_suffix('-').unwrap_or(&slug);
    format!("section-{trimmed}")
}

fn parse_h2_sections(markdown: &str) -> Vec<DocumentSection> {
    let lines: Vec<&str> = markdown.lines().collect();
    let mut headings: Vec<(usize, String, String)> = Vec::new();
    let mut seen_ids = HashSet::new();
    let mut in_fence = false;

    for (idx, line) in lines.iter().enumerate() {
        let trimmed = line.trim_start();
        if trimmed.starts_with("```") {
            in_fence = !in_fence;
            continue;
        }
        if in_fence {
            continue;
        }
        if let Some(label) = trimmed.strip_prefix("## ") {
            let label_clean = label.trim().replace(['`', '*'], "");
            let base = document_heading_id(&label_clean);
            let mut id = base.clone();
            let mut suffix = 2usize;
            while !seen_ids.insert(id.clone()) {
                id = format!("{base}-{suffix}");
                suffix += 1;
            }
            headings.push((idx, id, label_clean));
        }
    }

    if headings.is_empty() {
        let total_lines = lines.len().max(1);
        return vec![DocumentSection {
            section_id: SectionId::new("body"),
            heading: "본문".to_string(),
            anchor: "body".to_string(),
            order: 1,
            line_start: 1,
            line_end: total_lines,
            body: markdown.to_string(),
        }];
    }

    let mut sections = Vec::with_capacity(headings.len());
    for (order_idx, (line_idx, id, label)) in headings.iter().enumerate() {
        let next_line_idx = headings
            .get(order_idx + 1)
            .map_or(lines.len(), |(next_idx, _, _)| *next_idx);
        let body = if *line_idx + 1 < next_line_idx {
            lines[*line_idx + 1..next_line_idx]
                .join("\n")
                .trim()
                .to_string()
        } else {
            String::new()
        };
        sections.push(DocumentSection {
            section_id: SectionId::new(id.clone()),
            heading: label.clone(),
            anchor: id.clone(),
            order: order_idx + 1,
            line_start: line_idx + 1,
            line_end: next_line_idx.max(line_idx + 1),
            body,
        });
    }
    sections
}

fn fnv1a_64_hex(bytes: &[u8]) -> String {
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for &byte in bytes {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x0100_0000_01b3);
    }
    format!("{hash:016x}")
}

fn builtin_sample_documents() -> Vec<Document> {
    struct RawSample<'a> {
        id: &'a str,
        title: &'a str,
        summary: &'a str,
        source: &'a str,
        tag: &'a str,
        date: &'a str,
        category: &'a str,
        body: &'a str,
    }

    let items = [
        RawSample {
            id: "index-rollbacks",
            title: "인덱스 세대 전환과 롤백",
            summary: "신규 인덱스를 검증하고 검색 트래픽을 전환하는 운영 절차입니다.",
            source: "운영 가이드",
            tag: "색인",
            date: "2026-11-20",
            category: "manual",
            body: "현재 세대의 검색을 유지한 채 새 인덱스를 준비합니다. 문서 수와 대표 검색어 결과를 검증한 뒤 활성 세대를 전환합니다. 전환 후 이상이 있으면 이전에 검증된 세대를 다시 활성화합니다.",
        },
        RawSample {
            id: "rust-workspace",
            title: "Rust 워크스페이스와 검색 계약",
            summary: "검색 요청, 문서 모델, 결과 스냅샷의 경계와 예시를 확인합니다.",
            source: "개발 문서",
            tag: "Rust",
            date: "2026-11-19",
            category: "guide",
            body: "검색 요청은 질의어, 필터, 페이지 크기를 담습니다. 응답의 각 결과는 문서 ID와 섹션을 포함합니다. 화면과 검색 엔진은 이 계약을 사용해 독립적으로 개발할 수 있습니다.",
        },
        RawSample {
            id: "tantivy-query",
            title: "Tantivy 쿼리와 검색 필드",
            summary: "제목·본문·태그에 검색어를 적용하는 쿼리 구성 예시입니다.",
            source: "검색 엔진 참고 자료",
            tag: "Tantivy",
            date: "2026-11-18",
            category: "code",
            body: "정확한 심볼과 문서 제목을 우선 검색합니다. 일반 텍스트 질의는 제목, 본문, 태그를 대상으로 평가하고 결과에 일치 구간을 표시합니다.",
        },
        RawSample {
            id: "ingestion-policy",
            title: "폐쇄망 자료 반입 기준",
            summary: "문서 팩을 반입하기 전에 확인할 출처, 권한, 무결성 기준입니다.",
            source: "자료 관리 기준",
            tag: "폐쇄망",
            date: "2026-11-17",
            category: "policy",
            body: "반입 목록에 원본 출처와 이용 권한을 기록합니다. 파일 체크섬을 검증하고 승인된 자료만 지정된 내부 저장소에 등록합니다.",
        },
        RawSample {
            id: "worker-guide",
            title: "Rust 색인 워커 실행 가이드",
            summary: "색인 작업을 시작하고 상태를 확인하는 방법입니다.",
            source: "개발 문서",
            tag: "Rust",
            date: "2026-11-16",
            category: "guide",
            body: "작업을 등록한 뒤 상태 조회로 대기, 실행, 완료 여부를 확인합니다.",
        },
        RawSample {
            id: "incremental-index",
            title: "Tantivy 증분 색인 점검",
            summary: "변경된 문서를 색인에 반영하는 운영 점검표입니다.",
            source: "운영 가이드",
            tag: "Tantivy",
            date: "2026-11-15",
            category: "manual",
            body: "변경 목록을 확인하고 작업이 끝나면 대표 문서를 다시 검색합니다.",
        },
        RawSample {
            id: "search-api",
            title: "문서 검색 API 요청 예시",
            summary: "검색어, 카테고리 필터, 페이지 요청 예시입니다.",
            source: "API 계약",
            tag: "Tantivy",
            date: "2026-11-14",
            category: "code",
            body: "페이지 크기와 조회 위치를 전달하고 응답에 포함된 문서 식별자를 사용합니다.",
        },
        RawSample {
            id: "document-visibility",
            title: "문서 가시성과 권한 분류",
            summary: "업무 문서를 등록할 때 접근 범위를 지정하는 기준입니다.",
            source: "자료 관리 기준",
            tag: "색인",
            date: "2026-11-13",
            category: "policy",
            body: "문서를 분류하고 그에 맞는 열람 그룹을 연결합니다.",
        },
        RawSample {
            id: "test-data",
            title: "Rust 테스트용 검색 데이터 만들기",
            summary: "검색 검증에 사용할 문서 구성 예시입니다.",
            source: "개발 문서",
            tag: "Rust",
            date: "2026-11-12",
            category: "guide",
            body: "한글과 영문 검색어로 제목 일치와 본문 일치를 각각 검증합니다.",
        },
        RawSample {
            id: "search-fields",
            title: "Tantivy 검색 필드 설정 예시",
            summary: "제목과 본문에 다른 검색 우선순위를 지정합니다.",
            source: "검색 엔진 참고 자료",
            tag: "Tantivy",
            date: "2026-11-11",
            category: "code",
            body: "동일한 검색어가 여러 필드에 등장할 때 제목 일치에 우선순위를 부여합니다.",
        },
        RawSample {
            id: "search-health",
            title: "Tantivy 검색 서비스 상태 점검",
            summary: "최근 색인과 대표 쿼리로 검색 상태를 확인합니다.",
            source: "운영 가이드",
            tag: "Tantivy",
            date: "2026-11-10",
            category: "manual",
            body: "대표 쿼리를 실행해 결과 수와 문서 ID가 예상과 일치하는지 살핍니다.",
        },
        RawSample {
            id: "index-approval",
            title: "Tantivy 색인 반입 승인 기준",
            summary: "색인용 문서 묶음의 검증과 승인 항목입니다.",
            source: "자료 관리 기준",
            tag: "Tantivy",
            date: "2026-11-09",
            category: "policy",
            body: "출처와 버전을 등록하고 반입 목록의 체크섬을 확인합니다.",
        },
        RawSample {
            id: "source-paths",
            title: "문서 수집 경로 설정",
            summary: "수집할 폴더와 제외 규칙을 지정하는 예시입니다.",
            source: "개발 문서",
            tag: "커넥터",
            date: "2026-11-08",
            category: "guide",
            body: "수집 폴더를 지정하고 임시 파일과 중복 경로를 제외합니다.",
        },
        RawSample {
            id: "offline-verification",
            title: "오프라인 배포 파일 검증",
            summary: "설치 전 배포 파일의 목록과 체크섬을 대조합니다.",
            source: "운영 가이드",
            tag: "배포",
            date: "2026-11-07",
            category: "manual",
            body: "배포 목록에 적힌 체크섬과 받은 파일의 체크섬을 비교합니다.",
        },
        RawSample {
            id: "search-snapshot",
            title: "검색 결과 스냅샷 계약",
            summary: "페이지 이동 시 사용하는 검색 결과 식별자를 설명합니다.",
            source: "API 계약",
            tag: "검색",
            date: "2026-11-06",
            category: "code",
            body: "응답이 반환한 스냅샷 식별자를 다음 페이지 요청에 함께 보냅니다.",
        },
        RawSample {
            id: "retention-policy",
            title: "문서 보존 기간 관리",
            summary: "문서 분류에 따라 보존 기간을 기록하는 방법입니다.",
            source: "자료 관리 기준",
            tag: "보존",
            date: "2026-11-05",
            category: "policy",
            body: "등록 시 문서 종류와 보존 기한을 기록하고 만료 여부를 정기적으로 확인합니다.",
        },
        RawSample {
            id: "korean-analysis",
            title: "한글 검색어 분석 예시",
            summary: "띄어쓰기와 복합어가 검색에 미치는 영향을 살펴봅니다.",
            source: "개발 문서",
            tag: "한국어",
            date: "2026-11-04",
            category: "guide",
            body: "같은 뜻의 띄어쓰기 변형을 비교해 제목과 본문에서 어느 구간이 일치하는지 확인합니다.",
        },
        RawSample {
            id: "retry-index",
            title: "색인 실패 작업 재시도",
            summary: "실패한 색인 작업을 재실행하기 전에 확인할 항목입니다.",
            source: "운영 가이드",
            tag: "색인",
            date: "2026-11-03",
            category: "manual",
            body: "실패한 입력과 오류 사유를 확인한 다음 영향 범위를 좁혀 다시 실행합니다.",
        },
        RawSample {
            id: "section-api",
            title: "문서 섹션 조회 응답 예시",
            summary: "문서 ID와 절 ID로 본문을 읽는 응답 예시입니다.",
            source: "API 계약",
            tag: "문서",
            date: "2026-11-02",
            category: "code",
            body: "응답에는 절 제목과 본문, 원본 버전과 출처가 포함됩니다.",
        },
        RawSample {
            id: "source-registration",
            title: "내부 문서 출처 등록 기준",
            summary: "문서에 출처와 소유 부서를 남기는 기준입니다.",
            source: "자료 관리 기준",
            tag: "출처",
            date: "2026-11-01",
            category: "policy",
            body: "소유 부서와 원본 위치를 기록해 정정이 필요할 때 원문을 찾을 수 있게 합니다.",
        },
    ];

    items
        .into_iter()
        .map(|item| {
            let content_hash = fnv1a_64_hex(item.body.as_bytes());
            Document {
                id: DocumentId::new(item.id),
                revision_id: RevisionId::new(format!("fnv1a64:{content_hash}")),
                source_id: SourceId::new("builtin-samples"),
                source: item.source.to_string(),
                title: item.title.to_string(),
                summary: item.summary.to_string(),
                category: item.category.to_string(),
                tag: item.tag.to_string(),
                tags: vec![item.tag.to_string()],
                keywords: vec![item.tag.to_string()],
                language: "ko".to_string(),
                version: "sample-v1".to_string(),
                date: Some(item.date.to_string()),
                source_uri: format!("sample://{}", item.id),
                sample_path: None,
                acl_scope: AclScope::Public,
                content_hash,
                body: item.body.to_string(),
                markdown: None,
                sections: vec![DocumentSection {
                    section_id: SectionId::new("body"),
                    heading: "본문".to_string(),
                    anchor: "body".to_string(),
                    order: 1,
                    line_start: 1,
                    line_end: 1,
                    body: item.body.to_string(),
                }],
            }
        })
        .collect()
}
