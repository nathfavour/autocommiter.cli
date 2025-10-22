mod api_client;
mod changes_summarizer;
mod config;
mod git;
mod gitmoji;
mod model_manager;

use anyhow::{anyhow, Result};
use clap::{Parser, Subcommand};
use colored::*;
use std::io::{self, Write};

#[derive(Parser)]
#[command(name = "autocommiter")]
#[command(about = "Auto-generate git commit messages using AI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Generate commit message and commit changes")]
    Generate {
        #[arg(
            short,
            long,
            help = "Path to git repository (defaults to current directory)"
        )]
        repo: Option<String>,

        #[arg(short, long, help = "Skip pushing after commit")]
        no_push: bool,

        #[arg(
            short,
            long,
            help = "Don't ask for confirmation before committing"
        )]
        force: bool,
    },

    #[command(name = "set-api-key", about = "Set GitHub API key")]
    SetApiKey {
        #[arg(value_name = "KEY", help = "GitHub API key")]
        key: Option<String>,
    },

    #[command(name = "get-api-key", about = "Get stored API key")]
    GetApiKey,

    #[command(name = "refresh-models", about = "Refresh available AI models from GitHub Models API")]
    RefreshModels,

    #[command(name = "list-models", about = "List available AI models")]
    ListModels,

    #[command(name = "select-model", about = "Select default AI model")]
    SelectModel,

    #[command(name = "get-model", about = "Get current default model")]
    GetModel,

    #[command(name = "toggle-gitmoji", about = "Enable/disable gitmoji prefixes")]
    ToggleGitmoji,

    #[command(name = "get-config", about = "Display current configuration")]
    GetConfig,

    #[command(name = "reset-config", about = "Reset configuration to defaults")]
    ResetConfig,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::WARN)
        .init();

    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Generate {
            repo,
            no_push,
            force,
        }) => {
            generate_commit(repo.as_deref(), no_push, force).await
        }
        Some(Commands::SetApiKey { key }) => set_api_key(key).await,
        Some(Commands::GetApiKey) => get_api_key(),
        Some(Commands::RefreshModels) => refresh_models().await,
        Some(Commands::ListModels) => list_models(),
        Some(Commands::SelectModel) => select_model().await,
        Some(Commands::GetModel) => get_model(),
        Some(Commands::ToggleGitmoji) => toggle_gitmoji(),
        Some(Commands::GetConfig) => get_config(),
        Some(Commands::ResetConfig) => reset_config(),
        None => {
            generate_commit(None, false, false).await
        }
    }
}

async fn generate_commit(repo_path: Option<&str>, no_push: bool, force: bool) -> Result<()> {
    let cwd = repo_path.unwrap_or(".");

    println!("{}", "🪄 Autocommiter: Generating commit...".cyan());

    // Check if it's a git repo
    if !git::is_git_repository(cwd) {
        return Err(anyhow!("Not a git repository"));
    }

    let repo_root = git::get_repo_root(cwd)?;
    println!("{} {}", "📂 Repository:".cyan(), repo_root.dimmed());

    // Ensure gitignore safety
    println!("{}", "🛡️  Ensuring .gitignore safety...".cyan());
    ensure_gitignore_safety(&repo_root)?;

    // Stage changes
    println!("{}", "📦 Staging changes...".cyan());
    git::stage_all_changes(&repo_root)?;

    // Check for staged files
    println!("{}", "📋 Checking staged changes...".cyan());
    let staged_files = git::get_staged_files(&repo_root)?;
    if staged_files.is_empty() {
        println!(
            "{}",
            "ℹ️  No changes to commit — Autocommit skipped.".yellow()
        );
        return Ok(());
    }

    println!("{} {} files", "✓ Found".green(), staged_files.len());
    for file in &staged_files {
        println!("  - {}", file.dimmed());
    }

    // Generate message
    let message = generate_message(&repo_root).await?;
    println!("{} {}", "💬 Message:".cyan(), message.italic());

    // Ask for confirmation if not forced
    if !force {
        print!("{}", "\n🤔 Proceed with commit? (y/n): ".cyan());
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        if !input.trim().eq_ignore_ascii_case("y") {
            println!("{}", "❌ Cancelled.".red());
            return Ok(());
        }
    }

    // Commit
    println!("{}", "✍️  Committing changes...".cyan());
    git::commit_with_message(&repo_root, &message)?;
    println!("{}", "✓ Commit successful!".green());

    // Push
    if !no_push {
        println!("{}", "🚀 Pushing to remote...".cyan());
        git::push_changes(&repo_root)?;
        println!("{}", "✓ Push successful!".green());
    }

    println!("{}", "✨ Done!".green().bold());
    Ok(())
}

