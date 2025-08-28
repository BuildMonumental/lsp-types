use lsp_types::*;
use serde_json;
use std::str::FromStr;

#[test]
fn test_uri_with_brackets_serialization() {
    // Create a URI with brackets like Fresh framework uses: [slug].tsx
    let file_path = "file:///Users/test/project/routes/blog/[slug].tsx";
    let uri = Uri::from_str(file_path).expect("Should parse URI with brackets");

    // Test serialization of TextDocumentIdentifier
    let doc_id = TextDocumentIdentifier::new(uri.clone());
    let serialized = serde_json::to_string(&doc_id).expect("Should serialize");

    // Test that the serialized JSON contains properly encoded brackets
    assert!(
        serialized.contains("%5B"),
        "Serialized JSON should contain encoded opening bracket"
    );
    assert!(
        serialized.contains("%5D"),
        "Serialized JSON should contain encoded closing bracket"
    );
    assert!(
        !serialized.contains("[slug]"),
        "Serialized JSON should not contain unencoded brackets"
    );

    // Test deserialization - should handle encoded URLs correctly
    let deserialized: TextDocumentIdentifier =
        serde_json::from_str(&serialized).expect("Should deserialize");
    // The deserialized URI will have encoded brackets in the underlying URL
    let expected_encoded_uri =
        Uri::from_str("file:///Users/test/project/routes/blog/%5Bslug%5D.tsx")
            .expect("Should parse encoded URI");
    assert_eq!(
        deserialized.uri, expected_encoded_uri,
        "Deserialized URI should have encoded brackets"
    );

    // Test Location with brackets
    let location = Location::new(
        uri.clone(),
        Range::new(Position::new(0, 0), Position::new(0, 10)),
    );
    let location_serialized = serde_json::to_string(&location).expect("Should serialize Location");

    let location_deserialized: Location =
        serde_json::from_str(&location_serialized).expect("Should deserialize Location");
    // Check that Location serialization also contains encoded brackets
    assert!(
        location_serialized.contains("%5B"),
        "Location serialization should contain encoded brackets"
    );
    assert_eq!(
        location.range, location_deserialized.range,
        "Ranges should match"
    );
}

#[test]
fn test_workspace_edit_with_bracket_uris() {
    use std::collections::HashMap;

    // Test WorkspaceEdit which uses the custom url_map serializer
    let file_path = "file:///Users/test/project/routes/blog/[slug].tsx";
    let uri = Uri::from_str(file_path).expect("Should parse URI with brackets");

    let mut changes = HashMap::new();
    changes.insert(
        uri.clone(),
        vec![TextEdit::new(
            Range::new(Position::new(0, 0), Position::new(0, 5)),
            "Hello".to_string(),
        )],
    );

    let workspace_edit = WorkspaceEdit::new(changes);
    let serialized =
        serde_json::to_string(&workspace_edit).expect("Should serialize WorkspaceEdit");

    // Verify encoded brackets in serialized form
    assert!(
        serialized.contains("%5B"),
        "Serialized WorkspaceEdit should contain encoded brackets"
    );
    assert!(
        serialized.contains("%5D"),
        "Serialized WorkspaceEdit should contain encoded brackets"
    );

    // Test deserialization
    let deserialized: WorkspaceEdit =
        serde_json::from_str(&serialized).expect("Should deserialize WorkspaceEdit");

    // Check that the URI with brackets is preserved
    if let Some(changes) = deserialized.changes {
        // After serialization and deserialization, the URI will have encoded brackets
        let expected_encoded_uri =
            Uri::from_str("file:///Users/test/project/routes/blog/%5Bslug%5D.tsx")
                .expect("Should parse encoded URI");
        assert!(
            changes.contains_key(&expected_encoded_uri),
            "Should contain URI with encoded brackets"
        );
        assert_eq!(changes.len(), 1, "Should have exactly one change");
    } else {
        panic!("WorkspaceEdit should have changes");
    }
}

#[test]
fn test_uri_serialization_roundtrip() {
    let test_cases = vec![
        "file:///Users/test/[slug].tsx",
        "file:///Users/test/blog/[id]/[slug].tsx",
        "file:///Users/test/[[...slug]].tsx",
        "file:///Users/test/[category]/[...slug].tsx",
    ];

    for file_path in test_cases {
        let uri = Uri::from_str(file_path).expect(&format!("Should parse URI: {}", file_path));

        // Direct URI serialization
        let uri_serialized = serde_json::to_string(&uri).expect("Should serialize URI");
        let uri_deserialized: Uri =
            serde_json::from_str(&uri_serialized).expect("Should deserialize URI");

        // Test that serialized form contains encoded brackets
        if file_path.contains('[') || file_path.contains(']') {
            assert!(
                uri_serialized.contains("%5B"),
                "Serialized URI should contain encoded opening brackets when original has brackets"
            );
            assert!(
                uri_serialized.contains("%5D"),
                "Serialized URI should contain encoded closing brackets when original has brackets"
            );
        }

        // After round-trip, URIs with brackets will have encoded brackets
        // This is correct behavior for LSP compliance
        if file_path.contains('[') || file_path.contains(']') {
            // The deserialized URI should have encoded brackets
            assert!(
                uri_deserialized.as_str().contains("%5B")
                    && uri_deserialized.as_str().contains("%5D"),
                "Round-trip URI should have encoded brackets"
            );
        } else {
            assert_eq!(
                uri, uri_deserialized,
                "URI without brackets should round-trip exactly"
            );
        }
    }
}
