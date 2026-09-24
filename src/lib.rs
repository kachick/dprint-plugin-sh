use dprint_core::configuration::{
    ConfigKeyMap, GlobalConfiguration, get_unknown_property_diagnostics, get_value,
};
use dprint_core::plugins::{
    FileMatchingInfo, FormatError, FormatResult, PluginInfo, PluginResolveConfigurationResult,
    SyncFormatRequest, SyncHostFormatRequest, SyncPluginHandler,
};

pub mod configuration;
use configuration::{Configuration, Dialect};

#[derive(Default)]
pub struct ShellPluginHandler;

impl SyncPluginHandler<Configuration> for ShellPluginHandler {
    fn plugin_info(&mut self) -> PluginInfo {
        let version = env!("CARGO_PKG_VERSION").to_string();
        PluginInfo {
            name: env!("CARGO_PKG_NAME").to_string(),
            version: version.clone(),
            config_key: "sh".to_string(),
            help_url: "https://github.com/kachick/dprint-plugin-sh".to_string(),
            config_schema_url: format!(
                "https://plugins.dprint.dev/kachick/sh/{version}/schema.json"
            ),
            update_url: Some("https://plugins.dprint.dev/kachick/sh/latest.json".to_string()),
        }
    }

    fn license_text(&mut self) -> String {
        include_str!("../LICENSE").to_string()
    }

    fn resolve_config(
        &mut self,
        config: ConfigKeyMap,
        global_config: &GlobalConfiguration,
    ) -> PluginResolveConfigurationResult<Configuration> {
        let mut config = config;
        let mut diagnostics = Vec::new();
        let default_config = Configuration::default();

        let dialect = get_value(
            &mut config,
            "dialect",
            default_config.dialect,
            &mut diagnostics,
        );

        let indent_width = get_value(
            &mut config,
            "indentWidth",
            global_config
                .indent_width
                .unwrap_or(default_config.indent_width),
            &mut diagnostics,
        );

        let use_tabs = get_value(
            &mut config,
            "useTabs",
            global_config.use_tabs.unwrap_or(default_config.use_tabs),
            &mut diagnostics,
        );

        let binary_next_line = get_value(
            &mut config,
            "binaryNextLine",
            default_config.binary_next_line,
            &mut diagnostics,
        );

        let switch_case_indent = get_value(
            &mut config,
            "switchCaseIndent",
            default_config.switch_case_indent,
            &mut diagnostics,
        );

        let space_redirects = get_value(
            &mut config,
            "spaceRedirects",
            default_config.space_redirects,
            &mut diagnostics,
        );

        let keep_padding = get_value(
            &mut config,
            "keepPadding",
            default_config.keep_padding,
            &mut diagnostics,
        );

        let function_next_line = get_value(
            &mut config,
            "functionNextLine",
            default_config.function_next_line,
            &mut diagnostics,
        );

        let never_split = get_value(
            &mut config,
            "neverSplit",
            default_config.never_split,
            &mut diagnostics,
        );

        let simplify = get_value(
            &mut config,
            "simplify",
            default_config.simplify,
            &mut diagnostics,
        );

        let minify = get_value(
            &mut config,
            "minify",
            default_config.minify,
            &mut diagnostics,
        );

        diagnostics.extend(get_unknown_property_diagnostics(config));

        PluginResolveConfigurationResult {
            config: Configuration {
                dialect,
                indent_width,
                use_tabs,
                binary_next_line,
                switch_case_indent,
                space_redirects,
                keep_padding,
                function_next_line,
                never_split,
                simplify,
                minify,
            },
            diagnostics,
            file_matching: FileMatchingInfo {
                file_extensions: vec![
                    "sh".to_string(),
                    "bash".to_string(),
                    "zsh".to_string(),
                    "ksh".to_string(),
                    "mksh".to_string(),
                    "dash".to_string(),
                    "bats".to_string(),
                ],
                file_names: vec![
                    ".envrc".to_string(),
                    ".bashrc".to_string(),
                    ".bash_profile".to_string(),
                    ".bash_aliases".to_string(),
                    ".bash_logout".to_string(),
                    ".profile".to_string(),
                    ".zshrc".to_string(),
                    ".zshenv".to_string(),
                    ".zprofile".to_string(),
                    ".zlogin".to_string(),
                    ".zlogout".to_string(),
                ],
            },
        }
    }

