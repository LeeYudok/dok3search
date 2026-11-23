use dok3_domain::DocumentParserPort;
use dok3_parser::{CodeSymbolParser, ParserRegistry, PlainTextParser};

#[test]
fn extracts_rust_java_go_sql_symbols_with_line_ranges() {
    let parser = CodeSymbolParser::new();

    let rust_src = br#"
use std::io;

pub struct AccountService {
    id: u64,
}

pub async fn fetch_balance(id: u64) -> u64 {
    id
}
"#;
    let rust_doc = parser.parse("src/account.rs", rust_src).unwrap();
    assert_eq!(rust_doc.title, "account.rs");
    assert!(
        rust_doc
            .sections
            .iter()
            .any(|s| s.heading == "struct AccountService")
    );
    assert!(
        rust_doc
            .sections
            .iter()
            .any(|s| s.heading == "fn fetch_balance")
    );

    let java_src = br#"
package com.doksam.core;

public class LedgerController {
    public String getBalance(String id) {
        return id;
    }
}
"#;
    let java_doc = parser.parse("LedgerController.java", java_src).unwrap();
    assert!(
        java_doc
            .sections
            .iter()
            .any(|s| s.heading == "class LedgerController")
    );
    assert!(
        java_doc
            .sections
            .iter()
            .any(|s| s.heading == "method getBalance")
    );

    let go_src = br#"
package main

type Server struct {
    Port int
}

func (s *Server) Start() error {
    return nil
}
"#;
    let go_doc = parser.parse("server.go", go_src).unwrap();
    assert!(go_doc.sections.iter().any(|s| s.heading == "type Server"));
    assert!(go_doc.sections.iter().any(|s| s.heading == "func Start"));

    let sql_src = br#"
CREATE TABLE accounts (id BIGINT PRIMARY KEY);
CREATE OR REPLACE VIEW active_accounts AS SELECT id FROM accounts;
"#;
    let sql_doc = parser.parse("schema.sql", sql_src).unwrap();
    assert!(
        sql_doc
            .sections
            .iter()
            .any(|s| s.heading == "TABLE accounts")
    );
    assert!(
        sql_doc
            .sections
            .iter()
            .any(|s| s.heading == "VIEW active_accounts")
    );
}

#[test]
fn records_warning_on_jsp_scriptlet_and_dispatches_via_registry() {
    let reg = ParserRegistry::with_defaults();
    assert!(reg.supports("", "index.jsp"));
    assert!(reg.supports("text/plain", "notes.txt"));
    assert!(!reg.supports("application/octet-stream", "blob.bin"));

    let jsp = b"<%@ page contentType=\"text/html\" %>\n<% out.println(\"hi\"); %>\n";
    let parsed_jsp = reg.parse_auto("", "page.jsp", jsp).unwrap();
    assert!(!parsed_jsp.parse_warnings.is_empty());

    let txt = reg
        .parse_auto("text/plain", "config.yaml", b"port: 8080\n")
        .unwrap();
    assert_eq!(txt.sections.len(), 1);
    assert_eq!(txt.sections[0].body, "port: 8080");

    let plain = PlainTextParser::new();
    assert!(plain.supports("", "sample.json"));
}
