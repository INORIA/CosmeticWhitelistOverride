# Cosmetic Whitelist Override

Rust-based generator that turns human-friendly YAML entries into the CSV-format
`whitelist.txt` consumed by CosmeticWhitelistOverride.

## Layout

- `data/overrides.yaml`: source of truth; list of entries (`name`, `mod_id`,
  `enable_dynamic_download`, `allow_non_dataonly_blueprints`).
- `src/main.rs`: generator; reads YAML with `serde_yaml` and emits CSV rows by
  converting booleans to `1`/`0`.
- `public/whitelist.txt`: generated artifact ready to distribute or publish.

## Usage

```shell
cargo run -- data/overrides.yaml public/whitelist.txt
```

Arguments are optional: defaults are the same as in the example above. The tool
creates `public/` if needed, writes the CSV header, then each entry as
`ModId,Enable Dynamic Download,Allow non-dataonly blueprints`.

## Adding Entries

```yaml
entries:
  - name: "Example Cosmetic Mod"
    mod_id: "1234567890"
    enable_dynamic_download: true
    allow_non_dataonly_blueprints: false
```

- `name` is documentation only.
- Booleans stay `true`/`false` in YAML; the generator outputs `1`/`0`.
- Keep the list sorted or grouped however you prefer for review, then rerun the
  generator to refresh `public/whitelist.txt`.

## GitHub Actions

`.github/workflows/generate-whitelist.yml` runs on pushes to `main` that touch
the generator or data. It:

1. Checks out the repo and installs stable Rust.
2. Executes `cargo run -- data/overrides.yaml public/whitelist.txt`.
3. Commits and pushes `public/whitelist.txt` back to `main` if it changed.

This keeps `public/whitelist.txt` in sync automatically whenever the YAML or
generator changes.
