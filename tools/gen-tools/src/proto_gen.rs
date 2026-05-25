//! Proto type definition generation from CosmWasm schemas.
//!
//! Converts CosmWasm JSON schema types into .proto definitions,
//! enabling cross-language protobuf serialization of contract types.

use crate::config::GenerationContext;
use crate::resolver::SourceResolver;
use crate::{GenerationResult, Generator};
use std::collections::BTreeMap;
use std::path::Path;

pub struct ProtoGenGenerator;

impl Generator for ProtoGenGenerator {
    fn name(&self) -> &'static str {
        "proto"
    }

    fn enabled_by_default(&self) -> bool {
        true
    }

    fn generate(&self, ctx: &GenerationContext) -> anyhow::Result<GenerationResult> {
        let proto_out = &ctx.proto_out;
        std::fs::create_dir_all(proto_out)?;

        let mut total_files = 0;

        // If we have a proto vendor directory, scan and generate .proto files
        // by converting schema types -> proto message definitions.
        // This fills the gap between CosmWasm schema JSON and protobuf types.

        let (_label, grouped) = SourceResolver::resolve_grouped(ctx);
        for (contract_name, schemas) in &grouped {
            let proto_content = generate_proto_from_schemas(schemas, contract_name)?;
            if !proto_content.is_empty() {
                let proto_path = proto_out.join(format!("{}.proto", contract_name));
                std::fs::write(&proto_path, &proto_content)?;
                total_files += 1;
                log::info!("[proto] Wrote {:?}", proto_path);
            }
        }

        // Also generate a cosmos-sdk style type URL registry
        if total_files > 0 {
            let registry = generate_type_url_registry(proto_out)?;
            let reg_path = proto_out.join("type_url_registry.rs");
            std::fs::write(&reg_path, &registry)?;
            total_files += 1;
        }

        Ok(GenerationResult {
            name: "proto",
            success: true,
            files_generated: total_files,
            output_dir: Some(proto_out.to_string_lossy().to_string()),
            message: Some(format!("{} proto types generated", total_files)),
        })
    }
}

/// Generate proto definitions from JSON schema files.
fn generate_proto_from_schemas(
    schemas: &BTreeMap<String, serde_json::Value>,
    package_name: &str,
) -> anyhow::Result<String> {
    if schemas.is_empty() {
        return Ok(String::new());
    }

    let clean_package = package_name.replace('-', "_").replace('.', "_");
    let mut output = String::new();

    output.push_str("syntax = \"proto3\";\n");
    output.push_str(&format!("package {}.contract.v1;\n\n", clean_package));
    output.push_str("import \"gogoproto/gogo.proto\";\n\n");
    output.push_str("option (gogoproto.goproto_getters_all) = false;\n\n");

    let mut processed = BTreeMap::new();

    for (type_name, schema) in schemas {
        let msg_name = schema
            .get("title")
            .and_then(|v| v.as_str())
            .unwrap_or(type_name);
        let clean_name = msg_name
            .replace(|c: char| !c.is_alphanumeric(), "_")
            .trim_matches('_')
            .to_string();

        if processed.contains_key(&clean_name) {
            continue;
        }

        match schema.get("type").and_then(|v| v.as_str()) {
            Some("object") => {
                output.push_str(&format!("message {} {{\n", clean_name));
                if let Some(props) = schema.get("properties").and_then(|v| v.as_object()) {
                    let required = schema
                        .get("required")
                        .and_then(|v| v.as_array())
                        .map(|arr| {
                            arr.iter()
                                .filter_map(|r| r.as_str())
                                .collect::<Vec<_>>()
                        })
                        .unwrap_or_default();

                    for (idx, (field_name, field_schema)) in props.iter().enumerate() {
                        let field_num = (idx + 1) as u32;
                        let proto_type = json_type_to_proto(field_schema, &schemas);
                        let optional = if !required.contains(&field_name.as_str()) {
                            "optional "
                        } else {
                            ""
                        };
                        output.push_str(&format!(
                            "  {} {} {} = {};\n",
                            optional, proto_type, field_name, field_num
                        ));
                    }
                }
                output.push_str("}\n\n");
                processed.insert(clean_name, true);
            }
            Some("string") if schema.get("enum").is_some() => {
                // String enum -> proto enum
                output.push_str(&format!("enum {} {{\n", clean_name));
                output.push_str("  INVALID = 0;\n");
                if let Some(variants) = schema.get("enum").and_then(|v| v.as_array()) {
                    for (idx, variant) in variants.iter().enumerate() {
                        if let Some(v) = variant.as_str() {
                            let clean_var = v.to_uppercase().replace('-', "_");
                            output.push_str(&format!("  {} = {};\n", clean_var, idx + 1));
                        }
                    }
                }
                output.push_str("}\n\n");
                processed.insert(clean_name, true);
            }
            _ => {}
        }
    }

    Ok(output)
}

