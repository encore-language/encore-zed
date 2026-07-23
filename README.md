# Zed Encore

A [Encore](https://github.com/encore-language/encore) extension for [Zed](https://zed.dev).

## Development

To develop this extension, see the [Developing Extensions](https://zed.dev/docs/extensions/developing-extensions) section of the Zed docs.

## Language Server

The extension starts the Encore language server automatically for `.enq` files.

Install the server binary first:

```sh
cd /path/to/encore/index/lsp
../encore/target/debug/encore install --path . --name encore-lsp --force --profile release
```

By default this copies the executable to:

```text
~/.encore/bin/encore-lsp
```

The extension resolves the server in this order:

1. `ENCORE_LSP_PATH`
2. local development checkout `../encore/index/lsp/target/release/lsp` (then `debug`)
3. `~/.encore/bin/encore-lsp` or `$ENCORE_INSTALL_ROOT/bin/encore-lsp`
4. `encore-lsp` from `PATH`

For local development it also supports:

- `ENCORE_LSP_PATH=/absolute/path/to/encore/index/lsp/target/release/lsp`
- sibling checkout fallback `../encore/index/lsp/target/{release,debug}/lsp` relative to the `encore-zed` repo
