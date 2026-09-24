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
    "lineWidth": 100,
    "indentWidth": 2
  }
}
```

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