fn json_type_to_proto(
    schema: &serde_json::Value,
    all_schemas: &BTreeMap<String, serde_json::Value>,
) -> String {
    // $ref resolution
    if let Some(ref_path) = schema.get("$ref").and_then(|v| v.as_str()) {
        let ref_name = ref_path.rsplit('/').next().unwrap_or(ref_path);
        let clean = ref_name
            .replace(|c: char| !c.is_alphanumeric(), "_")
            .trim_matches('_')
            .to_string();
        return clean;
    }

    // oneOf/anyOf -> google.protobuf.Any
    if let Some(_) = schema.get("oneOf").or_else(|| schema.get("anyOf")) {
        return "google.protobuf.Any".to_string();
    }

    match schema.get("type").and_then(|v| v.as_str()) {
        Some("string") => "string".to_string(),
        Some("integer") => {
            if let Some(_format) = schema.get("format").and_then(|v| v.as_str()) {
                match _format {
                    "uint64" => "uint64".to_string(),
                    "int64" => "int64".to_string(),
                    _ => "uint64".to_string(),
                }
            } else {
                "uint64".to_string()
            }
        }
        Some("number") => "string".to_string(), // Cosmos convention
        Some("boolean") => "bool".to_string(),
        Some("array") => {
            if let Some(items) = schema.get("items") {
                let inner = json_type_to_proto(items, all_schemas);
                format!("repeated {}", inner)
            } else {
                "repeated string".to_string()
            }
        }
        Some("null") => "google.protobuf.NullValue".to_string(),
        _ => "string".to_string(),
    }
}

/// Generate a Rust type URL registry (compatible with terp-rs proto-gen format).
fn generate_type_url_registry(proto_dir: &Path) -> anyhow::Result<String> {
    let mut output = String::new();
    output.push_str("// Auto-generated type URL registry — do not edit.\n");
    output.push_str("// Run `gen-tools` to regenerate.\n\n");

    // Scan for .proto files and extract message names
    let proto_files: Vec<_> = std::fs::read_dir(proto_dir)
        .map(|entries| {
            entries
                .filter_map(|e| e.ok())
                .filter(|e| e.path().extension().map_or(false, |ext| ext == "proto"))
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    for entry in &proto_files {
        let path = entry.path();
        if let Ok(content) = std::fs::read_to_string(&path) {
            let package = content
                .lines()
                .find(|l| l.trim().starts_with("package "))
                .and_then(|l| {
                    l.trim()
                        .strip_prefix("package ")
                        .and_then(|s| s.strip_suffix(';'))
                })
                .unwrap_or("unknown");

            for line in content.lines() {
                if line.trim().starts_with("message ") {
                    let msg_name = line
                        .trim()
                        .strip_prefix("message ")
                        .and_then(|s| s.split('{').next())
                        .map(|s| s.trim())
                        .unwrap_or("");
                    if !msg_name.is_empty() {
                        let type_url = format!("/{}.{}", package, msg_name);
                        output.push_str(&format!(
                            "pub const {}_TYPE_URL: &str = \"{}\";\n",
                            msg_name.to_uppercase(),
                            type_url
                        ));
                    }
                }
            }
        }
    }

    Ok(output)
}