# cvto

Utility for converting text data between structured formats. Currently supports following formats:

- JSON
- YAML
- TOML
- Java Properties

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

If you need to configure how to serialize and deserialize input and output data, you can provide additional options which have the following naming: `{format}-{type}-{name}`, where:

- `format` may be:
    - `json`
    - `yaml`
    - `toml`
    - `java-properties`
- `type` may be:
    - `de` which means deserialization (how to parse input file)
    - `ser` which means serialization (how to write to output file)
- `name` is actual name of option

### Options Reference

- `java-properties-ser-kv-separator`: Separator to use to determine key and value

## Motivation

There was no actual utility for conversing TOML or Java Properties to JSON, YAML and vise versa.

Alternatives:

- [`yj`](https://github.com/sclevine/yj) - provides support for JSON, YAML, TOML and HCL. Not maintained in last years.