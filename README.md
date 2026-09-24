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

| Option             | Type    | Default | Description                                                     |
| ------------------ | ------- | ------- | --------------------------------------------------------------- |
| `indentWidth`      | number  | `2`     | Number of spaces for indent. Inherits global config if not set. |
| `useTabs`          | boolean | `false` | Indent with tabs. Inherits global config if not set.            |
| `binaryNextLine`   | boolean | `false` | Put binary operators (like `&&`) at start of next line.         |
| `switchCaseIndent` | boolean | `false` | Indent `case` patterns.                                         |
| `spaceRedirects`   | boolean | `false` | Put a space after redirect operators (like `> file`).           |
| `keepPadding`      | boolean | `false` | Keep column padding.                                            |
| `functionNextLine` | boolean | `false` | Put function `{` on next line.                                  |
| `neverSplit`       | boolean | `false` | Keep code on one line where possible.                           |
| `simplify`         | boolean | `false` | Simplify code before format.                                    |
| `minify`           | boolean | `false` | Make code small.                                                |

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
