# Zed Encore

A [Encore](https://github.com/encore-language/encore) extension for [Zed](https://zed.dev).

## Development

To develop this extension, see the [Developing Extensions](https://zed.dev/docs/extensions/developing-extensions) section of the Zed docs.

## Language Server

The extension starts the Encore language server automatically for `.enq` files.

Preferred setup:

```sh
encore install --path /path/to/encore/lsp --name encore-lsp --force
```

For a local Python compiler checkout, use `uv run --project /path/to/encore encore-py install --path /path/to/encore/lsp --name encore-lsp --force`.

The extension starts the standalone `encore-lsp` binary. Python CLI `lsp` modes are no longer used.
