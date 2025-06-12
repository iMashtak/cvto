use std::io::{Read, Write};

use anyhow::anyhow;
use indexmap::IndexMap;
use protobuf::reflect::{FileDescriptor, MessageDescriptor};

use crate::options;

fn write_properties(
    properties: IndexMap<String, String>,
    output: impl Write,
    separator: Option<&str>,
) -> Result<(), anyhow::Error> {
    let mut writer = java_properties::PropertiesWriter::new(output);
    if let Some(separator) = separator {
        writer.set_kv_separator(separator)?;
    }
    for (key, value) in properties {
        writer.write(key.as_str(), value.as_str())?;
    }
    writer.finish()?;
    Ok(())
}

fn get_protobuf_message_descriptor(
    input: Vec<String>,
    include: Vec<String>,
    message_name: String,
) -> Result<MessageDescriptor, anyhow::Error> {
    let mut parser = protobuf_parse::Parser::new();
    let parser = parser.pure().inputs(input).includes(include);
    let proto = parser.parse_and_typecheck()?;
    let mut message_descriptor = None;
    for file in proto.file_descriptors {
        let descriptor = FileDescriptor::new_dynamic(file, &[])?;
        for message in descriptor.messages() {
            if message.name() == message_name {
                message_descriptor = Some(message);
            }
        }
    }
    if let None = message_descriptor {
        return Err(anyhow!("Defined message is not found in input files"));
    }
    Ok(message_descriptor.unwrap())
}

pub fn just_write(mut input: impl Read, mut output: impl Write) -> Result<(), anyhow::Error> {
    let mut buf = Vec::new();
    input.read_to_end(&mut buf)?;
    output.write_all(&buf)?;
    Ok(())
}

pub fn json_to_yaml(input: impl Read, output: impl Write) -> Result<(), anyhow::Error> {
    let yaml: serde_yaml::Value = serde_json::from_reader(input)?;
    serde_yaml::to_writer(output, &yaml)?;
    Ok(())
}

pub fn json_to_toml(input: impl Read, mut output: impl Write) -> Result<(), anyhow::Error> {
    let toml: toml::Value = serde_json::from_reader(input)?;
    let toml = toml::to_string_pretty(&toml)?;
    output.write_all(toml.as_bytes())?;
    Ok(())
}

pub fn json_to_properties(
    input: impl Read,
    output: impl Write,
    out_options: options::java_properties::OutOptions,
) -> Result<(), anyhow::Error> {
    fn traverse(
        properties: &mut IndexMap<String, String>,
        key: Option<&str>,
        json: &serde_json::Value,
    ) -> Result<(), anyhow::Error> {
        match json {
            serde_json::Value::Null => {}
            serde_json::Value::Bool(x) => {
                if let Some(key) = key {
                    properties.insert(key.to_string(), x.to_string());
                } else {
                    return Err(anyhow!("Expected to have key defined but have raw boolean"));
                }
            }
            serde_json::Value::Number(x) => {
                if let Some(key) = key {
                    properties.insert(key.to_string(), x.to_string());
                } else {
                    return Err(anyhow!("Expected to have key defined but have raw number"));
                }
            }
            serde_json::Value::String(x) => {
                if let Some(key) = key {
                    properties.insert(key.to_string(), x.to_string());
                } else {
                    return Err(anyhow!("Expected to have key defined but have raw string"));
                }
            }
            serde_json::Value::Array(x) => {
                if let Some(key) = key {
                    let mut result: Vec<String> = Vec::new();
                    for json in x {
                        let value = match json {
                            serde_json::Value::Null => {
                                return Err(anyhow!("Array cannot contain null values"));
                            }
                            serde_json::Value::Bool(x) => x.to_string(),
                            serde_json::Value::Number(x) => x.to_string(),
                            serde_json::Value::String(x) => x.to_string(),
                            serde_json::Value::Array(_) => {
                                return Err(anyhow!("Array cannot contain another array"));
                            }
                            serde_json::Value::Object(_) => {
                                return Err(anyhow!("Array cannot contain object"));
                            }
                        };
                        result.push(value);
                    }
                    let result = result.join(",");
                    properties.insert(key.to_string(), result);
                } else {
                    return Err(anyhow!("Expected to have key defined but have raw array"));
                }
            }
            serde_json::Value::Object(x) => {
                for (k, v) in x {
                    let key = if let Some(key) = key {
                        format!("{}.{}", key, k)
                    } else {
                        k.to_string()
                    };
                    traverse(properties, Some(key.as_str()), v)?;
                }
            }
        };
        Ok(())
    }

    let json: serde_json::Value = serde_json::from_reader(input)?;
    let mut properties = IndexMap::new();
    traverse(&mut properties, None, &json)?;
    write_properties(properties, output, out_options.get_kv_separator())
        .map_err(|x| anyhow!("Error while writing Java Properties to file: {}", x))?;
    Ok(())
}

pub fn json_to_protobuf(
    mut input: impl Read,
    mut output: impl Write,
    out_options: options::protobuf::OutOptions,
) -> Result<(), anyhow::Error> {
    if let None = out_options.get_message() {
        return Err(anyhow!(
            "Message is required to be defined for protobuf serialization"
        ));
    }
    let message_name = out_options.get_message().unwrap();
    let message_descriptor = get_protobuf_message_descriptor(
        out_options.get_input(),
        out_options.get_include(),
        message_name,
    )?;
    let mut json = String::new();
    input.read_to_string(&mut json)?;
    let message = protobuf_json_mapping::parse_dyn_from_str(&message_descriptor, &json)?;
    message.write_to_writer_dyn(&mut output)?;
    Ok(())
}

