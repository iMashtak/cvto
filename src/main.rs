use std::{
    fs::File,
    io::{Read, Write},
    path::PathBuf,
};

use anyhow::anyhow;
use clap::{Parser, ValueEnum, builder::PossibleValue};

use transformations::*;

use options::Options as _;

mod options;
mod transformations;

#[derive(Parser, Debug)]
#[command(name = "cvto")]
#[command(version = "0.1.0")]
#[command(about = r#"Converts data between following formats:
- JSON
- YAML
- TOML
- Java Properties
- Protobuf"#, long_about = None)]
struct Cli {
    #[arg(help = "Path to input file")]
    input: Option<PathBuf>,
    #[arg(help = "Path to output file")]
    output: Option<PathBuf>,

    #[arg(short = 'i', help = "Format of input file", display_order = 0)]
    input_format: Option<Format>,

    #[arg(short = 'o', help = "Format of output file", display_order = 1)]
    output_format: Option<Format>,

    #[arg(
        long,
        value_name = "VALUE",
        help = "Separator to use to determine key and value"
    )]
    java_properties_out_kv_separator: Option<String>,

    #[arg(
        long,
        value_name = "FILES",
        help = "Paths to .proto files that will be used as inputs"
    )]
    protobuf_in_input: Option<Vec<String>>,

    #[arg(
        long,
        value_name = "DIRS",
        help = "Paths to directories with .proto files"
    )]
    protobuf_in_include: Option<Vec<String>>,

    #[arg(long, value_name = "VALUE", help = "Name of the target message type")]
    protobuf_in_message: Option<String>,

    #[arg(
        long,
        value_name = "FILES",
        help = "Paths to .proto files that will be used as inputs"
    )]
    protobuf_out_input: Option<Vec<String>>,

    #[arg(
        long,
        value_name = "DIRS",
        help = "Paths to directories with .proto files"
    )]
    protobuf_out_include: Option<Vec<String>>,

    #[arg(long, value_name = "VALUE", help = "Name of the target message type")]
    protobuf_out_message: Option<String>,
}

#[derive(Clone, Debug)]
enum Format {
    Json,
    Yaml,
    Toml,
    JavaProperties,
    Protobuf,
}

impl ValueEnum for Format {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            Format::Json,
            Format::Yaml,
            Format::Toml,
            Format::JavaProperties,
            Format::Protobuf,
        ]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        match self {
            Format::Json => Some(PossibleValue::new("json").help("JSON")),
            Format::Yaml => Some(PossibleValue::new("yaml").help("YAML")),
            Format::Toml => Some(PossibleValue::new("toml").help("TOML")),
            Format::JavaProperties => {
                Some(PossibleValue::new("properties").help("Java Properties"))
            }
            Format::Protobuf => Some(PossibleValue::new("protobuf").help("Protocol Buffers")),
        }
    }
}

fn determine_format(path: &PathBuf) -> Option<Format> {
    let extension = path.extension()?;
    let extension = extension.to_string_lossy().into_owned();
    match extension.as_str() {
        "json" => Some(Format::Json),
        "yaml" => Some(Format::Yaml),
        "yml" => Some(Format::Yaml),
        "toml" => Some(Format::Toml),
        "properties" => Some(Format::JavaProperties),
        "protobuf" => Some(Format::Protobuf),
        _ => None,
    }
}

fn get_format(path: &PathBuf, format: Option<Format>) -> Result<Format, anyhow::Error> {
    if let Some(format) = format {
        Ok(format)
    } else {
        let format = determine_format(path);
        if let Some(format) = format {
            Ok(format)
        } else {
            Err(anyhow!(
                "Format cannot be determined for file: {}",
                path.as_path().to_string_lossy()
            ))
        }
    }
}

fn get_input_with_format() -> Result<(Box<dyn Read>, Format), anyhow::Error> {
    let cli = Cli::parse();
    if let Some(input) = cli.input {
        let input_format = get_format(&input, cli.input_format)?;
        let input = File::open(input.as_path()).map_err(|x| {
            anyhow!(
                "Error while opening file {}: {}",
                input.to_string_lossy(),
                x
            )
        })?;
        Ok((Box::new(input), input_format))
    } else {
        let handle = std::io::stdin().lock();
        if let None = cli.input_format {
            return Err(anyhow!(
                "Format cannot be determined automatically for stdin, provide `-i` option"
            ));
        }
        let input_format = cli.input_format.unwrap();
        Ok((Box::new(handle), input_format))
    }
}

