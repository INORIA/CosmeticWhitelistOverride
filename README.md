# Cosmetic Whitelist Override

Rust-based generator that turns human-friendly YAML entries into the flattened
`whitelist.txt` consumed by CosmeticWhitelistOverride. The output format is a
single line with comma-separated rows, and each row is a pipe-delimited triple:
`ModId|Enable Dynamic Download|Allow non-dataonly blueprints` (booleans as
`1`/`0`).

## Layout

- `data/overrides.yaml`: source of truth; list of entries (`name`, `mod_id`,
  `enable_dynamic_download`, `allow_non_dataonly_blueprints`).
- `src/main.rs`: generator; reads YAML with `serde_yaml` and emits joined
  `mod_id|flag|flag` rows with no header or trailing newline.
- `docs/whitelist.txt`: generated artifact ready to distribute or publish.

## Usage

```shell
cargo run -- data/overrides.yaml docs/whitelist.txt OfficialPVECosmeticWhitelist.txt
```

Arguments are optional: the first two fall back to the default paths, and the
third (base whitelist) is optional. When provided, the generator reads that file
(same format) and prepends it to the locally generated rows. The tool creates
`docs/` if needed, converts each YAML entry into `ModId|1|0`, then joins base +
custom rows with commas so the entire file is a single line without a header.

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
  generator to refresh `docs/whitelist.txt`.

## GitHub Actions

`.github/workflows/generate-whitelist.yml` runs on pushes to `main` that touch
the generator or data. It:

1. Checks out the repo and installs stable Rust.
2. Downloads `https://cdn2.arkdedicated.com/asa/OfficialPVECosmeticWhitelist.txt`
   and feeds it to the generator.
3. Executes `cargo run -- data/overrides.yaml docs/whitelist.txt OfficialPVECosmeticWhitelist.txt`.
4. Commits and pushes `docs/whitelist.txt` back to `main` if it changed.

This keeps `docs/whitelist.txt` in sync automatically whenever the YAML or
generator changes.
