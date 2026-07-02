# C3 Zed Extension

C3 language support for Zed, using the official
[`c3lang/tree-sitter-c3`](https://github.com/c3lang/tree-sitter-c3) grammar and
[`pherrymason/c3-lsp`](https://github.com/pherrymason/c3-lsp).

[Chinese README](./README_zh.md)

## Features

- Syntax highlighting, folding, indentation, bracket matching, and outline support.
- LSP-backed completion, go to definition/declaration, hover/type display,
  signature help, and diagnostics.
- C3 snippets for common declarations and control flow.
- Optional external formatting through Zed settings and `c3fmt`.

Find references depends on upstream `c3-lsp` support. This extension does not
fake semantic references with text search.

## Language Server

`c3lsp` is used from `PATH` when available. If it is not found, the extension
downloads the official `c3-lsp` release for supported platforms.

Diagnostics require `c3c` to be available to `c3lsp`, usually through `PATH` or
project `c3lsp.json` configuration. The extension starts `c3lsp` with a 250ms
diagnostics debounce instead of the upstream 2000ms default. Set
`C3_ZED_DIAGNOSTICS_DELAY_MS` before launching Zed to override this value.

## Formatting

Neither `c3c` nor `c3lsp` currently provides formatting. To format C3 in Zed,
install or build [`c3fmt`](https://github.com/lmichaudel/c3fmt), then configure
Zed to use it as an external formatter:

```jsonc
{
  "languages": {
    "C3": {
      "formatter": {
        "external": {
          "command": "c3fmt",
          "arguments": ["--stdin", "--stdout"]
        }
      },
      "format_on_save": "on"
    }
  }
}
```

External formatters receive the buffer on standard input and must write the
formatted result to standard output. Zed external command formatters format the
whole buffer and do not support range formatting.

## Installation

1. Open Zed's command palette with `Ctrl+Shift+P` and select `extensions`.
2. Search for `C3` and click `Install`.

## Manual Installation

1. Clone this repository:

   ```sh
   git clone https://github.com/un4gt/c3-zed-next
   ```

2. Open Zed's command palette with `Ctrl+Shift+P` and select `extensions`.
3. Click `Install Dev Extension` and select the cloned directory.

## Development

Build and check the extension locally:

```sh
cargo check --target wasm32-wasip1
cargo fmt --check
```

In Zed, use `Reload Dev Extension` after changing query files, snippets, or Rust
extension code.

## Credits

- Tree-sitter grammar:
  [`c3lang/tree-sitter-c3`](https://github.com/c3lang/tree-sitter-c3)
- Language server:
  [`pherrymason/c3-lsp`](https://github.com/pherrymason/c3-lsp)
- Formatter option:
  [`lmichaudel/c3fmt`](https://github.com/lmichaudel/c3fmt)