fn get_output_with_format() -> Result<(Box<dyn Write>, Format), anyhow::Error> {
    let cli = Cli::parse();
    if let Some(output) = cli.output {
        let output_format = get_format(&output, cli.input_format)?;
        let output = File::create(output.as_path()).map_err(|x| {
            anyhow!(
                "Error while creating file {}: {}",
                output.to_string_lossy(),
                x
            )
        })?;
        Ok((Box::new(output), output_format))
    } else {
        let handle = std::io::stdout();
        if let None = cli.output_format {
            return Err(anyhow!(
                "Format cannot be determined automatically for stdout, provide `-o` option"
            ));
        }
        let output_format = cli.output_format.unwrap();
        Ok((Box::new(handle), output_format))
    }
}

fn main() -> Result<(), anyhow::Error> {
    let cli = Cli::parse();

    let (input, input_format) = get_input_with_format()?;
    let (output, output_format) = get_output_with_format()?;

    let mut protobuf_in_options = options::protobuf::InOptions::new();
    match input_format {
        Format::Json => {}
        Format::Yaml => {}
        Format::Toml => {}
        Format::JavaProperties => {}
        Format::Protobuf => {
            if let Some(x) = cli.protobuf_in_include {
                protobuf_in_options.set(options::protobuf::InOption::Include(x));
            }
            if let Some(x) = cli.protobuf_in_input {
                protobuf_in_options.set(options::protobuf::InOption::Input(x));
            }
            if let Some(x) = cli.protobuf_in_message {
                protobuf_in_options.set(options::protobuf::InOption::Message(x));
            }
        }
    };

    let mut java_properties_out_options = options::java_properties::OutOptions::new();
    let mut protobuf_out_options = options::protobuf::OutOptions::new();
    match output_format {
        Format::Json => {}
        Format::Yaml => {}
        Format::Toml => {}
        Format::JavaProperties => {
            if let Some(x) = cli.java_properties_out_kv_separator {
                java_properties_out_options
                    .set(options::java_properties::OutOption::KvSeparator(x));
            }
        }
        Format::Protobuf => {
            if let Some(x) = cli.protobuf_out_include {
                protobuf_out_options.set(options::protobuf::OutOption::Include(x));
            }
            if let Some(x) = cli.protobuf_out_input {
                protobuf_out_options.set(options::protobuf::OutOption::Input(x));
            }
            if let Some(x) = cli.protobuf_out_message {
                protobuf_out_options.set(options::protobuf::OutOption::Message(x));
            }
        }
    }

    match (input_format, output_format) {
        (Format::Json, Format::Json) => json_to_json(input, output)?,
        (Format::Json, Format::Yaml) => json_to_yaml(input, output)?,
        (Format::Json, Format::Toml) => json_to_toml(input, output)?,
        (Format::Json, Format::JavaProperties) => {
            json_to_properties(input, output, java_properties_out_options)?
        }
        (Format::Json, Format::Protobuf) => json_to_protobuf(input, output, protobuf_out_options)?,
        (Format::Yaml, Format::Json) => yaml_to_json(input, output)?,
        (Format::Yaml, Format::Yaml) => yaml_to_yaml(input, output)?,
        (Format::Yaml, Format::Toml) => yaml_to_toml(input, output)?,
        (Format::Yaml, Format::JavaProperties) => {
            yaml_to_properties(input, output, java_properties_out_options)?
        }
        (Format::Yaml, Format::Protobuf) => todo!(),
        (Format::Toml, Format::Json) => toml_to_json(input, output)?,
        (Format::Toml, Format::Yaml) => toml_to_yaml(input, output)?,
        (Format::Toml, Format::Toml) => toml_to_toml(input, output)?,
        (Format::Toml, Format::JavaProperties) => toml_to_properties(input, output)?,
        (Format::Toml, Format::Protobuf) => todo!(),
        (Format::JavaProperties, Format::Json) => properties_to_json(input, output)?,
        (Format::JavaProperties, Format::Yaml) => properties_to_yaml(input, output)?,
        (Format::JavaProperties, Format::Toml) => properties_to_toml(input, output)?,
        (Format::JavaProperties, Format::JavaProperties) => {
            properties_to_properties(input, output, java_properties_out_options)?
        }
        (Format::JavaProperties, Format::Protobuf) => todo!(),
        (Format::Protobuf, Format::Json) => protobuf_to_json(input, output, protobuf_in_options)?,
        (Format::Protobuf, Format::Yaml) => todo!(),
        (Format::Protobuf, Format::Toml) => protobuf_to_toml(input, output, protobuf_in_options)?,
        (Format::Protobuf, Format::JavaProperties) => todo!(),
        (Format::Protobuf, Format::Protobuf) => {
            protobuf_to_protobuf(input, output, protobuf_in_options, protobuf_out_options)?
        }
    };

    Ok(())
}
