use std::collections::HashMap;
use std::io::Write;
use std::process::Command;
use std::{fs, io};

use serde::Deserialize;
use serde_json::Value;

static JSON_SPEC: &str = include_str!("src/specification/spec.json");

fn main() -> io::Result<()> {
    println!("cargo:rerun-if-changed=src/specification/spec.json");
    let raw_spec: RawSpecification =
        serde_json::from_str(JSON_SPEC).expect("unable to parse spec.json");

    let mut spec = fs::File::create("src/specification/spec.rs")?;

    writeln!(spec, "// Generated by build.rs, all edits will be lost")?;
    writeln!(spec)?;

    {
        let mut property_types = phf_codegen::Map::new();
        for (name, raw_rule) in raw_spec.property_types {
            property_types.entry(name, &raw_rule.as_rust_code());
        }
        writeln!(spec, "#[rustfmt::skip]")?;
        writeln!(
            spec,
            "pub const PROPERTY_TYPES: phf::Map<&'static str, super::Rule> = {};",
            property_types.build()
        )?;
    }

    {
        let mut resource_types = phf_codegen::Map::new();
        for (name, raw_rule) in raw_spec.resource_types {
            let properties = {
                let properties = raw_rule.all.get("Properties").unwrap().as_object().unwrap();
                let mut map = phf_codegen::Map::new();
                for (name, value) in properties {
                    let value: PropertyRule = serde_json::from_value(value.clone()).unwrap();
                    map.entry(name, &value.as_rust_code());
                }
                map
            };

            resource_types.entry(name, &format!("{}", properties.build()));
        }
        writeln!(spec, "#[rustfmt::skip]")?;
        writeln!(
            spec,
            "pub const RESOURCE_TYPES: phf::Map<&'static str, phf::Map<&'static str, super::PropertyRule>> = {};",
            resource_types.build()
        )?;
    }

    // Install some TypeScript stuff in the right places for IDE comfort. Silently ignore if failing...
    match Command::new("npm")
        .args(["install", "--no-save", "aws-cdk-lib", "@types/node"])
        .current_dir("tests/end-to-end")
        .status()
    {
        Ok(npm_exit) => {
            if !npm_exit.success() {
                eprintln!("npm install failed with {npm_exit:?}");
            }
        }
        Err(cause) => {
            eprintln!("npm install failed with {cause:?}");
        }
    }

    Ok(())
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct RawSpecification {
    #[serde(with = "::serde_with::rust::maps_first_key_wins")]
    pub property_types: HashMap<String, RawRule>,
    pub resource_types: HashMap<String, RawRule>,
}

#[derive(Debug, Deserialize)]
struct RawRule {
    #[serde(flatten, with = "::serde_with::rust::maps_first_key_wins")]
    all: HashMap<String, Value>,
}

impl RawRule {
    fn as_rust_code(&self) -> String {
        let rule = if let Some(primitive_type) = self.all.get("PrimitiveType") {
            assert!(self.all.get("ItemType").is_none());
            assert!(self.all.get("PrimitiveItemType").is_none());
            assert!(self.all.get("Properties").is_none());

            format!(
                "Primitive(super::CfnType::{})",
                primitive_type.as_str().unwrap()
            )
        } else if let Some(type_name) = self.all.get("Type") {
            let type_name = type_name.as_str().unwrap();
            match type_name {
                "List" => {
                    let item_type = self.all.get("PrimitiveItemType").map_or_else(
                        || {
                            format!(
                                "PropertyType({:?})",
                                self.all.get("ItemType").unwrap().as_str().unwrap()
                            )
                        },
                        |primitive_type| {
                            format!(
                                "Primitive(super::CfnType::{})",
                                primitive_type.as_str().unwrap()
                            )
                        },
                    );
                    format!("List(super::ItemTypeRule::{item_type})")
                }
                "Map" => {
                    let item_type = self.all.get("PrimitiveItemType").map_or_else(
                        || {
                            format!(
                                "PropertyType({:?})",
                                self.all.get("ItemType").unwrap().as_str().unwrap()
                            )
                        },
                        |primitive_type| {
                            format!(
                                "Primitive(super::CfnType::{})",
                                primitive_type.as_str().unwrap()
                            )
                        },
                    );
                    format!("Map(super::ItemTypeRule::{item_type})")
                }
                other => todo!("{other}"),
            }
        } else if let Some(properties) = self.all.get("Properties") {
            let properties = {
                let properties = properties.as_object().unwrap();
                let mut map = phf_codegen::Map::new();
                for (name, value) in properties {
                    let value: PropertyRule = serde_json::from_value(value.clone()).unwrap();
                    map.entry(name, &value.as_rust_code());
                }
                map
            };
            format!("Properties({})", properties.build())
        } else {
            unimplemented!("{self:?}")
        };

        format!("super::Rule::{rule}")
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct PropertyRule {
    required: Option<bool>,
    primitive_type: Option<String>,
    primitive_item_type: Option<String>,
    item_type: Option<String>,
    #[serde(rename = "Type")]
    property_type: Option<String>,
}

impl PropertyRule {
    fn as_rust_code(&self) -> String {
        let required = self
            .required
            .map_or_else(|| "false".to_string(), |v| v.to_string());

        let type_rule = if let Some(primitive_type) = &self.primitive_type {
            assert!(self.primitive_item_type.is_none());
            assert!(self.item_type.is_none());
            assert!(self.property_type.is_none());

            format!("Primitive(super::CfnType::{primitive_type})")
        } else if let Some(property_type) = &self.property_type {
            assert!(self.primitive_type.is_none());

            match property_type.as_str() {
                "List" => {
                    let item_type = self
                        .primitive_item_type
                        .as_ref()
                        .map(|item_type| format!("Primitive(super::CfnType::{item_type})"))
                        .unwrap_or_else(|| {
                            self.item_type
                                .as_ref()
                                .map(|item_type| format!("PropertyType({item_type:?})"))
                                .unwrap()
                        });
                    format!("List(super::ItemTypeRule::{item_type})")
                }
                "Map" => {
                    let item_type = self
                        .primitive_item_type
                        .as_ref()
                        .map(|item_type| format!("Primitive(super::CfnType::{item_type})"))
                        .unwrap_or_else(|| {
                            self.item_type
                                .as_ref()
                                .map(|item_type| format!("PropertyType({item_type:?})"))
                                .unwrap()
                        });
                    format!("Map(super::ItemTypeRule::{item_type})")
                }
                name => {
                    assert!(self.primitive_item_type.is_none());
                    assert!(self.item_type.is_none());

                    format!("PropertyType({name:?})")
                }
            }
        } else {
            unimplemented!("{self:?}")
        };

        format!("super::PropertyRule {{ required: {required}, type_rule: super::TypeRule::{type_rule} }}")
    }
}
