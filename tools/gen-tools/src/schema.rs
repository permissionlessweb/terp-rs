//! `cargo schema` runner — invokes CosmWasm schema generation for each contract.

use crate::config::GenerationContext;
use crate::{GenerationResult, Generator};
use std::path::Path;
use std::process::Command;

pub struct SchemaGenerator;

impl Generator for SchemaGenerator {
    fn name(&self) -> &'static str {
        "schema"
    }

    fn enabled_by_default(&self) -> bool {
        true
    }

    fn generate(&self, ctx: &GenerationContext) -> anyhow::Result<GenerationResult> {
        if ctx.skip_schema {
            log::info!("[schema] Skipping (--skip-schema set)");
            return Ok(GenerationResult {
                name: "schema",
                success: true,
                files_generated: 0,
                output_dir: None,
                message: Some("Skipped via --skip-schema".into()),
            });
        }

        let mut total_files = 0;

        if ctx.contracts.is_empty() {
            // No contracts detected — run `cargo schema` at workspace root
            log::info!("[schema] No contracts detected; running `cargo schema` at workspace root");
            total_files += run_cargo_schema(&ctx.workspace_root)?;
        } else {
            for contract in &ctx.contracts {
                if !contract.is_cosmwasm {
                    log::debug!("[schema] Skipping non-cosmwasm crate: {}", contract.name);
                    continue;
                }
                log::info!(
                    "[schema] Running `cargo schema` for contract: {}",
                    contract.name
                );
                match run_cargo_schema(&contract.manifest_path.parent().unwrap()) {
                    Ok(count) => total_files += count,
                    Err(e) => log::warn!(
                        "[schema] `cargo schema` failed for {}: {}",
                        contract.name,
                        e
                    ),
                }
            }
        }

        // Count schema files produced
        let schema_files = count_schema_files(&ctx.workspace_root);

        Ok(GenerationResult {
            name: "schema",
            success: true,
            files_generated: total_files,
            output_dir: Some(
                ctx.workspace_root
                    .join("schema")
                    .to_string_lossy()
                    .to_string(),
            ),
            message: Some(format!("{} schema files found after generation", schema_files)),
        })
    }
}

/// Run `cargo schema` in the given directory.
fn run_cargo_schema(dir: &Path) -> anyhow::Result<usize> {
    let output = Command::new("cargo")
        .args(["schema"])
        .current_dir(dir)
        .output()
        .map_err(|e| anyhow::anyhow!("Failed to run `cargo schema`: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        // `cargo schema` may not exist for non-contract crates — that's OK.
        if stderr.contains("no such command") || stderr.contains("No subcommand found") {
            log::warn!("[schema] `cargo schema` not available (not a CosmWasm contract?)");
            return Ok(0);
        }
        return Err(anyhow::anyhow!(
            "`cargo schema` failed: {}",
            stderr.lines().last().unwrap_or("(unknown)")
        ));
    }

    // Count generated schema JSON files
    let stdout = String::from_utf8_lossy(&output.stdout);
    log::debug!("[schema] stdout:\n{}", stdout);

    // Count schema dirs created
    let schema_dirs = find_schema_dirs(dir);
    let file_count: usize = schema_dirs
        .iter()
        .map(|d| count_json_files(d))
        .sum();

    Ok(file_count)
}

/// Find all `schema/` directories recursively (depth <= 3).
fn find_schema_dirs(root: &Path) -> Vec<std::path::PathBuf> {
    let mut dirs = Vec::new();
    if root.join("schema").exists() {
        dirs.push(root.join("schema"));
    }
    // Also check subdirectories for contracts with their own schema dirs
    if let Ok(entries) = std::fs::read_dir(root) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() && path.file_name().map_or(false, |n| n != "target") {
                let schema_path = path.join("schema");
                if schema_path.exists() {
                    dirs.push(schema_path);
                }
            }
        }
    }
    dirs
}

fn count_json_files(dir: &std::path::Path) -> usize {
    std::fs::read_dir(dir)
        .map(|entries| {
            entries
                .filter_map(|e| e.ok())
                .filter(|e| {
                    e.path().extension().map_or(false, |ext| ext == "json")
                })
                .count()
        })
        .unwrap_or(0)
}

/// Count total schema JSON files across the workspace.
fn count_schema_files(root: &Path) -> usize {
    find_schema_dirs(root)
        .iter()
        .map(|d| count_json_files(d))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_schema_dirs_no_root() {
        let tmp = tempfile::tempdir().unwrap();
        let dirs = find_schema_dirs(tmp.path());
        assert!(dirs.is_empty());
    }

    #[test]
    fn test_find_schema_dirs_with_root() {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::create_dir(tmp.path().join("schema")).unwrap();
        let dirs = find_schema_dirs(tmp.path());
        assert_eq!(dirs.len(), 1);
    }
}