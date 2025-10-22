# Autocommiter CLI

Autocommiter is a small command-line tool that helps you generate concise and
informative git commit messages automatically. It can use an AI inference
service (you provide an API key) to produce messages based on staged changes,
or fall back to a local default message when no API is configured.

Key features
- Generate commit messages from staged changes
- Optional AI model integration (configurable model list)
- Gitmoji support to prepend an emoji that matches the change
- Safe `.gitignore` management to avoid accidentally committing secrets

Quick start

1. Build the project:

```bash
cargo build --release
```

2. Run the CLI (default: generate commit in current directory):

```bash
./target/release/autocommiter generate
```

3. Configure an API key (optional) to enable AI-based messages:

```bash
./target/release/autocommiter set-api-key <KEY>
```

Where to look next
- `src/` — core implementation files
- `docs/ARCHITECTURE.md` — high-level module responsibilities and data flows
- `docs/USAGE.md` — examples and common workflows

Contributing
- See `CONTRIBUTING.md` for contribution guidelines.

License
- MIT

