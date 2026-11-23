//! Machine-readable `/llms.txt` discovery summary for AI coding agents.

/// Renders a concise plain-text guide for agents connecting over REST, MCP, or A2A.
#[must_use]
pub fn render_llms_txt(doc_count: usize, mode: &str) -> String {
    format!(
        "# dok3search\n\n\
Air-gapped document and code search platform.\n\n\
## Runtime\n\
- mode: {mode}\n\
- indexed_documents: {doc_count}\n\n\
## Agent Protocols\n\
- MCP (Streamable HTTP): POST /mcp (tools: kb_search, kb_get_document, kb_get_section, kb_list_categories, kb_list_sources, kb_health)\n\
- MCP (stdio): dok3search --mcp-stdio\n\
- A2A Agent Card: GET /.well-known/agent.json\n\
- A2A JSON-RPC: POST /a2a (methods: tasks/send, tasks/get)\n\n\
## REST API\n\
- POST /api/v1/search (query, filters, page, limit, cursor)\n\
- GET /api/v1/documents/{{id}}\n\
- GET /api/v1/documents/{{id}}/sections/{{section_id}}\n\
- GET /api/v1/corpora\n\
- GET /health/ready\n"
    )
}
