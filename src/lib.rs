//! Autocommiter
//!
//! Autocommiter is a small CLI tool that generates concise, useful git commit
//! messages automatically using an AI inference API (configurable) and a
//! local fallback. It stages changes, summarizes staged diffs, asks the AI for
//! a commit message, optionally prepends a gitmoji, and commits & pushes the
//! changes.
//!
//! High-level responsibilities by module:
//! - `api_client` — HTTP/inference API calls and commit-message generation.
//! - `changes_summarizer` — Builds lightweight summaries of staged changes and
//!   compresses them into a small JSON payload to send to the model service.
//! - `config` — Load/save user configuration stored in the home directory
//!   (`~/.autocommiter.json`). Provides defaults and helpers for common config
//!   operations (API key, selected model, flags).
//! - `git` — Thin wrapper around git command invocations used to stage,
//!   inspect, commit and push changes.
//! - `gitmoji` — Utilities to select or guess a gitmoji and prepend it to
//!   generated messages.
//! - `model_manager` — Fetch and cache available models from the Models API and
//!   expose a local cached list.
//!
//! Usage (short):
//! ```text
//! # generate commit for current repo (default)
//! autocommiter generate
//! # set an API key for model-based generation
//! autocommiter set-api-key <KEY>
//! ```
//!
//! Configuration:
//! - The config file is stored at `~/.autocommiter.json` and contains fields
//!   like `api_key`, `selected_model`, `enable_gitmoji` and
//!   `update_gitignore`.
//!
//! See the `docs/` directory for architecture notes and usage examples.

pub mod api_client;
pub mod changes_summarizer;
pub mod config;
pub mod git;
pub mod gitmoji;
pub mod model_manager;
