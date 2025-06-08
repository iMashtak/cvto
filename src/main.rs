use std::{fs::File, path::PathBuf};

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
- Java Properties"#, long_about = None)]
struct Cli {
    #[arg(help = "Path to input file")]
    input: PathBuf,
    #[arg(help = "Path to output file")]
    output: PathBuf,

    #[arg(short = 'i', help = "Format of input file", display_order = 0)]
    input_format: Option<Format>,

    #[arg(short = 'o', help = "Format of output file", display_order = 1)]
    output_format: Option<Format>,

    #[arg(
        long,
        value_name = "VALUE",
        help = "Separator to use to determine key and value"
    )]
    java_properties_ser_kv_separator: Option<String>,
}

#[derive(Clone, Debug)]
enum Format {
    Json,
    Yaml,
    Toml,
    JavaProperties,
}

impl ValueEnum for Format {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            Format::Json,
            Format::Yaml,
            Format::Toml,
            Format::JavaProperties,
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

fn main() -> Result<(), anyhow::Error> {
    let cli = Cli::parse();

    let input_format = get_format(&cli.input, cli.input_format)?;
    let output_format = get_format(&cli.output, cli.output_format)?;
    let input = File::open(cli.input.as_path()).map_err(|x| {
        anyhow!(
            "Error while opening file {}: {}",
            cli.input.to_string_lossy(),
            x
        )
    })?;
    let output = File::create(cli.output.as_path()).map_err(|x| {
        anyhow!(
            "Error while creating file {}: {}",
            cli.input.to_string_lossy(),
            x
        )
    })?;

    let mut java_properties_ser_options = options::java_properties::SerOptions::new();
    match output_format {
        Format::Json => {}
        Format::Yaml => {}
        Format::Toml => {}
        Format::JavaProperties => {
            if let Some(x) = cli.java_properties_ser_kv_separator {
                java_properties_ser_options.set(options::java_properties::SerOption::KvSeparator(x));
            }
        }
    }

    match (input_format, output_format) {
        (Format::Json, Format::Json) => just_write(input, output)?,
        (Format::Json, Format::Yaml) => json_to_yaml(input, output)?,
        (Format::Json, Format::Toml) => json_to_toml(input, output)?,
        (Format::Json, Format::JavaProperties) => json_to_properties(input, output, java_properties_ser_options)?,
        (Format::Yaml, Format::Json) => yaml_to_json(input, output)?,
        (Format::Yaml, Format::Yaml) => just_write(input, output)?,
        (Format::Yaml, Format::Toml) => yaml_to_toml(input, output)?,
        (Format::Yaml, Format::JavaProperties) => yaml_to_properties(input, output, java_properties_ser_options)?,
        (Format::Toml, Format::Json) => toml_to_json(input, output)?,
        (Format::Toml, Format::Yaml) => toml_to_yaml(input, output)?,
        (Format::Toml, Format::Toml) => just_write(input, output)?,
        (Format::Toml, Format::JavaProperties) => toml_to_properties(input, output)?,
        (Format::JavaProperties, Format::Json) => properties_to_json(input, output)?,
        (Format::JavaProperties, Format::Yaml) => properties_to_yaml(input, output)?,
        (Format::JavaProperties, Format::Toml) => properties_to_toml(input, output)?,
        (Format::JavaProperties, Format::JavaProperties) => just_write(input, output)?,
    };

    Ok(())
}
