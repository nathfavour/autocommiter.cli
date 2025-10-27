Autocommiter Usage

Installation

Build from source:

```bash
cargo build --release
```

The binary will be available at `target/release/autocommiter`.

Basic workflows

1. Generate a commit for the current repository

```bash
./target/release/autocommiter generate
```

This will stage all changes, summarize them, attempt to generate a message
(using configured API if available), ask for confirmation, then commit and
optionally push.

2. Set an API key for model-based generation

```bash
./target/release/autocommiter set-api-key
# or
./target/release/autocommiter set-api-key <KEY>
```

3. List and select models

```bash
./target/release/autocommiter refresh-models
./target/release/autocommiter list-models
./target/release/autocommiter select-model
```

4. Toggle gitmoji

```bash
./target/release/autocommiter toggle-gitmoji
```

5. Toggle skip confirmation

```bash
./target/release/autocommiter toggle-skip-confirmation
```

This allows autocommit to proceed without prompting for confirmation.

Configuration file

The config file is `~/.autocommiter.json`. Fields:

- api_key: string | null — API key used by `api_client`
- selected_model: string — default model id (e.g., `gpt-4o-mini`)
- enable_gitmoji: bool — whether to prepend gitmoji
- skip_confirmation: bool — whether to skip commit confirmation prompt (enabled
  via `toggle-skip-confirmation` or CLI `--force` flag)
- update_gitignore: bool — whether the tool should append recommended patterns
  to the repository's `.gitignore`
- gitignore_patterns: [string] — list of patterns to ensure are in `.gitignore`

Troubleshooting

- Not a git repository
  - Ensure you run the command in a git repo or pass `--repo` with the path.

- No staged changes
  - The tool stages all files before summarizing, but if no files changed it
    will exit gracefully.

- API errors
  - If the inference API call fails the CLI falls back to a local default
    message. Check `~/.autocommiter.json` for the API key and network
    connectivity to the endpoint.

Developer notes

- Run tests for the summarizer with `cargo test -p autocommiter`.
- Generate docs locally with `cargo doc --no-deps`.

CLI flags

- `--force` or `-f`: Skip confirmation prompt for a single run
- `--no-push` or `-n`: Skip pushing after commit
- `--repo <PATH>`: Specify a different git repository (defaults to current directory)
