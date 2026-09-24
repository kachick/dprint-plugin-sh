# How to develop

I maintain several dprint WASM plugins that only delegate formatting to upstream crates.

For changes to the project setup, CI, or codebase structure, I update [kachick/dprint-plugin-typstyle](https://github.com/kachick/dprint-plugin-typstyle) first and then sync them across the other plugins.\
This repository should follow its patterns.

## Notes for maintainers

### Upstream dialect behavior

Formatting is delegated to the upstream crate [`shuck-formatter`](https://crates.io/crates/shuck-formatter) (and `shuck-parser`).

- Upstream only supports shell scripts (Bash, Zsh, Posix, and Mksh). It does not support non-shell files like `Makefile`.
- When dialect is `Auto`, upstream checks the shebang first. If no shebang exists, it infers dialect from the file extension (`.sh` becomes `Posix`).
- In `Posix` mode, Bash features like `[[ ]]` are syntax errors. When writing tests or debugging user issues, keep this behavior in mind.

### Adding new options from upstream

When upstream `shuck-formatter` adds new formatting options:

1. Update `src/configuration.rs`: add the new field to `Configuration`.
2. Update `src/lib.rs`: parse it in `resolve_config` and pass it to `shuck_formatter::ShellFormatOptions` in `format`.
3. Update `crates/schemagen`: update schema assertions in `crates/schemagen/src/main.rs`.
4. Update `README.md`: add the new option to the example JSON and table.
5. Update `tests/`: add or update test cases and run `task test`.
