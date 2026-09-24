fn main() {
    print!("{}", include_str!(concat!(env!("OUT_DIR"), "/schema.json")));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_generate_json_schema() {
        let schema = include_str!(concat!(env!("OUT_DIR"), "/schema.json"));
        assert!(schema.contains(r#""indentWidth":"#));
        assert!(schema.contains(r#""useTabs":"#));
        assert!(schema.contains(r#""binaryNextLine":"#));
        assert!(schema.contains("https://plugins.dprint.dev/kachick/sh/"));
        assert!(schema.contains("/schema.json"));
        assert!(schema.contains(r#""additionalProperties": false"#));
        assert!(!schema.contains(r#""title":"#));
        assert!(!schema.contains(r#""required":"#));

        let schema_value: serde_json::Value = serde_json::from_str(schema).unwrap();
        let validator = jsonschema::validator_for(&schema_value).expect("valid JSON Schema");

        let fixture: serde_json::Value =
            serde_json::from_str(include_str!("../../../tests/default/dprint.json")).unwrap();
        assert!(validator.is_valid(&fixture["sh"]));

        let valid = serde_json::json!({ "indentWidth": 4, "useTabs": true });
        assert!(validator.is_valid(&valid));

        let invalid = serde_json::json!({ "unknownKey": "invalid" });
        assert!(!validator.is_valid(&invalid));
    }
}
