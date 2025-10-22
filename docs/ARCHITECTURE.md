Autocommiter Architecture

Overview

Autocommiter is structured as a small CLI with focused modules. The design
prioritizes clarity and simple dependencies so it is easy to inspect staged
changes, build a compact summary, and ask a model for a short commit message.

Modules

- api_client
  - Responsible for calling a chat-completion style inference endpoint and
    extracting a single-line commit message from the response. The client
    constructs a prompt that contains a short list of filenames and a compact
    JSON summary of file changes.

- changes_summarizer
  - Inspects staged files and tries to create compact descriptors for each
    file (numstat like `5+/2−` where possible or a short unified diff snippet).
  - Provides `compress_to_json` which attempts multiple levels of lossy
    compression (reduce change strings, drop files from the end) until the
    payload fits a specified maximum length.

- config
  - Simple JSON file stored at `~/.autocommiter.json`.
  - Config options: `api_key`, `selected_model`, `enable_gitmoji`,
    `update_gitignore`, `gitignore_patterns`.
  - Helpers for getting/setting common values.

- git
  - Executes git commands using the system shell and returns stdout or
    errors. Used for staging, inspecting staged diffs, committing and pushing.
  - Small helpers to build ephemeral commit message files for `git commit -F`.

- model_manager
  - Optionally fetches a list of available models from a Models API and
    caches them in `~/.autocommiter.models.json`.
  - Falls back to a small list of curated defaults when fetching fails.

- gitmoji
  - Contains a curated set of gitmoji with keywords and a fuzzy scorer. When
    enabled, the best-matching emoji will be prepended to the generated
    commit message.

Data flow for `generate` command

1. Ensure the directory is a git repository and determine repo root.
2. Optionally add safe `.gitignore` patterns (if enabled in config).
3. Stage all changes and gather a list of staged files.
4. Build `FileChange` objects via `changes_summarizer`.
5. Compress filenames and changes to a small JSON string.
6. If `api_key` is set, call `api_client::generate_commit_message` to request
   a message using the selected model.
7. If gitmoji is enabled, prepend the best-fitting gitmoji.
8. Commit using `git commit -F` with the generated message and optionally
   push.

Design notes

- The code intentionally keeps network interaction isolated in `api_client`
  and `model_manager` so the rest of the logic is testable offline.
- `changes_summarizer::compress_to_json` is defensive: it attempts several
  reductions to ensure the prompt is small enough for model input limits.
- The `git` helper uses shell commands which is simple and portable for a CLI
  tool; switching to a git library (libgit2) would be possible if binary size
  or finer control were needed.

Security considerations

- API keys are stored in plain JSON in the user's home directory. For
  sensitive deployments, consider integrating OS-native secure storage (e.g.
  keyring) instead of a plaintext file.
- `.gitignore` updates are opt-in (`update_gitignore`) to avoid surprise
  modifications to repository files.