    fn format(
        &mut self,
        request: SyncFormatRequest<Configuration>,
        _format_with_host: impl FnMut(SyncHostFormatRequest) -> FormatResult,
    ) -> FormatResult {
        if request.range.is_some() {
            return Ok(None);
        }

        let text = std::str::from_utf8(&request.file_bytes)?;

        let indent_style = if request.config.use_tabs {
            shuck_formatter::IndentStyle::Tab
        } else {
            shuck_formatter::IndentStyle::Space
        };

        let dialect = resolve_dialect(request.config.dialect, text, request.file_path);

        let options = shuck_formatter::ShellFormatOptions::default()
            .with_dialect(dialect)
            .with_indent_style(indent_style)
            .with_indent_width(request.config.indent_width)
            .with_binary_next_line(request.config.binary_next_line)
            .with_switch_case_indent(request.config.switch_case_indent)
            .with_space_redirects(request.config.space_redirects)
            .with_keep_padding(request.config.keep_padding)
            .with_function_next_line(request.config.function_next_line)
            .with_never_split(request.config.never_split)
            .with_simplify(request.config.simplify)
            .with_minify(request.config.minify);

        match shuck_formatter::format_source(text, Some(request.file_path), &options) {
            Ok(shuck_formatter::FormattedSource::Formatted(result)) => {
                Ok(Some(result.into_bytes()))
            }
            Ok(shuck_formatter::FormattedSource::Unchanged) => Ok(None),
            Err(err) => Err(FormatError::new(format!("Formatting failed: {err}"))),
        }
    }

    fn check_config_updates(
        &self,
        _message: dprint_core::plugins::CheckConfigUpdatesMessage,
    ) -> Result<Vec<dprint_core::plugins::ConfigChange>, FormatError> {
        Ok(Vec::new())
    }
}

fn resolve_dialect(
    configured: Dialect,
    source: &str,
    file_path: &std::path::Path,
) -> shuck_formatter::ShellDialect {
    if configured != Dialect::Auto {
        return configured.into();
    }

    // When the first line has a shebang, let upstream determine the dialect.
    if source.starts_with("#!") {
        return shuck_formatter::ShellDialect::Auto;
    }

    // Upstream `shuck-formatter` falls back to Bash for extensionless files.
    // We help detect known zsh dotfiles when no shebang exists.
    if matches!(
        file_path.file_name().and_then(|n| n.to_str()),
        Some(".zshrc" | ".zshenv" | ".zprofile" | ".zlogin" | ".zlogout")
    ) {
        return shuck_formatter::ShellDialect::Zsh;
    }

    shuck_formatter::ShellDialect::Auto
}

#[cfg(target_arch = "wasm32")]
use dprint_core::generate_plugin_code;

