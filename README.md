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
cvto input.json output.yaml
```

`cvto` automatically detects formats based on file extension. If required it may be set explicitly by passing `-i` or `-o` options:

```sh
cvto -i json -o yaml file_with_no_extension output.conf
```

Possible values for `-i` and `-o` are:
- `json`
- `yaml`
- `toml`
- `properties`
- `protobuf`

If you need to configure how to serialize and deserialize input and output data, you can provide additional options which have the following naming: `{format}-{type}-{name}`, where:

- `format` may be:
    - `json`
    - `yaml`
    - `toml`
    - `java-properties`
    - `protobuf`
- `type` may be:
    - `in` which means deserialization (how to parse input file)
    - `out` which means serialization (how to write to output file)
- `name` is actual name of option

### Options Reference

**Java Properties**

- `java-properties-out-kv-separator`: Separator to use to determine key and value

**Protobuf**

- `protobuf-in-input`: Paths to .proto files that will be used as inputs
- `protobuf-in-include`: Paths to directories with .proto files
- `protobuf-in-message`: Name of the target message type
- `protobuf-out-input`: Paths to .proto files that will be used as inputs
- `protobuf-out-include`: Paths to directories with .proto files
- `protobuf-out-message`: Name of the target message type

## Motivation

Key points:

- there was no actual utility for conversing TOML or Java Properties to JSON, YAML and vise versa
- there was to convenient utility to generate payloads for Protocol Buffers transports testing. There are only specialized tools for gRPC exist but Protocol Buffers may be used in many cases other than gRPC

Alternatives:

- [`yj`](https://github.com/sclevine/yj) - provides support for JSON, YAML, TOML and HCL. Not maintained in last years.