async fn generate_message(repo_root: &str) -> Result<String> {
    let config = config::load_config()?;

    // Try API-based generation if API key exists
    if let Some(api_key) = &config.api_key {
        if let Ok(message) = try_api_generation(repo_root, api_key, &config).await {
            return Ok(message);
        }
    }

    // Fallback to local generation
    Ok("chore: automated commit generated by Autocommiter".to_string())
}

async fn try_api_generation(
    repo_root: &str,
    api_key: &str,
    config: &config::Config,
) -> Result<String> {
    let model = config
        .selected_model
        .clone()
        .unwrap_or_else(|| "gpt-4o-mini".to_string());

    println!("{} {}...", "🤖 Generating with model:".cyan(), model.dimmed());

    let file_changes = changes_summarizer::build_file_changes(repo_root).await?;
    let file_names = file_changes
        .iter()
        .take(50)
        .map(|f| f.file.as_str())
        .collect::<Vec<_>>()
        .join("\n");
    let compressed_json = changes_summarizer::compress_to_json(&file_changes, 400);

    let message = api_client::generate_commit_message(api_key, &file_names, &compressed_json, &model)
        .await?;

    // Apply gitmoji if enabled
    let message = if config.enable_gitmoji.unwrap_or(false) {
        gitmoji::get_gitmojified_message(&message)
    } else {
        message
    };

    Ok(message)
}

async fn set_api_key(key: Option<String>) -> Result<()> {
    let key_to_set = if let Some(key) = key {
        key
    } else {
        print!("{}", "Enter GitHub API key (will be stored securely): ".cyan());
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        input.trim().to_string()
    };

    if key_to_set.is_empty() {
        return Err(anyhow!("API key cannot be empty"));
    }

    config::set_api_key(key_to_set)?;
    println!("{}", "✓ API key saved!".green());
    Ok(())
}

fn get_api_key() -> Result<()> {
    match config::get_api_key()? {
        Some(key) => {
            let masked = if key.len() > 8 {
                format!("{}...{}", &key[..4], &key[key.len() - 4..])
            } else {
                "****".to_string()
            };
            println!("{} {}", "🔑 API Key:".cyan(), masked.yellow());
            Ok(())
        }
        None => {
            println!("{}", "ℹ️  No API key set. Use 'set-api-key' to add one.".yellow());
            Ok(())
        }
    }
}

async fn refresh_models() -> Result<()> {
    let api_key = config::get_api_key()?
        .ok_or_else(|| anyhow!("API key not set. Use 'set-api-key' first."))?;

    println!("{}", "🔄 Fetching models from GitHub Models API...".cyan());
    let (success, message, count) = model_manager::refresh_model_list(&api_key).await?;

    if success {
        println!("{} {} models cached", "✓".green(), count);
    } else {
        println!("{} {}", "✗".red(), message);
    }
    Ok(())
}

fn list_models() -> Result<()> {
    let models = model_manager::list_available_models()?;
    let current = config::get_selected_model()?;

    println!("{}\n", "📋 Available Models:".cyan().bold());
    for model in models {
        let marker = if model.id == current { "→" } else { " " };
        println!("{} {}", marker.green(), model.name.cyan());
        if let Some(friendly) = &model.friendly_name {
            println!("   {}", friendly.dimmed());
        }
        if let Some(summary) = &model.summary {
            println!("   {}", summary.dimmed());
        }
        println!();
    }
    Ok(())
}

