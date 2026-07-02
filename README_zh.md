# C3 Zed 扩展

这是 Zed 的 C3 语言扩展，使用官方
[`c3lang/tree-sitter-c3`](https://github.com/c3lang/tree-sitter-c3) 语法和
[`pherrymason/c3-lsp`](https://github.com/pherrymason/c3-lsp)。

[English README](./README.md)

## 功能

- 语法高亮、代码折叠、缩进、括号匹配和 outline 支持。
- 基于 LSP 的补全、跳转定义/声明、hover/类型显示、签名帮助和诊断。
- 常用声明和控制流的 C3 snippets。
- 可通过 Zed settings 和 `c3fmt` 配置外部格式化。

查找引用取决于上游 `c3-lsp` 是否支持。本扩展不会用纯文本搜索伪造语义引用。

## 语言服务器

扩展会优先使用 `PATH` 中的 `c3lsp`。如果找不到，会为受支持的平台下载官方
`c3-lsp` release。

诊断功能要求 `c3lsp` 能找到 `c3c`，通常通过 `PATH` 或项目内的 `c3lsp.json`
配置。扩展启动 `c3lsp` 时会把诊断 debounce 设置为 250ms，而不是上游默认的
2000ms。如果需要覆盖这个值，可以在启动 Zed 前设置
`C3_ZED_DIAGNOSTICS_DELAY_MS`。

## 格式化

`c3c` 和 `c3lsp` 目前都不提供格式化功能。要在 Zed 中格式化 C3，可以安装或构建
[`c3fmt`](https://github.com/lmichaudel/c3fmt)，然后在 Zed 中配置 external
formatter：

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

Zed 的 external formatter 会从标准输入读取当前 buffer，并从标准输出读取格式化后
的结果。external command formatter 只支持整文件格式化，不支持范围格式化。

## 安装

1. 在 Zed 中按 `Ctrl+Shift+P` 打开命令面板，选择 `extensions`。
2. 搜索 `C3` 并点击 `Install`。

## 手动安装

1. 克隆本仓库：

   ```sh
   git clone https://github.com/un4gt/c3-zed-next
   ```

2. 在 Zed 中按 `Ctrl+Shift+P` 打开命令面板，选择 `extensions`。
3. 点击 `Install Dev Extension`，选择克隆下来的目录。

## 开发

本地检查扩展：

```sh
cargo check --target wasm32-wasip1
cargo fmt --check
```

修改 query、snippets 或 Rust 扩展代码后，在 Zed 中执行 `Reload Dev Extension`。

## 致谢

- Tree-sitter 语法：
  [`c3lang/tree-sitter-c3`](https://github.com/c3lang/tree-sitter-c3)
- 语言服务器：
  [`pherrymason/c3-lsp`](https://github.com/pherrymason/c3-lsp)
- 可选格式化工具：
  [`lmichaudel/c3fmt`](https://github.com/lmichaudel/c3fmt)
