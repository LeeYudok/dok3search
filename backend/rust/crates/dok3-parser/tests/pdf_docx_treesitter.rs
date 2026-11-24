use std::io::{Cursor, Write};

use dok3_domain::DocumentParserPort;
use dok3_parser::{CodeSymbolParser, DocxParser, ParserRegistry, PdfParser};
use lopdf::content::{Content, Operation};
use lopdf::{Dictionary, Object, Stream, dictionary};
use zip::ZipWriter;
use zip::write::SimpleFileOptions;

fn build_test_docx(xml: &str) -> Vec<u8> {
    let mut buf = Cursor::new(Vec::new());
    let mut zip = ZipWriter::new(&mut buf);
    zip.start_file("word/document.xml", SimpleFileOptions::default())
        .unwrap();
    zip.write_all(xml.as_bytes()).unwrap();
    zip.finish().unwrap();
    buf.into_inner()
}

fn build_test_pdf(text: &str) -> Vec<u8> {
    let mut doc = lopdf::Document::with_version("1.5");
    let pages_id = doc.new_object_id();
    let font_id = doc.add_object(dictionary! {
        "Type" => "Font",
        "Subtype" => "Type1",
        "BaseFont" => "Helvetica",
    });
    let resources_id = doc.add_object(dictionary! {
        "Font" => dictionary! { "F1" => font_id },
    });
    let content = Content {
        operations: vec![
            Operation::new("BT", vec![]),
            Operation::new("Tf", vec!["F1".into(), 12.into()]),
            Operation::new("Td", vec![50.into(), 700.into()]),
            Operation::new("Tj", vec![Object::string_literal(text)]),
            Operation::new("ET", vec![]),
        ],
    };
    let content_id = doc.add_object(Stream::new(dictionary! {}, content.encode().unwrap()));
    let page_id = doc.add_object(dictionary! {
        "Type" => "Page",
        "Parent" => pages_id,
        "MediaBox" => vec![0.into(), 0.into(), 595.into(), 842.into()],
        "Contents" => content_id,
        "Resources" => resources_id,
    });
    doc.objects.insert(
        pages_id,
        Object::Dictionary(dictionary! {
            "Type" => "Pages",
            "Kids" => vec![page_id.into()],
            "Count" => 1,
        }),
    );
    let catalog_id = doc.add_object(dictionary! {
        "Type" => "Catalog",
        "Pages" => pages_id,
    });
    doc.trailer.set("Root", catalog_id);
    let mut out = Vec::new();
    doc.save_to(&mut out).unwrap();
    out
}

#[test]
fn docx_parser_extracts_headings_paragraphs_and_media_warnings() {
    let xml = r#"<w:document><w:body>
        <w:p><w:pPr><w:pStyle w:val="Heading1"/></w:pPr><w:r><w:t>Architecture</w:t></w:r></w:p>
        <w:p><w:r><w:t>Hello DOCX world</w:t></w:r></w:p>
        <w:p><w:r><w:drawing/></w:r></w:p>
    </w:body></w:document>"#;
    let bytes = build_test_docx(xml);
    let parsed = DocxParser::new().parse("spec.docx", &bytes).unwrap();
    assert_eq!(parsed.title, "Architecture");
    assert!(parsed.normalized_text.contains("Hello DOCX world"));
    assert!(!parsed.parse_warnings.is_empty());

    let via_reg = ParserRegistry::with_defaults()
        .parse_auto(
            "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
            "spec.docx",
            &bytes,
        )
        .unwrap();
    assert_eq!(via_reg.title, "Architecture");
}

#[test]
fn pdf_parser_extracts_page_text_and_records_loss_warning() {
    let bytes = build_test_pdf("Hello PDF Knowledge");
    let parsed = PdfParser::new().parse("manual.pdf", &bytes).unwrap();
    assert_eq!(parsed.title, "manual");
    assert!(parsed.normalized_text.contains("Hello PDF Knowledge"));
    assert!(!parsed.parse_warnings.is_empty());
}

#[test]
fn treesitter_code_parser_handles_multiline_rust_java_and_go() {
    let rust_src = b"pub fn multiline_fn(\n    a: i32,\n    b: i32,\n) -> i32 {\n    a + b\n}\n\npub struct AccountInfo {\n    id: u64,\n}\n";
    let parsed = CodeSymbolParser::new()
        .parse("sample.rs", rust_src)
        .unwrap();
    let headings: Vec<&str> = parsed.sections.iter().map(|s| s.heading.as_str()).collect();
    assert!(headings.contains(&"fn multiline_fn"), "{headings:?}");
    assert!(headings.contains(&"struct AccountInfo"), "{headings:?}");

    let go_src = b"package main\n\nfunc Serve(\n    addr string,\n) error {\n    return nil\n}\n";
    let go_parsed = CodeSymbolParser::new().parse("main.go", go_src).unwrap();
    assert!(
        go_parsed.sections.iter().any(|s| s.heading == "func Serve"),
        "{:?}",
        go_parsed.sections
    );
}
