# cvto

Utility for converting data between structured formats. Currently supports following formats:

- JSON
- YAML
- TOML
- Java Properties
- Protobuf

## Installation

You can download pre-built binaries from releases page. Download archive and place the content in your `PATH`. 

## Usage

Simple conversion with default options requires just the following call:

```sh
cvto -i input.json -o output.yaml
```

`cvto` automatically detects formats based on file extension. If required it may be set explicitly by passing `--in-format` or `--out-format` options:

```sh
cvto -i file_with_no_extension -o output.conf --in-format json --out-format yaml
```

Possible values for `--in-format` and `--out-format` are:
- `json`
- `yaml`
- `toml`
- `properties`
- `protobuf`

You can use stdin as input and stdout as output by not passing `-i` or (and) `-o` options, but you need then set `--in-format` or (and) `--out-format` options:

```sh
cat input.json | cvto --in-format json --out-format yaml > output.yaml
```

If you need to configure how to serialize and deserialize input and output data, you can provide additional options which have the following naming: `{format}-{type}-{name}`, where:

- `format` may be:
    - `json`
    - `yaml`
    - `toml`
    - `properties`
    - `protobuf`
- `type` may be:
    - `in` which means deserialization (how to parse input)
    - `out` which means serialization (how to write to output)
- `name` is actual name of option

### Options Reference

**Java Properties**

- `properties-out-kv-separator`: Separator to use to determine key and value

**Protobuf**

- `protobuf-in-input`: Paths to .proto files that will be used as inputs
- `protobuf-in-include`: Paths to directories with .proto files
- `protobuf-in-message`: Name of the target message type
- `protobuf-out-input`: Paths to .proto files that will be used as inputs
- `protobuf-out-include`: Paths to directories with .proto files
- `protobuf-out-message`: Name of the target message type

## Alternatives

- [`yj`](https://github.com/sclevine/yj) - provides support for JSON, YAML, TOML and HCL. Not maintained in last years.