//! Machine-readable `/llms.txt` discovery summary for AI coding agents.

/// Renders a concise plain-text guide for agents connecting over REST, MCP, or A2A.
#[must_use]
pub fn render_llms_txt(doc_count: usize, mode: &str) -> String {
    format!(
        "# dok3search\n\n\
Air-gapped document and code search platform (offline web_search / read_url replacement).\n\n\
## Runtime\n\
- mode: {mode}\n\
- indexed_documents: {doc_count}\n\n\
## Recommended Agent Retrieval Loop\n\
1. Search with `kb_search` or `kb_search_code` (supports `path:<dir>`, `ext:<ext>`, `sym:<name>` operators).\n\
2. Inspect `hits[].matched_section_id` and `hits[].matched_line_start`.\n\
3. Read only the relevant section via `kb_get_section(document_id, section_id)` (returns `citation.line_start..line_end`), or use `kb_get_document(..., toc_only=true)` before fetching sections.\n\n\
## Agent Protocols\n\
- MCP (Streamable HTTP): POST /mcp (tools: kb_search, kb_search_code, kb_search_api, kb_search_error, kb_get_document, kb_get_section, kb_list_categories)\n\
- MCP (stdio): dok3search --mcp-stdio\n\
- ACP: POST /acp (or dok3search --acp-stdio)\n\
- A2A Agent Card: GET /.well-known/agent.json\n\
- A2A JSON-RPC: POST /a2a (methods: tasks/send, tasks/get)\n\n\
## REST API\n\
- POST /api/v1/search (query, filters, page, limit, include_snippets)\n\
- GET /api/v1/search/recommendations (q, category, limit)\n\
- GET /api/v1/documents/{{id}}\n\
- GET /api/v1/documents/{{id}}/sections/{{section_id}}\n\
- GET /api/v1/corpora\n\
- GET /health/ready\n"
    )
}
