use anyhow::{anyhow, Result};
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

pub fn is_git_repository(cwd: &str) -> bool {
    run_git_command("git rev-parse --git-dir", cwd).is_ok()
}

pub fn get_repo_root(cwd: &str) -> Result<String> {
    run_git_command("git rev-parse --show-toplevel", cwd)
}
