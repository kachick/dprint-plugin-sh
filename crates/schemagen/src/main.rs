fn main() {
    print!("{}", include_str!(concat!(env!("OUT_DIR"), "/schema.json")));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_generate_json_schema() {
        let schema = include_str!(concat!(env!("OUT_DIR"), "/schema.json"));
        assert!(schema.contains(r#""dialect":"#));
        assert!(schema.contains(r#""indentWidth":"#));
        assert!(schema.contains(r#""useTabs":"#));
        assert!(schema.contains(r#""binaryNextLine":"#));
        assert!(schema.contains("https://plugins.dprint.dev/kachick/sh/"));
        assert!(schema.contains("/schema.json"));
        assert!(schema.contains(r#""additionalProperties": false"#));
        assert!(!schema.contains(r#""title":"#));
        assert!(!schema.contains(r#""required":"#));

        let schema_value: serde_json::Value = serde_json::from_str(schema).unwrap();
        assert_eq!(schema_value["properties"]["dialect"]["default"], "auto");
        assert_eq!(schema_value["properties"]["indentWidth"]["default"], 2);
        assert_eq!(schema_value["properties"]["useTabs"]["default"], false);
        assert_eq!(
            schema_value["properties"]["binaryNextLine"]["default"],
            false
        );
        assert_eq!(
            schema_value["properties"]["switchCaseIndent"]["default"],
            false
        );
        assert_eq!(
            schema_value["properties"]["spaceRedirects"]["default"],
            false
        );
        assert_eq!(schema_value["properties"]["keepPadding"]["default"], false);
        assert_eq!(
            schema_value["properties"]["functionNextLine"]["default"],
            false
        );
        assert_eq!(schema_value["properties"]["neverSplit"]["default"], false);
        assert_eq!(schema_value["properties"]["simplify"]["default"], false);
        assert_eq!(schema_value["properties"]["minify"]["default"], false);

        let validator = jsonschema::validator_for(&schema_value).expect("valid JSON Schema");

        let fixture: serde_json::Value =
            serde_json::from_str(include_str!("../../../tests/default/dprint.json")).unwrap();
        assert!(validator.is_valid(&fixture["sh"]));

        let valid = serde_json::json!({ "dialect": "bash", "indentWidth": 4, "useTabs": true });
        assert!(validator.is_valid(&valid));

        let invalid = serde_json::json!({ "unknownKey": "invalid" });
        assert!(!validator.is_valid(&invalid));
    }
}
