//! Tantivy schema definition, tokenizer registration, and document serialization helpers.

use dok3_core::Dok3Error;
use dok3_domain::Document;
use serde::{Deserialize, Serialize};
use tantivy::schema::{
    Field, IndexRecordOption, STORED, STRING, Schema, TextFieldIndexing, TextOptions, Value,
};
use tantivy::tokenizer::RemoveLongFilter;
use tantivy::{Index, TantivyDocument};

use crate::tokenizer::{GramTokenizer, TextTokenizer};

/// Increment when index field definitions or tokenizers change incompatibly.
pub const SCHEMA_VERSION: u32 = 2;
/// Tokenizer name for general words and code identifier segments.
pub const TOKENIZER_TEXT: &str = "dok3_text_v2";
/// Tokenizer name for 2-3 character n-gram partial matching.
pub const TOKENIZER_NGRAM: &str = "dok3_ngram_v2";

/// Tantivy field handles bound to a schema.
#[derive(Debug, Clone, Copy)]
pub struct IndexFields {
    pub document_id: Field,
    pub revision_id: Field,
    pub source_id: Field,
    pub acl_scope: Field,
    pub category: Field,
    pub language: Field,
    pub tag_exact: Field,
    pub title: Field,
    pub summary: Field,
    pub tags_text: Field,
    pub body: Field,
    pub body_ngram: Field,
    pub hit_payload: Field,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct StoredHitMeta {
    pub source: String,
    pub tag: String,
    pub tags: Vec<String>,
    pub date: Option<String>,
    pub sample_path: Option<String>,
    pub matched_section_id: Option<String>,
    pub snippet_source: String,
}

/// Builds the Tantivy schema and returns typed field handles.
#[must_use]
pub fn build_schema() -> (Schema, IndexFields) {
    let mut builder = Schema::builder();

    let text_indexed = TextOptions::default().set_indexing_options(
        TextFieldIndexing::default()
            .set_tokenizer(TOKENIZER_TEXT)
            .set_index_option(IndexRecordOption::WithFreqsAndPositions),
    );
    let ngram_indexed = TextOptions::default().set_indexing_options(
        TextFieldIndexing::default()
            .set_tokenizer(TOKENIZER_NGRAM)
            .set_index_option(IndexRecordOption::Basic),
    );

    let fields = IndexFields {
        document_id: builder.add_text_field("document_id", STRING | STORED),
        revision_id: builder.add_text_field("revision_id", STRING | STORED),
        source_id: builder.add_text_field("source_id", STRING | STORED),
        acl_scope: builder.add_text_field("acl_scope", STRING | STORED),
        category: builder.add_text_field("category", STRING | STORED),
        language: builder.add_text_field("language", STRING | STORED),
        tag_exact: builder.add_text_field("tag_exact", STRING),
        title: builder.add_text_field("title", text_indexed.clone().set_stored()),
        summary: builder.add_text_field("summary", text_indexed.clone().set_stored()),
        tags_text: builder.add_text_field("tags_text", text_indexed.clone()),
        body: builder.add_text_field("body", text_indexed.set_stored()),
        body_ngram: builder.add_text_field("body_ngram", ngram_indexed),
        hit_payload: builder.add_text_field("hit_payload", STORED),
    };

    (builder.build(), fields)
}

/// Registers the schema's versioned text and n-gram tokenizers.
pub(crate) fn register_tokenizers(index: &Index) {
    let text = tantivy::tokenizer::TextAnalyzer::builder(TextTokenizer)
        .filter(RemoveLongFilter::limit(128))
        .build();
    index.tokenizers().register(TOKENIZER_TEXT, text);
    index.tokenizers().register(TOKENIZER_NGRAM, GramTokenizer);
}

pub(crate) fn to_tantivy_document(
    fields: &IndexFields,
    doc: &Document,
) -> Result<TantivyDocument, Dok3Error> {
    let body_text = effective_body(doc);
    let snippet_source = build_snippet_source(doc, body_text);
    let meta = StoredHitMeta {
        source: doc.source.clone(),
        tag: doc.tag.clone(),
        tags: doc.tags.clone(),
        date: doc.date.clone(),
        sample_path: doc.sample_path.clone(),
        matched_section_id: doc
            .sections
            .first()
            .map(|s| s.section_id.as_str().to_string()),
        snippet_source,
    };
    let payload_json = serde_json::to_string(&meta)
        .map_err(|e| Dok3Error::Index(format!("failed to encode stored hit metadata: {e}")))?;

    let mut tv = TantivyDocument::default();
    tv.add_text(fields.document_id, doc.id.as_str());
    tv.add_text(fields.revision_id, doc.revision_id.as_str());
    tv.add_text(fields.source_id, doc.source_id.as_str());
    tv.add_text(fields.acl_scope, doc.acl_scope.to_string());
    tv.add_text(fields.category, &doc.category);
    tv.add_text(fields.language, doc.language.trim().to_lowercase());
    for tag in &doc.tags {
        let trimmed = tag.trim().to_lowercase();
        if !trimmed.is_empty() {
            tv.add_text(fields.tag_exact, &trimmed);
        }
    }
    tv.add_text(fields.title, &doc.title);
    tv.add_text(fields.summary, &doc.summary);

    let tags_combined = doc
        .tags
        .iter()
        .chain(doc.keywords.iter())
        .map(String::as_str)
        .collect::<Vec<_>>()
        .join(" ");
    if !tags_combined.trim().is_empty() {
        tv.add_text(fields.tags_text, &tags_combined);
    }

    tv.add_text(fields.body, body_text);
    let ngram_input = format!("{} {} {}", doc.title, doc.summary, body_text);
    tv.add_text(fields.body_ngram, &ngram_input);
    tv.add_text(fields.hit_payload, &payload_json);
    Ok(tv)
}

pub(crate) fn read_stored_str(doc: &TantivyDocument, field: Field) -> &str {
    doc.get_first(field)
        .and_then(|v| v.as_str())
        .unwrap_or_default()
}

fn effective_body(doc: &Document) -> &str {
    if doc.body.trim().is_empty() {
        doc.markdown.as_deref().unwrap_or("")
    } else {
        doc.body.as_str()
    }
}

fn build_snippet_source(doc: &Document, body_text: &str) -> String {
    let summary = doc.summary.trim();
    let body = body_text.trim();
    match (summary.is_empty(), body.is_empty()) {
        (true, true) => String::new(),
        (true, false) => body.to_string(),
        (false, true) => summary.to_string(),
        (false, false) => format!("{summary} {body}"),
    }
}