pub fn yaml_to_json(input: impl Read, output: impl Write) -> Result<(), anyhow::Error> {
    let json: serde_json::Value = serde_yaml::from_reader(input)?;
    serde_json::to_writer_pretty(output, &json)?;
    Ok(())
}

pub fn yaml_to_toml(input: impl Read, mut output: impl Write) -> Result<(), anyhow::Error> {
    let toml: toml::Value = serde_yaml::from_reader(input)?;
    let toml = toml::to_string_pretty(&toml)?;
    output.write_all(toml.as_bytes())?;
    Ok(())
}

pub fn yaml_to_properties(
    input: impl Read,
    output: impl Write,
    out_options: options::java_properties::OutOptions,
) -> Result<(), anyhow::Error> {
    fn yaml_to_string(yaml: &serde_yaml::Value) -> Result<String, anyhow::Error> {
        match yaml {
            serde_yaml::Value::Null => Err(anyhow!("Unexpected null value")),
            serde_yaml::Value::Bool(x) => Ok(x.to_string()),
            serde_yaml::Value::Number(x) => Ok(x.to_string()),
            serde_yaml::Value::String(x) => Ok(x.to_string()),
            serde_yaml::Value::Sequence(_) => Err(anyhow!("Unexpected sequence value")),
            serde_yaml::Value::Mapping(_) => Err(anyhow!("Unexpected mapping value")),
            serde_yaml::Value::Tagged(x) => yaml_to_string(&x.value),
        }
    }
    fn traverse(
        properties: &mut IndexMap<String, String>,
        key: Option<&str>,
        yaml: &serde_yaml::Value,
    ) -> Result<(), anyhow::Error> {
        match yaml {
            serde_yaml::Value::Null => {}
            serde_yaml::Value::Bool(x) => {
                if let Some(key) = key {
                    properties.insert(key.to_string(), x.to_string());
                } else {
                    return Err(anyhow!("Expected to have key defined but have raw boolean"));
                }
            }
            serde_yaml::Value::Number(x) => {
                if let Some(key) = key {
                    properties.insert(key.to_string(), x.to_string());
                } else {
                    return Err(anyhow!("Expected to have key defined but have raw number"));
                }
            }
            serde_yaml::Value::String(x) => {
                if let Some(key) = key {
                    properties.insert(key.to_string(), x.to_string());
                } else {
                    return Err(anyhow!("Expected to have key defined but have raw string"));
                }
            }
            serde_yaml::Value::Sequence(x) => {
                if let Some(key) = key {
                    let mut result: Vec<String> = Vec::new();
                    for yaml in x {
                        let value = yaml_to_string(yaml)?;
                        result.push(value);
                    }
                    let result = result.join(",");
                    properties.insert(key.to_string(), result);
                } else {
                    return Err(anyhow!("Expected to have key defined but have raw array"));
                }
            }
            serde_yaml::Value::Mapping(x) => {
                for (k, v) in x {
                    let key = if let Some(key) = key {
                        format!("{}.{}", key, yaml_to_string(k)?)
                    } else {
                        yaml_to_string(k)?
                    };
                    traverse(properties, Some(key.as_str()), v)?;
                }
            }
            serde_yaml::Value::Tagged(x) => {
                traverse(properties, key, &x.value)?;
            }
        }
        Ok(())
    }

    let yaml: serde_yaml::Value = serde_yaml::from_reader(input)?;
    let mut properties = IndexMap::new();
    traverse(&mut properties, None, &yaml)?;
    write_properties(properties, output, out_options.get_kv_separator())
        .map_err(|x| anyhow!("Error while writing Java Properties to file: {}", x))?;
    Ok(())
}

pub fn toml_to_json(mut input: impl Read, output: impl Write) -> Result<(), anyhow::Error> {
    let mut toml = String::new();
    input.read_to_string(&mut toml)?;
    let toml: toml::Value = toml::from_str(toml.as_str())?;
    serde_json::to_writer_pretty(output, &toml)?;
    Ok(())
}

pub fn toml_to_yaml(mut input: impl Read, output: impl Write) -> Result<(), anyhow::Error> {
    let mut toml = String::new();
    input.read_to_string(&mut toml)?;
    let toml: toml::Value = toml::from_str(toml.as_str())?;
    serde_yaml::to_writer(output, &toml)?;
    Ok(())
}

pub fn toml_to_properties(input: impl Read, output: impl Write) -> Result<(), anyhow::Error> {
    todo!()
}

pub fn properties_to_json(input: impl Read, output: impl Write) -> Result<(), anyhow::Error> {
    todo!()
}

pub fn properties_to_yaml(input: impl Read, output: impl Write) -> Result<(), anyhow::Error> {
    todo!()
}

pub fn properties_to_toml(input: impl Read, output: impl Write) -> Result<(), anyhow::Error> {
    todo!()
}

pub fn protobuf_to_toml(
    mut input: impl Read,
    mut output: impl Write,
    in_options: options::protobuf::InOptions,
) -> Result<(), anyhow::Error> {
    if let None = in_options.get_message() {
        return Err(anyhow!(
            "Message is required to be defined for protobuf deserialization"
        ));
    }
    let message_name = in_options.get_message().unwrap();
    let message_descriptor = get_protobuf_message_descriptor(
        in_options.get_input(),
        in_options.get_include(),
        message_name,
    )?;
    let protobuf = message_descriptor.parse_from_reader(&mut input)?;
    let json = protobuf_json_mapping::print_to_string(protobuf.as_ref())?;
    let toml: toml::Value = serde_json::from_str(json.as_str())?;
    let toml = toml::to_string_pretty(&toml)?;
    output.write_all(toml.as_bytes())?;
    Ok(())
}
