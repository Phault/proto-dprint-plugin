# dprint plugin for proto

[dprint](https://dprint.dev) WASM plugin for [proto](https://github.com/moonrepo/proto).

## Installation

This is a community plugin and is thus not built-in to proto. In order to use it, add the following to `.prototools`:

```toml
[plugins]
dprint = "source:https://github.com/Phault/proto-dprint-plugin/releases/latest/download/dprint_plugin.wasm"
```

Or preferably pin a specific version, to avoid nasty surprises if we mess up a release:

```toml
[plugins]
dprint = "source:https://github.com/Phault/proto-dprint-plugin/releases/download/vX.Y.Z/dprint_plugin.wasm"
```

## Usage

```shell
# install latest SDK
proto install dprint

# install a specific version
proto install dprint 0.43.2
```

## Configuration

dprint plugin does not support configuration.

## Hooks

dprint plugin does not support hooks.

## Contributing

Build the plugin:

```shell
cargo build --target wasm32-wasi
```

Test the plugin by running `proto` commands.

```shell
proto install dprint-test
proto list-remote dprint-test
```
