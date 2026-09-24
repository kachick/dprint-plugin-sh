use std::fmt;
use std::str::FromStr;

#[cfg(feature = "schema")]
use schemars::{JsonSchema, schema_for};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[serde(rename_all = "lowercase")]
pub enum Dialect {
    #[default]
    Auto,
    Bash,
    Posix,
    Mksh,
    Zsh,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseDialectError(String);

impl fmt::Display for ParseDialectError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Invalid dialect: '{}'. Expected 'auto', 'bash', 'posix', 'mksh', or 'zsh'.",
            self.0
        )
    }
}

impl std::error::Error for ParseDialectError {}

impl FromStr for Dialect {
    type Err = ParseDialectError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "auto" => Ok(Dialect::Auto),
            "bash" => Ok(Dialect::Bash),
            "posix" | "sh" | "dash" | "ksh" => Ok(Dialect::Posix),
            "mksh" => Ok(Dialect::Mksh),
            "zsh" => Ok(Dialect::Zsh),
            _ => Err(ParseDialectError(s.to_string())),
        }
    }
}

impl From<Dialect> for shuck_formatter::ShellDialect {
    fn from(dialect: Dialect) -> Self {
        match dialect {
            Dialect::Auto => shuck_formatter::ShellDialect::Auto,
            Dialect::Bash => shuck_formatter::ShellDialect::Bash,
            Dialect::Posix => shuck_formatter::ShellDialect::Posix,
            Dialect::Mksh => shuck_formatter::ShellDialect::Mksh,
            Dialect::Zsh => shuck_formatter::ShellDialect::Zsh,
        }
    }
}

#[derive(Clone, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
// NOTE:
// dprint is a customizable formatter platform. We should expose options here
// when upstream `shuck-formatter` provides new formatting settings.
pub struct Configuration {
    pub dialect: Dialect,
    pub indent_width: u8,
    pub use_tabs: bool,
    pub binary_next_line: bool,
    pub switch_case_indent: bool,
    pub space_redirects: bool,
    pub keep_padding: bool,
    pub function_next_line: bool,
    pub never_split: bool,
    pub simplify: bool,
    pub minify: bool,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            dialect: Dialect::default(),
            indent_width: 2,
            use_tabs: false,
            binary_next_line: false,
            switch_case_indent: false,
            space_redirects: false,
            keep_padding: false,
            function_next_line: false,
            never_split: false,
            simplify: false,
            minify: false,
        }
    }
}

#[cfg(feature = "schema")]
#[must_use]
pub fn generate_json_schema() -> String {
    let mut schema = serde_json::to_value(schema_for!(Configuration)).unwrap();
    let version = env!("CARGO_PKG_VERSION");
    if let Some(obj) = schema.as_object_mut() {
        obj.remove("title");
        obj.remove("required");
        obj.insert(
            "$id".to_string(),
            serde_json::Value::String(format!(
                "https://plugins.dprint.dev/kachick/sh/{version}/schema.json"
            )),
        );
        obj.insert(
            "additionalProperties".to_string(),
            serde_json::Value::Bool(false),
        );

        if let Some(properties) = obj.get_mut("properties").and_then(|p| p.as_object_mut()) {
            if let Ok(serde_json::Value::Object(defaults)) =
                serde_json::to_value(Configuration::default())
            {
                for (key, default_val) in defaults {
                    if let Some(prop) = properties.get_mut(&key).and_then(|p| p.as_object_mut()) {
                        prop.insert("default".to_string(), default_val);
                    }
                }
            }
        }
    }
    serde_json::to_string_pretty(&schema).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dialect_from_str() {
        assert_eq!("auto".parse::<Dialect>(), Ok(Dialect::Auto));
        assert_eq!("bash".parse::<Dialect>(), Ok(Dialect::Bash));
        assert_eq!("posix".parse::<Dialect>(), Ok(Dialect::Posix));
        assert_eq!("sh".parse::<Dialect>(), Ok(Dialect::Posix));
        assert_eq!("dash".parse::<Dialect>(), Ok(Dialect::Posix));
        assert_eq!("ksh".parse::<Dialect>(), Ok(Dialect::Posix));
        assert_eq!("mksh".parse::<Dialect>(), Ok(Dialect::Mksh));
        assert_eq!("zsh".parse::<Dialect>(), Ok(Dialect::Zsh));
        let err = "unknown".parse::<Dialect>().unwrap_err();
        assert_eq!(
            err.to_string(),
            "Invalid dialect: 'unknown'. Expected 'auto', 'bash', 'posix', 'mksh', or 'zsh'."
        );
    }

    #[test]
    fn test_configuration_default() {
        let default_config = Configuration::default();
        assert_eq!(default_config.dialect, Dialect::Auto);
        assert_eq!(default_config.indent_width, 2);
        assert!(!default_config.use_tabs);
        assert!(!default_config.binary_next_line);
        assert!(!default_config.switch_case_indent);
        assert!(!default_config.space_redirects);
        assert!(!default_config.keep_padding);
        assert!(!default_config.function_next_line);
        assert!(!default_config.never_split);
        assert!(!default_config.simplify);
        assert!(!default_config.minify);
    }
}
