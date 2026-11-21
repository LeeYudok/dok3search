//! Builds an independent local source snapshot before the HTTP listener opens.

use std::{path::Path, sync::Arc};

use dok3_connectors::{LocalFileConnector, SourceConfig, build_document};
use dok3_domain::{DocumentParserPort, GenerationId, SourceId};
use dok3_index::TantivyIndex;
use dok3_parser::MarkdownParser;
use dok3_search::SearchService;
use dok3_storage::FileDocumentStore;
use tempfile::TempDir;

pub fn build(root: &Path) -> Result<(SearchService, TempDir, usize), String> {
    let connector = LocalFileConnector::new(SourceConfig::markdown_defaults(
        SourceId::new("local-markdown"),
        "Local Markdown",
        root,
    ))
    .map_err(|err| err.to_string())?;
    let report = connector.scan().map_err(|err| err.to_string())?;
    let skipped = report.skipped.len();
    let snapshot = tempfile::Builder::new()
        .prefix("dok3-local-index-")
        .tempdir()
        .map_err(|err| format!("create local snapshot directory: {err}"))?;
    let store = Arc::new(
        FileDocumentStore::open(&snapshot.path().join("originals"))
            .map_err(|err| err.to_string())?,
    );
    let index = Arc::new(
        TantivyIndex::create_in_ram(GenerationId::new("local-markdown"))
            .map_err(|err| err.to_string())?,
    );
    let parser = MarkdownParser::new();
    for file in report.files {
        let parsed = parser
            .parse(&file.filename, &file.bytes)
            .map_err(|err| format!("parse {}: {err}", file.relative_path))?;
        let mut document = build_document(connector.config(), &file, &parsed);
        let outcome = store
            .put(document.clone(), &file.bytes)
            .map_err(|err| format!("store {}: {err}", file.relative_path))?;
        document.revision_id = outcome.revision().clone();
        index
            .upsert_document(&document)
            .map_err(|err| format!("index {}: {err}", file.relative_path))?;
    }
    index
        .commit_and_reload(None)
        .map_err(|err| err.to_string())?;
    Ok((SearchService::new(index, store), snapshot, skipped))
}
