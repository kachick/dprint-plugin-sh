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
- File names: `.envrc`, `.bashrc`, `.bash_profile`, `.bash_aliases`, `.bash_logout`, `.profile`, `.zshrc`, `.zshenv`, `.zprofile`, `.zlogin`, `.zlogout`

> [!WARNING]
> Upstream `shuck-formatter` only infers dialects from shebangs and file extensions, falling back to Bash for extensionless files.
> This plugin bridges this gap by detecting known Zsh dotfiles (like `.zshrc`) and passing `zsh` dialect upstream.
> This workaround is heuristic. If you encounter unexpected formatting behavior, set `"dialect": "zsh"` explicitly.

Non-shell files such as `Makefile` are not supported.

## Tips

To format code blocks in markdown, use the `tags` option in the official markdown plugin:

```bash
dprint add markdown
```

In `tags`:

- **Key**: The code block language tag in Markdown (e.g. `bash`).
- **Value**: The target file extension (without a period) that tells this plugin how to format the code.

For example, `.envrc` is a file name, not an extension. So map `"envrc"` to `"bash"`.

```json
{
  "markdown": {
    "tags": {
      "sh": "sh",
      "bash": "bash",
      "zsh": "zsh",
      "envrc": "bash",
      "shell": "sh",
      "shell-script": "sh"
    }
  }
}
```

This list satisfies linguist [mapping](https://github.com/github-linguist/linguist/blob/90fe0515940a5192980b2545c33ec59ef9fef17b/lib/linguist/languages.yml#L7468-L7476).

## Acknowledgments

- Thanks to [shuck](https://github.com/ewhauser/shuck) for making it easy to use as a dprint WASM plugin.
- Thanks to [shfmt](https://github.com/mvdan/sh) for the reference and the original formatter.