async fn select_model() -> Result<()> {
    let models = model_manager::list_available_models()?;
    if models.is_empty() {
        return Err(anyhow!("No models available"));
    }

    println!("{}\n", "🤖 Select a Model:".cyan().bold());
    for (idx, model) in models.iter().enumerate() {
        println!("{}. {} ({})", idx + 1, model.name.cyan(), model.friendly_name.as_ref().unwrap_or(&model.name).dimmed());
    }

    print!("\n{}", "Enter choice (1-{}): ".cyan());
    println!("{}", models.len());
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let choice: usize = input.trim().parse().map_err(|_| anyhow!("Invalid choice"))?;

    if choice < 1 || choice > models.len() {
        return Err(anyhow!("Choice out of range"));
    }

    let selected = &models[choice - 1];
    config::set_selected_model(selected.id.clone())?;
    println!("{} {}", "✓ Selected:".green(), selected.name.cyan());
    Ok(())
}

fn get_model() -> Result<()> {
    let model = config::get_selected_model()?;
    println!("{} {}", "🤖 Current Model:".cyan(), model.yellow());
    Ok(())
}

fn toggle_gitmoji() -> Result<()> {
    let config = config::load_config()?;
    let current = config.enable_gitmoji.unwrap_or(false);
    let new_value = !current;

    let mut new_config = config;
    new_config.enable_gitmoji = Some(new_value);
    config::save_config(&new_config)?;

    if new_value {
        println!("{} {}", "✓ Gitmoji".green(), "enabled".green());
    } else {
        println!("{} {}", "✓ Gitmoji".green(), "disabled".yellow());
    }
    Ok(())
}

fn get_config() -> Result<()> {
    let config = config::load_config()?;
    println!("{}\n", "⚙️  Configuration:".cyan().bold());
    
    println!("{}:", "API Key".cyan());
    match &config.api_key {
        Some(key) => {
            let masked = if key.len() > 8 {
                format!("{}...{}", &key[..4], &key[key.len() - 4..])
            } else {
                "****".to_string()
            };
            println!("  {}", masked.yellow());
        }
        None => println!("  {}", "Not set".dimmed()),
    }

    println!("\n{}:", "Selected Model".cyan());
    println!(
        "  {}",
        config.selected_model.unwrap_or_default().yellow()
    );

    println!("\n{}:", "Gitmoji Enabled".cyan());
    println!(
        "  {}",
        if config.enable_gitmoji.unwrap_or(false) {
            "Yes".green()
        } else {
            "No".red()
        }
    );

    println!("\n{}:", "Update Gitignore".cyan());
    println!(
        "  {}",
        if config.update_gitignore.unwrap_or(false) {
            "Yes".green()
        } else {
            "No".red()
        }
    );

    Ok(())
}

fn reset_config() -> Result<()> {
    let default_config = config::Config::default();
    config::save_config(&default_config)?;
    println!("{}", "✓ Configuration reset to defaults!".green());
    Ok(())
}

fn ensure_gitignore_safety(repo_root: &str) -> Result<()> {
    let config = config::load_config()?;
    let should_update = config.update_gitignore.unwrap_or(false);

    if !should_update {
        tracing::debug!("Gitignore updates disabled");
        return Ok(());
    }

    let gitignore_path = std::path::Path::new(repo_root).join(".gitignore");
    let existing = std::fs::read_to_string(&gitignore_path).unwrap_or_default();
    let patterns = config.gitignore_patterns.unwrap_or_default();

    let lines: Vec<&str> = existing
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .collect();

    let mut to_append = Vec::new();
    for pattern in patterns {
        if !lines.iter().any(|l| l == &pattern) {
            to_append.push(format!("# Added by Autocommiter: ensure {}", pattern));
            to_append.push(pattern);
        }
    }

    if !to_append.is_empty() {
        let content = if existing.trim().is_empty() {
            to_append.join("\n") + "\n"
        } else {
            existing + "\n" + &to_append.join("\n") + "\n"
        };
        std::fs::write(&gitignore_path, content)?;
    }

    Ok(())
}
