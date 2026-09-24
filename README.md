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

| Option             | Type    | Default  | Description                                                     |
| ------------------ | ------- | -------- | --------------------------------------------------------------- |
| `dialect`          | string  | `"auto"` | Shell dialect: `"auto"`, `"bash"`, `"posix"`, `"mksh"`, `"zsh"` |
| `indentWidth`      | number  | `2`      | Number of spaces for indent. Inherits global config if not set. |
| `useTabs`          | boolean | `false`  | Indent with tabs. Inherits global config if not set.            |
| `binaryNextLine`   | boolean | `false`  | Put binary operators (like `&&`) at start of next line.         |
| `switchCaseIndent` | boolean | `false`  | Indent `case` patterns.                                         |
| `spaceRedirects`   | boolean | `false`  | Put a space after redirect operators (like `> file`).           |
| `keepPadding`      | boolean | `false`  | Keep column padding.                                            |
| `functionNextLine` | boolean | `false`  | Put function `{` on next line.                                  |
| `neverSplit`       | boolean | `false`  | Keep code on one line where possible.                           |
| `simplify`         | boolean | `false`  | Simplify code before format.                                    |
| `minify`           | boolean | `false`  | Make code small.                                                |

### Dialect Resolution

When `dialect` is `"auto"` (default), this plugin chooses the dialect in this order:

1. **Shebang**: Reads the first line `#!...` (e.g., `#!/bin/bash` -> Bash, `#!/usr/bin/env zsh` -> Zsh).
2. **File extension**:
   - `.bash` -> Bash
   - `.zsh` -> Zsh
   - `.sh` -> Posix
   - `.mksh` -> Mksh
3. **Fallback**: Files without an extension (like `.envrc`) default to Bash.

> [!NOTE]
> `.sh` files without a shebang resolve to `posix` by default.
> If your `.sh` file uses Bash syntax (such as `[[ ]]`), parsing will fail in `posix` mode.
> To fix this, set `"dialect": "bash"` in your configuration or add a `#!/bin/bash` shebang line.

### Supported Files

- File extensions: `.sh`, `.bash`, `.zsh`, `.ksh`, `.mksh`
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
      "shellscript": "sh",
      "bash": "sh",
      "zsh": "sh"
    }
  }
}
```
