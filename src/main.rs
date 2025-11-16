use std::{
    env,
    fs::{self, File},
    io::{BufWriter, Write},
    path::PathBuf,
};

use anyhow::{Context, Result};
use serde::Deserialize;

const DEFAULT_INPUT: &str = "data/overrides.yaml";
const DEFAULT_OUTPUT: &str = "docs/whitelist.txt";

fn main() -> Result<()> {
    let (input_path, output_path, base_path) = parse_args();

    let overrides: Overrides =
        serde_yaml::from_reader(File::open(&input_path).with_context(|| {
            format!("Unable to open YAML input at {}", input_path.display())
        })?)
        .with_context(|| format!("Unable to parse YAML at {}", input_path.display()))?;

    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent).with_context(|| {
            format!("Unable to create output directory {}", parent.display())
        })?;
    }

    let output_file = File::create(&output_path).with_context(|| {
        format!("Unable to create output file {}", output_path.display())
    })?;
    let mut writer = BufWriter::new(output_file);

    let custom_rows: Vec<String> = overrides
        .entries
        .into_iter()
        .map(|entry| format_row(&entry))
        .collect();
    let custom_payload = custom_rows.join(",");

    let base_payload = load_base_payload(base_path.as_ref())?;
    let payload = build_payload(base_payload, custom_payload);

    writer
        .write_all(payload.as_bytes())
        .with_context(|| format!("Unable to write output file {}", output_path.display()))?;

    writer.flush()?;

    println!(
        "Generated {} from {}",
        output_path.display(),
        input_path.display()
    );

    Ok(())
}

fn parse_args() -> (PathBuf, PathBuf, Option<PathBuf>) {
    let mut args = env::args().skip(1);
    let input = args
        .next()
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(DEFAULT_INPUT));

    let output = args
        .next()
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(DEFAULT_OUTPUT));

    let base = args.next().map(PathBuf::from);

    (input, output, base)
}

fn flag(value: bool) -> &'static str {
    if value {
        "1"
    } else {
        "0"
    }
}

fn load_base_payload(base_path: Option<&PathBuf>) -> Result<Option<String>> {
    if let Some(path) = base_path {
        let content = fs::read_to_string(path).with_context(|| {
            format!("Unable to read base whitelist at {}", path.display())
        })?;
        Ok(normalize_payload(&content))
    } else {
        Ok(None)
    }
}

fn normalize_payload(content: &str) -> Option<String> {
    let trimmed = content.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

fn build_payload(base: Option<String>, custom: String) -> String {
    let mut segments = Vec::new();
    if let Some(base_payload) = base {
        if !base_payload.is_empty() {
            segments.push(base_payload);
        }
    }
    if !custom.is_empty() {
        segments.push(custom);
    }

    segments.join(",")
}

fn format_row(entry: &Entry) -> String {
    format!(
        "{}|{}|{}",
        entry.mod_id,
        flag(entry.enable_dynamic_download),
        flag(entry.allow_non_dataonly_blueprints)
    )
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
    use super::{
        build_payload, flag, format_row, load_base_payload, normalize_payload, Entry,
    };
    use std::fs;

    #[test]
    fn converts_bool_to_flag() {
        assert_eq!(flag(true), "1");
        assert_eq!(flag(false), "0");
    }

    #[test]
    fn formats_row_with_pipe_delimiters() {
        let entry = Entry {
            name: String::from("Test"),
            mod_id: String::from("mod"),
            enable_dynamic_download: true,
            allow_non_dataonly_blueprints: false,
        };

        assert_eq!(format_row(&entry), "mod|1|0");
    }

    #[test]
    fn normalizes_payload() {
        assert_eq!(normalize_payload("row|1|1\n"), Some(String::from("row|1|1")));
        assert!(normalize_payload("\n\n").is_none());
    }

    #[test]
    fn builds_payload_from_segments() {
        assert_eq!(build_payload(Some("a".into()), String::from("b")), "a,b");
        assert_eq!(build_payload(Some(String::new()), String::from("b")), "b");
        assert_eq!(build_payload(None, String::from("b")), "b");
        assert_eq!(build_payload(Some("a".into()), String::new()), "a");
    }

    #[test]
    fn loads_base_payload() {
        let temp_dir = std::env::temp_dir();
        let path = temp_dir.join("whitelist_base_test.txt");
        fs::write(&path, "base|1|1\n").unwrap();

        let payload = load_base_payload(Some(&path)).unwrap();
        assert_eq!(payload.unwrap(), "base|1|1");

        fs::remove_file(path).unwrap();
    }
}
