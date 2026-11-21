use dok3_analyzer::{AnalyzeOptions, analyze_text, generate_char_ngrams, normalize_query_terms};

#[test]
fn korean_and_english_ngrams_stay_inside_words() {
    let analyzed = analyze_text("계좌조회 RGB 한 글", AnalyzeOptions::default());
    assert_eq!(analyzed.terms, ["계좌조회", "rgb", "한", "글"]);
    assert_eq!(
        analyzed.char_ngrams,
        [
            "계좌",
            "계좌조",
            "좌조",
            "좌조회",
            "조회",
            "rg",
            "rgb",
            "gb"
        ]
    );
    assert_eq!(
        generate_char_ngrams("한A-글B\n漢字"),
        ["한a", "글b", "漢字"]
    );
}

#[test]
fn identifier_components_follow_the_whole_term() {
    assert_eq!(
        normalize_query_terms(
            "getAccountList AccountList parseHTTPResponse user_id tokio::spawn X.Y::Z"
        ),
        [
            "getaccountlist",
            "get",
            "account",
            "list",
            "accountlist",
            "parsehttpresponse",
            "parse",
            "http",
            "response",
            "user_id",
            "user",
            "id",
            "tokio",
            "spawn",
            "x",
            "y",
            "z"
        ]
    );
    assert_eq!(
        analyze_text("std/io/src.rs path__segment", AnalyzeOptions::default()).terms,
        ["std", "io", "src", "rs", "path__segment", "path", "segment"]
    );
}

#[test]
fn rust_java_and_sql_codes_and_versions() {
    assert_eq!(
        analyze_text(
            "E0382 java.lang.NullPointerException ORA-00904 SQLSTATE 23505 HTTP/2 v1.2.3",
            AnalyzeOptions::default(),
        )
        .terms,
        [
            "e0382",
            "java",
            "lang",
            "nullpointerexception",
            "null",
            "pointer",
            "exception",
            "ora-00904",
            "ora",
            "00904",
            "sqlstate",
            "23505",
            "http/2",
            "http",
            "2",
            "v1.2.3",
            "v1",
            "2",
            "3"
        ]
    );
}

#[test]
fn empty_and_punctuation_inputs() {
    for input in ["", " \n\t", "_::./—!?🙂"] {
        assert_eq!(
            analyze_text(input, AnalyzeOptions::default()),
            Default::default()
        );
        assert!(normalize_query_terms(input).is_empty());
    }
}

#[test]
fn output_order_is_deterministic_and_queries_deduplicate() {
    let input = "getAccount getAccount 계좌 계좌";
    let options = AnalyzeOptions::default();
    let expected = analyze_text(input, options);
    assert_eq!(
        expected.terms,
        [
            "getaccount",
            "get",
            "account",
            "getaccount",
            "get",
            "account",
            "계좌",
            "계좌"
        ]
    );
    for _ in 0..8 {
        assert_eq!(analyze_text(input, options), expected);
        assert_eq!(
            normalize_query_terms(input),
            ["getaccount", "get", "account", "계좌"]
        );
    }
}

#[test]
fn optional_expansions_can_be_disabled() {
    let analyzed = analyze_text(
        "getHTTPStatus ORA-00904",
        AnalyzeOptions {
            split_identifiers: false,
            char_ngrams: false,
        },
    );
    assert_eq!(analyzed.terms, ["gethttpstatus", "ora-00904"]);
    assert!(analyzed.char_ngrams.is_empty());
}
