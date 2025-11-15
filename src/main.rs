use std::{env, fs::File, path::PathBuf};

use anyhow::{Context, Result};
use serde::Deserialize;

const DEFAULT_INPUT: &str = "data/overrides.yaml";
const DEFAULT_OUTPUT: &str = "docs/whitelist.txt";

fn main() -> Result<()> {
    let (input_path, output_path) = parse_args();

    let overrides: Overrides =
        serde_yaml::from_reader(File::open(&input_path).with_context(|| {
            format!("Unable to open YAML input at {}", input_path.display())
        })?)
        .with_context(|| format!("Unable to parse YAML at {}", input_path.display()))?;

    if let Some(parent) = output_path.parent() {
        std::fs::create_dir_all(parent).with_context(|| {
            format!(
                "Unable to create output directory {}",
                parent.display()
            )
        })?;
    }

    let output_file = File::create(&output_path).with_context(|| {
        format!("Unable to create output file {}", output_path.display())
    })?;

    let mut writer = csv::WriterBuilder::new()
        .has_headers(true)
        .from_writer(output_file);

    writer.write_record([
        "ModId",
        "Enable Dynamic Download",
        "Allow non-dataonly blueprints",
    ])?;

    for entry in overrides.entries {
        writer.write_record([
            entry.mod_id.as_str(),
            flag(entry.enable_dynamic_download),
            flag(entry.allow_non_dataonly_blueprints),
        ])?;
    }

    writer.flush()?;

    println!(
        "Generated {} from {}",
        output_path.display(),
        input_path.display()
    );

    Ok(())
}

fn parse_args() -> (PathBuf, PathBuf) {
    let mut args = env::args().skip(1);
    let input = args
        .next()
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(DEFAULT_INPUT));

    let output = args
        .next()
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(DEFAULT_OUTPUT));

    (input, output)
}

fn flag(value: bool) -> &'static str {
    if value {
        "1"
    } else {
        "0"
    }
}

#[derive(Debug, Deserialize)]
struct Overrides {
    entries: Vec<Entry>,
}

#[derive(Debug, Deserialize)]
struct Entry {
    #[allow(dead_code)]
    name: String,
    mod_id: String,
    enable_dynamic_download: bool,
    allow_non_dataonly_blueprints: bool,
}

#[cfg(test)]
mod tests {
    use super::flag;

    #[test]
    fn converts_bool_to_flag() {
        assert_eq!(flag(true), "1");
        assert_eq!(flag(false), "0");
    }
}
