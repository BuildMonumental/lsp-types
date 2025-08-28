use lsp_types::*;
use serde_json;
use std::str::FromStr;

#[test]
fn test_bracket_percent_encoding_issue() {
    // This demonstrates the actual issue: brackets should be percent-encoded in URIs for LSP
    let file_path_with_brackets = "file:///Users/test/blog/[slug].tsx";

    // What the URI does with bracket encoding
    let uri = Uri::from_str(file_path_with_brackets).expect("Should parse URI");

    // When we serialize this with serde, it should percent-encode brackets
    let doc_id = TextDocumentIdentifier::new(uri.clone());
    let serialized = serde_json::to_string(&doc_id).expect("Should serialize");

    // The issue is that LSP expects percent-encoded brackets
    // [ should become %5B and ] should become %5D
    let expected_encoded = r#"{"uri":"file:///Users/test/blog/%5Bslug%5D.tsx"}"#;

    // Show that we now have correct percent-encoding
    assert_eq!(
        serialized, expected_encoded,
        "Implementation should now percent-encode brackets"
    );
}

#[test]
fn test_uri_behavior_with_brackets() {
    // Test how the Uri handles different ways of creating URIs with brackets

    // Method 1: Parse a string with unencoded brackets
    let uri1 = Uri::from_str("file:///Users/test/[slug].tsx").expect("Should parse");

    // Method 2: Build URI with encoded brackets
    let uri2 = Uri::from_str("file:///Users/test/%5Bslug%5D.tsx").expect("Should parse");

    // Show what happens during serialization
    let doc1 = TextDocumentIdentifier::new(uri1);
    let doc2 = TextDocumentIdentifier::new(uri2.clone());

    let ser1 = serde_json::to_string(&doc1).expect("Should serialize");
    let ser2 = serde_json::to_string(&doc2).expect("Should serialize");

    // Both should serialize with encoded brackets
    assert!(
        ser1.contains("%5B") && ser1.contains("%5D"),
        "Serialized URI should contain encoded brackets"
    );
    assert!(
        ser2.contains("%5B") && ser2.contains("%5D"),
        "Serialized URI should contain encoded brackets"
    );
}
