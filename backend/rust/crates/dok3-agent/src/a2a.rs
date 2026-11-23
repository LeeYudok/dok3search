//! Agent-to-Agent (A2A) Agent Card and JSON-RPC 2.0 task adapter.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use dok3_domain::{Principal, SearchRequest};
use dok3_search::SearchService;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::protocol::{
    JSONRPC_INVALID_PARAMS, JSONRPC_INVALID_REQUEST, JSONRPC_METHOD_NOT_FOUND, JSONRPC_PARSE_ERROR,
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AgentCapabilities {
    pub streaming: bool,
    pub push_notifications: bool,
    pub state_transition_history: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AgentSkill {
    pub id: String,
    pub name: String,
    pub description: String,
    pub tags: Vec<String>,
}

/// Public A2A discovery card served at `GET /.well-known/agent.json`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AgentCard {
    pub name: String,
    pub description: String,
    pub version: String,
    pub url: String,
    pub mcp_endpoint: String,
    pub capabilities: AgentCapabilities,
    pub skills: Vec<AgentSkill>,
}

#[must_use]
pub fn default_agent_card() -> AgentCard {
    AgentCard {
        name: "dok3search".to_string(),
        description: "Air-gapped document and code knowledge foundation (REST, MCP, A2A)"
            .to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        url: "/a2a".to_string(),
        mcp_endpoint: "/mcp".to_string(),
        capabilities: AgentCapabilities {
            streaming: false,
            push_notifications: false,
            state_transition_history: true,
        },
        skills: vec![
            AgentSkill {
                id: "kb_search".to_string(),
                name: "Knowledge Base Search".to_string(),
                description: "Search offline docs, code symbols, error codes, and manuals"
                    .to_string(),
                tags: vec!["search".into(), "rust".into(), "go".into(), "korean".into()],
            },
            AgentSkill {
                id: "kb_get_document".to_string(),
                name: "Read Document".to_string(),
                description: "Retrieve full document metadata and content by document_id"
                    .to_string(),
                tags: vec!["document".into()],
            },
            AgentSkill {
                id: "kb_get_section".to_string(),
                name: "Read Section".to_string(),
                description: "Retrieve bounded section text by document_id and section_id"
                    .to_string(),
                tags: vec!["section".into()],
            },
        ],
    }
}

/// Stateful A2A JSON-RPC 2.0 handler (`tasks/send`, `tasks/get`) delegating to [`SearchService`].
#[derive(Clone)]
pub struct A2aServer {
    search: Arc<SearchService>,
    tasks: Arc<Mutex<HashMap<String, Value>>>,
}

impl A2aServer {
    #[must_use]
    pub fn new(search: Arc<SearchService>) -> Self {
        Self {
            search,
            tasks: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    #[must_use]
    pub fn handle_json(&self, raw: &str, principal: &Principal) -> Value {
        let Ok(req) = serde_json::from_str::<Value>(raw) else {
            return rpc_err(Value::Null, JSONRPC_PARSE_ERROR, "invalid JSON payload");
        };
        let id = req.get("id").cloned().unwrap_or(Value::Null);
        if req.get("jsonrpc").and_then(Value::as_str) != Some("2.0") {
            return rpc_err(id, JSONRPC_INVALID_REQUEST, "jsonrpc must be '2.0'");
        }
        let Some(method) = req.get("method").and_then(Value::as_str) else {
            return rpc_err(id, JSONRPC_INVALID_REQUEST, "missing method");
        };
        let params = req.get("params").cloned().unwrap_or_else(|| json!({}));

        match method {
            "tasks/send" => self.handle_tasks_send(id, &params, principal),
            "tasks/get" => self.handle_tasks_get(id, &params),
            other => rpc_err(
                id,
                JSONRPC_METHOD_NOT_FOUND,
                &format!("unknown A2A method '{other}'"),
            ),
        }
    }

    fn handle_tasks_send(&self, id: Value, params: &Value, principal: &Principal) -> Value {
        let Some(task_id) = params.get("id").and_then(Value::as_str) else {
            return rpc_err(id, JSONRPC_INVALID_PARAMS, "params.id is required");
        };
        let query = extract_query_text(params).unwrap_or_default();
        if query.trim().is_empty() {
            return rpc_err(
                id,
                JSONRPC_INVALID_PARAMS,
                "params.message.parts[].text is required",
            );
        }

        let search_req = SearchRequest {
            query: query.clone(),
            limit: 5,
            ..SearchRequest::default()
        };
        match self.search.search(principal, &search_req) {
            Ok(resp) => {
                let task = json!({
                    "id": task_id,
                    "status": { "state": "completed" },
                    "artifacts": [
                        {
                            "name": "search-results",
                            "parts": [
                                {
                                    "type": "data",
                                    "data": {
                                        "query": resp.query,
                                        "total_hits": resp.total_hits,
                                        "hits": resp.hits,
                                    }
                                }
                            ]
                        }
                    ]
                });
                let mut store = self
                    .tasks
                    .lock()
                    .unwrap_or_else(|poisoned| poisoned.into_inner());
                if store.len() >= 256 {
                    store.clear();
                }
                store.insert(task_id.to_string(), task.clone());
                json!({ "jsonrpc": "2.0", "id": id, "result": task })
            }
            Err(err) => rpc_err(id, -32000, &err.to_string()),
        }
    }

    fn handle_tasks_get(&self, id: Value, params: &Value) -> Value {
        let Some(task_id) = params.get("id").and_then(Value::as_str) else {
            return rpc_err(id, JSONRPC_INVALID_PARAMS, "params.id is required");
        };
        let store = self
            .tasks
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        match store.get(task_id) {
            Some(task) => json!({ "jsonrpc": "2.0", "id": id, "result": task }),
            None => rpc_err(id, -32001, &format!("task '{task_id}' not found")),
        }
    }
}

fn extract_query_text(params: &Value) -> Option<String> {
    if let Some(parts) = params
        .get("message")
        .and_then(|m| m.get("parts"))
        .and_then(Value::as_array)
    {
        for part in parts {
            if let Some(text) = part.get("text").and_then(Value::as_str)
                && !text.trim().is_empty()
            {
                return Some(text.trim().to_string());
            }
        }
    }
    params
        .get("query")
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
}

fn rpc_err(id: Value, code: i32, message: &str) -> Value {
    json!({
        "jsonrpc": "2.0",
        "id": id,
        "error": {
            "code": code,
            "message": message,
        }
    })
}
