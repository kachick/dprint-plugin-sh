# dprint-plugin-sh

[dprint](https://dprint.dev/) WASM plugin for Shell Scripts using [shuck-formatter](https://crates.io/crates/shuck-formatter).

## Installation

```bash
dprint add kachick/sh
```

## Configuration

Empty works as default

```json
{
  "sh": {
  }
}
```

Customize if necessary

```json
{
  "sh": {
    "dialect": "auto",
    "indentWidth": 2,
    "useTabs": false,
    "binaryNextLine": false,
    "switchCaseIndent": false,
    "spaceRedirects": false,
    "keepPadding": false,
    "functionNextLine": false,
    "neverSplit": false,
    "simplify": false,
    "minify": false
  }
}
```

### Options

| Option             | Type    | Default  | Description                                                                           |
| ------------------ | ------- | -------- | ------------------------------------------------------------------------------------- |
| `dialect`          | string  | `"auto"` | Shell dialect: `"auto"`, `"bash"`, `"posix"`, `"mksh"`, `"zsh"`.                      |
| `indentWidth`      | number  | `2`      | Number of spaces for indent. Inherits global config if not set.                       |
| `useTabs`          | boolean | `false`  | Indent with tabs. Inherits global config if not set.                                  |
| `binaryNextLine`   | boolean | `false`  | Put binary operators (`&&`, `\|\|`, `\|`) at start of next line instead of end.       |
| `switchCaseIndent` | boolean | `false`  | Indent `case` pattern arms under the `case` statement.                                |
| `spaceRedirects`   | boolean | `false`  | Put a space between redirect operators and target (e.g. `> file` instead of `>file`). |
| `keepPadding`      | boolean | `false`  | Keep column alignment spaces between tokens.                                          |
| `functionNextLine` | boolean | `false`  | Put function opening brace `{` on a new line (Allman style).                          |
| `neverSplit`       | boolean | `false`  | Keep statements on a single line where possible.                                      |
| `simplify`         | boolean | `false`  | Rewrite redundant syntax to simpler forms (e.g. `${foo}` to `$foo`).                  |
| `minify`           | boolean | `false`  | Remove comments and extra whitespace to make code small.                              |

### Dialect Resolution

When `dialect` is `"auto"` (default), this plugin chooses the dialect in this order:

1. **Shebang**: Reads the first line `#!...` (e.g., `#!/usr/bin/env bash` -> Bash, `#!/usr/bin/env zsh` -> Zsh).
2. **File extension**:
   - `.bash`, `.bats` -> Bash
   - `.zsh` -> Zsh
   - `.sh`, `.dash`, `.ksh` -> Posix
   - `.mksh` -> Mksh
3. **Fallback**: Files without an extension (like `.envrc`) default to Bash.

> [!NOTE]
> `.sh` files without a shebang resolve to `posix` by default.
> If your `.sh` file uses Bash syntax (such as `[[ ]]`), parsing will fail in `posix` mode.
> To fix this, set `"dialect": "bash"` in your configuration or add a `#!/usr/bin/env bash` shebang line.

### Supported Files

- File extensions: `.sh`, `.bash`, `.zsh`, `.ksh`, `.mksh`, `.dash`, `.bats`
- File names: `.envrc`

Non-shell files such as `Makefile` are not supported.

## Tips

To format code block in markdown, use `tags` option in official markdown plugin

```bash
dprint add markdown
```

```json
{
  "markdown": {
    "tags": {
      "sh": "sh",
      "bash": "sh",
      "zsh": "sh",
      "shell": "sh",
      "shellscript": "sh",
      "shell-script": "sh",
      "ksh": "sh",
      "dash": "sh"
    }
  }
}
```