// generate_plugin_code! initializes a static variable, so the second argument must be a const expression.
// Default::default() cannot be used here because trait methods cannot be called in statics.
#[cfg(target_arch = "wasm32")]
generate_plugin_code!(ShellPluginHandler, ShellPluginHandler);

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use dprint_core::configuration::ConfigKeyValue;
    use dprint_core::plugins::{FormatConfigId, NullCancellationToken};

    use super::*;

    #[test]
    fn test_resolve_config_defaults() {
        let mut handler = ShellPluginHandler;
        let result = handler.resolve_config(ConfigKeyMap::new(), &GlobalConfiguration::default());
        assert!(result.diagnostics.is_empty());
        assert_eq!(result.config.dialect, Dialect::Auto);
        assert_eq!(result.config.indent_width, 2);
        assert!(!result.config.use_tabs);
        assert_eq!(
            result.file_matching.file_extensions,
            vec!["sh", "bash", "zsh", "ksh", "mksh", "dash", "bats"]
        );
        assert_eq!(
            result.file_matching.file_names,
            vec![
                ".envrc",
                ".bashrc",
                ".bash_profile",
                ".bash_aliases",
                ".bash_logout",
                ".profile",
                ".zshrc",
                ".zshenv",
                ".zprofile",
                ".zlogin",
                ".zlogout",
            ]
        );
    }

    #[test]
    fn test_resolve_config_explicit_dialect() {
        let mut handler = ShellPluginHandler;
        let mut config = ConfigKeyMap::new();
        config.insert(
            "dialect".to_string(),
            ConfigKeyValue::String("bash".to_string()),
        );
        let result = handler.resolve_config(config, &GlobalConfiguration::default());
        assert!(result.diagnostics.is_empty());
        assert_eq!(result.config.dialect, Dialect::Bash);
    }

    #[test]
    fn test_resolve_config_with_global() {
        let mut handler = ShellPluginHandler;
        let global = GlobalConfiguration {
            indent_width: Some(4),
            use_tabs: Some(true),
            ..Default::default()
        };
        let result = handler.resolve_config(ConfigKeyMap::new(), &global);
        assert!(result.diagnostics.is_empty());
        assert_eq!(result.config.indent_width, 4);
        assert!(result.config.use_tabs);
    }

    #[test]
    fn test_resolve_config_unknown_property() {
        let mut handler = ShellPluginHandler;
        let mut config = ConfigKeyMap::new();
        config.insert(
            "unknownProp".to_string(),
            ConfigKeyValue::String("val".to_string()),
        );
        let result = handler.resolve_config(config, &GlobalConfiguration::default());
        assert_eq!(result.diagnostics.len(), 1);
        assert_eq!(result.diagnostics[0].property_name, "unknownProp");
    }

    #[test]
    fn test_format() {
        let mut handler = ShellPluginHandler;
        let resolve_result =
            handler.resolve_config(ConfigKeyMap::new(), &GlobalConfiguration::default());
        let cancellation_token = NullCancellationToken;
        let request = SyncFormatRequest {
            file_path: &PathBuf::from("test.sh"),
            file_bytes: b"echo   hello\n".to_vec(),
            config_id: FormatConfigId::from_raw(1),
            config: &resolve_result.config,
            range: None,
            token: &cancellation_token,
        };
        let formatted = handler.format(request, |_| unreachable!()).unwrap();
        assert!(formatted.is_some());
        let formatted_str = String::from_utf8(formatted.unwrap()).unwrap();
        assert_eq!(formatted_str, "echo hello\n");
    }

    #[test]
    fn test_format_unchanged() {
        let mut handler = ShellPluginHandler;
        let resolve_result =
            handler.resolve_config(ConfigKeyMap::new(), &GlobalConfiguration::default());
        let cancellation_token = NullCancellationToken;
        let request = SyncFormatRequest {
            file_path: &PathBuf::from("test.sh"),
            file_bytes: b"echo hello\n".to_vec(),
            config_id: FormatConfigId::from_raw(1),
            config: &resolve_result.config,
            range: None,
            token: &cancellation_token,
        };
        let formatted = handler.format(request, |_| unreachable!()).unwrap();
        assert_eq!(formatted, None);
    }

    #[test]
    fn test_format_range_returns_none() {
        let mut handler = ShellPluginHandler;
        let resolve_result =
            handler.resolve_config(ConfigKeyMap::new(), &GlobalConfiguration::default());
        let cancellation_token = NullCancellationToken;
        let request = SyncFormatRequest {
            file_path: &PathBuf::from("test.sh"),
            file_bytes: b"echo   hello\n".to_vec(),
            config_id: FormatConfigId::from_raw(1),
            config: &resolve_result.config,
            range: Some(std::ops::Range { start: 0, end: 5 }),
            token: &cancellation_token,
        };
        let formatted = handler.format(request, |_| unreachable!()).unwrap();
        assert_eq!(formatted, None);
    }

    #[test]
    fn test_format_invalid_utf8() {
        let mut handler = ShellPluginHandler;
        let resolve_result =
            handler.resolve_config(ConfigKeyMap::new(), &GlobalConfiguration::default());
        let cancellation_token = NullCancellationToken;
        let request = SyncFormatRequest {
            file_path: &PathBuf::from("test.sh"),
            file_bytes: vec![0xFF, 0xFE, 0xFD],
            config_id: FormatConfigId::from_raw(1),
            config: &resolve_result.config,
            range: None,
            token: &cancellation_token,
        };
        let result = handler.format(request, |_| unreachable!());
        assert!(result.is_err());
    }

    #[test]
    fn test_format_with_indent_options() {
        let mut handler = ShellPluginHandler;
        let mut config = ConfigKeyMap::new();
        config.insert("indentWidth".to_string(), ConfigKeyValue::Number(4));
        let resolve_result = handler.resolve_config(config, &GlobalConfiguration::default());
        let cancellation_token = NullCancellationToken;
        let input = "if true; then\necho foo\nfi\n";
        let request = SyncFormatRequest {
            file_path: &PathBuf::from("test.sh"),
            file_bytes: input.as_bytes().to_vec(),
            config_id: FormatConfigId::from_raw(1),
            config: &resolve_result.config,
            range: None,
            token: &cancellation_token,
        };
        let formatted = handler.format(request, |_| unreachable!()).unwrap();
        assert!(formatted.is_some());
        let formatted_str = String::from_utf8(formatted.unwrap()).unwrap();
        assert_eq!(formatted_str, "if true; then\n    echo foo\nfi\n");
    }

    #[test]
    fn test_format_sh_file_with_bash_syntax_fails_in_auto_posix_mode() {
        let mut handler = ShellPluginHandler;
        let resolve_result =
            handler.resolve_config(ConfigKeyMap::new(), &GlobalConfiguration::default());
        let cancellation_token = NullCancellationToken;
        // In default Auto mode, .sh without a shebang is parsed as Posix.
        // `[[ ]]` is not supported in Posix dialect, so this should fail.
        let input = "if [[ 1 -eq 1 ]]; then\necho foo\nfi\n";
        let request = SyncFormatRequest {
            file_path: &PathBuf::from("test.sh"),
            file_bytes: input.as_bytes().to_vec(),
            config_id: FormatConfigId::from_raw(1),
            config: &resolve_result.config,
            range: None,
            token: &cancellation_token,
        };
        let result = handler.format(request, |_| unreachable!());
        assert!(result.is_err());
    }

    #[test]
    fn test_format_sh_file_with_explicit_bash_dialect_succeeds() {
        let mut handler = ShellPluginHandler;
        let mut config = ConfigKeyMap::new();
        config.insert(
            "dialect".to_string(),
            ConfigKeyValue::String("bash".to_string()),
        );
        let resolve_result = handler.resolve_config(config, &GlobalConfiguration::default());
        let cancellation_token = NullCancellationToken;
        let input = "if [[ 1 -eq 1 ]]; then\necho foo\nfi\n";
        let request = SyncFormatRequest {
            file_path: &PathBuf::from("test.sh"),
            file_bytes: input.as_bytes().to_vec(),
            config_id: FormatConfigId::from_raw(1),
            config: &resolve_result.config,
            range: None,
            token: &cancellation_token,
        };
        let formatted = handler.format(request, |_| unreachable!()).unwrap();
        assert!(formatted.is_some());
        let formatted_str = String::from_utf8(formatted.unwrap()).unwrap();
        assert_eq!(formatted_str, "if [[ 1 -eq 1 ]]; then\n  echo foo\nfi\n");
    }

    #[test]
    fn test_format_envrc_file() {
        let mut handler = ShellPluginHandler;
        let resolve_result =
            handler.resolve_config(ConfigKeyMap::new(), &GlobalConfiguration::default());
        let cancellation_token = NullCancellationToken;
        let input = "if [[ -f .env ]]; then\nexport   FOO=bar\nfi\n";
        let request = SyncFormatRequest {
            file_path: &PathBuf::from(".envrc"),
            file_bytes: input.as_bytes().to_vec(),
            config_id: FormatConfigId::from_raw(1),
            config: &resolve_result.config,
            range: None,
            token: &cancellation_token,
        };
        let formatted = handler.format(request, |_| unreachable!()).unwrap();
        assert!(formatted.is_some());
        let formatted_str = String::from_utf8(formatted.unwrap()).unwrap();
        assert_eq!(
            formatted_str,
            "if [[ -f .env ]]; then\n  export FOO=bar\nfi\n"
        );
    }

    #[test]
    fn test_format_sh_file_with_bash_shebang_succeeds_in_auto_mode() {
        let mut handler = ShellPluginHandler;
        let resolve_result =
            handler.resolve_config(ConfigKeyMap::new(), &GlobalConfiguration::default());
        let cancellation_token = NullCancellationToken;
        let input = "#!/usr/bin/env bash\nif [[ 1 -eq 1 ]]; then\necho foo\nfi\n";
        let request = SyncFormatRequest {
            file_path: &PathBuf::from("test.sh"),
            file_bytes: input.as_bytes().to_vec(),
            config_id: FormatConfigId::from_raw(1),
            config: &resolve_result.config,
            range: None,
            token: &cancellation_token,
        };
        let formatted = handler.format(request, |_| unreachable!()).unwrap();
        assert!(formatted.is_some());
        let formatted_str = String::from_utf8(formatted.unwrap()).unwrap();
        assert_eq!(
            formatted_str,
            "#!/usr/bin/env bash\nif [[ 1 -eq 1 ]]; then\n  echo foo\nfi\n"
        );
    }

    #[test]
    fn test_format_zshrc_without_shebang() {
        let mut handler = ShellPluginHandler;
        let resolve_result =
            handler.resolve_config(ConfigKeyMap::new(), &GlobalConfiguration::default());
        let cancellation_token = NullCancellationToken;
        let input = "repeat 2 {\nprint hi\n}\n";
        let request = SyncFormatRequest {
            file_path: &PathBuf::from(".zshrc"),
            file_bytes: input.as_bytes().to_vec(),
            config_id: FormatConfigId::from_raw(1),
            config: &resolve_result.config,
            range: None,
            token: &cancellation_token,
        };
        let formatted = handler.format(request, |_| unreachable!()).unwrap();
        assert!(formatted.is_some());
        let formatted_str = String::from_utf8(formatted.unwrap()).unwrap();
        assert_eq!(formatted_str, "repeat 2 {\n  print hi\n}\n");
    }

    #[test]
    fn test_format_bashrc_without_shebang() {
        let mut handler = ShellPluginHandler;
        let resolve_result =
            handler.resolve_config(ConfigKeyMap::new(), &GlobalConfiguration::default());
        let cancellation_token = NullCancellationToken;
        let input = "array=( \"one\" \"two\" )\n";
        let request = SyncFormatRequest {
            file_path: &PathBuf::from(".bashrc"),
            file_bytes: input.as_bytes().to_vec(),
            config_id: FormatConfigId::from_raw(1),
            config: &resolve_result.config,
            range: None,
            token: &cancellation_token,
        };
        let formatted = handler.format(request, |_| unreachable!()).unwrap();
        assert!(formatted.is_some());
        let formatted_str = String::from_utf8(formatted.unwrap()).unwrap();
        assert_eq!(formatted_str, "array=(\"one\" \"two\")\n");
    }
}
