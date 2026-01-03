use anyhow::{anyhow, Result};
use ignore::WalkBuilder;
use std::path::Path;
use std::process::Command;

pub fn run_git_command(cmd: &str, cwd: &str) -> Result<String> {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", cmd])
            .current_dir(cwd)
            .output()?
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .current_dir(cwd)
            .output()?
    };

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        return Err(anyhow!(
            "Git command failed: {}",
            if !stderr.is_empty() { stderr } else { stdout }
        ));
    }

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

pub fn stage_all_changes(cwd: &str) -> Result<()> {
    run_git_command("git add .", cwd)?;
    Ok(())
}

pub fn get_staged_files(cwd: &str) -> Result<Vec<String>> {
    let output = run_git_command("git diff --staged --name-only", cwd)?;
    if output.is_empty() {
        return Ok(vec![]);
    }
    Ok(output
        .lines()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect())
}

pub fn get_staged_diff_numstat(cwd: &str, file: &str) -> Result<String> {
    let cmd = format!(
        "git diff --staged --numstat -- \"{}\"",
        file.replace("\"", "\\\"")
    );
    run_git_command(&cmd, cwd)
}

pub fn get_staged_diff_unified(cwd: &str, file: &str) -> Result<String> {
    let cmd = format!(
        "git diff --staged --unified=0 -- \"{}\"",
        file.replace("\"", "\\\"")
    );
    run_git_command(&cmd, cwd)
}

pub fn commit_with_message(cwd: &str, message: &str) -> Result<()> {
    use std::io::Write;
    use tempfile::NamedTempFile;

    let mut file = NamedTempFile::new()?;
    file.write_all(message.as_bytes())?;
    file.flush()?;

    let path = file.path().to_string_lossy().to_string();
    let cmd = format!("git commit -F \"{}\"", path.replace("\"", "\\\""));
    run_git_command(&cmd, cwd)?;

    Ok(())
}

pub fn push_changes(cwd: &str) -> Result<()> {
    run_git_command("git push", cwd)?;
    Ok(())
}

pub fn get_repo_root(cwd: &str) -> Result<String> {
    run_git_command("git rev-parse --show-toplevel", cwd)
}

pub fn discover_repositories(root: &str) -> Vec<String> {
    let mut repos = Vec::new();

    // 1. Check if we are inside a git repo and add its root
    if let Ok(toplevel) = get_repo_root(root) {
        if let Ok(abs_toplevel) = Path::new(&toplevel).canonicalize() {
            if let Some(s) = abs_toplevel.to_str() {
                repos.push(s.to_string());
            }
        }
    }

    // 2. Search for sub-repositories
    let walker = WalkBuilder::new(root)
        .hidden(false)
        .filter_entry(|entry| {
            let name = entry.file_name().to_str().unwrap_or("");
            if name == ".git" || name == "node_modules" || name == "target" {
                return false;
            }
            true
        })
        .build();

    for entry in walker {
        if let Ok(entry) = entry {
            let path = entry.path();
            if entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false) {
                if path.join(".git").exists() {
                    if let Ok(abs_path) = path.canonicalize() {
                        if let Some(path_str) = abs_path.to_str() {
                            repos.push(path_str.to_string());
                        }
                    }
                }
            }
        }
    }

    repos.sort();
    repos.dedup();
    repos
}
