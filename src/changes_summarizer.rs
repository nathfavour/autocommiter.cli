use anyhow::Result;
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileChange {
    pub file: String,
    pub change: String,
}

pub async fn get_staged_files(cwd: &str) -> Result<Vec<String>> {
    crate::git::get_staged_files(cwd)
}

pub async fn analyze_file_change(cwd: &str, file: &str) -> Result<String> {
    match crate::git::get_staged_diff_numstat(cwd, file) {
        Ok(diff) => {
            if diff.is_empty() {
                return Ok("unchanged".to_string());
            }

            if let Some(first_line) = diff.lines().next() {
                let parts: Vec<&str> = first_line.split('\t').collect();
                if parts.len() >= 3 {
                    let added = if parts[0] == "-" {
                        0
                    } else {
                        parts[0].parse::<usize>().unwrap_or(0)
                    };
                    let removed = if parts[1] == "-" {
                        0
                    } else {
                        parts[1].parse::<usize>().unwrap_or(0)
                    };
                    return Ok(format!("{}+/{}−", added, removed));
                }
            }

            Ok("mod".to_string())
        }
        Err(_) => match crate::git::get_staged_diff_unified(cwd, file) {
            Ok(hunks) => {
                let first = hunks
                    .lines()
                    .map(|l| l.trim())
                    .find(|l| !l.is_empty())
                    .unwrap_or("mod");
                let truncated = &first[..std::cmp::min(40, first.len())];
                let collapsed = Regex::new(r"\s+")
                    .ok()
                    .and_then(|r| Some(r.replace_all(truncated, " ").to_string()))
                    .unwrap_or_else(|| truncated.to_string());
                Ok(collapsed)
            }
            Err(_) => Ok("err".to_string()),
        },
    }
}

pub async fn build_file_changes(cwd: &str) -> Result<Vec<FileChange>> {
    let files = get_staged_files(cwd).await?;
    let mut changes = Vec::new();

    for file in files {
        let change = analyze_file_change(cwd, &file).await?;
        changes.push(FileChange { file, change });
    }

    Ok(changes)
}

pub fn compress_to_json(file_changes: &[FileChange], max_len: usize) -> String {
    if file_changes.is_empty() {
        return r#"{"files":[]}"#.to_string();
    }

    let escape_str = |s: &str| -> String {
        s.replace('\\', "\\\\")
            .replace('"', "\\\"")
            .replace('\n', "\\n")
            .replace('\r', "\\r")
    };

    let serialize = |arr: &[FileChange], map_fn: &dyn Fn(&str) -> String| -> String {
        let items: Vec<String> = arr
            .iter()
            .map(|fc| {
                format!(
                    r#"{{"f":"{}","c":"{}"}}"#,
                    escape_str(&fc.file),
                    escape_str(&map_fn(&fc.change))
                )
            })
            .collect();
        format!(r#"{{"files":[{}]}}"#, items.join(","))
    };

    let maps: Vec<Box<dyn Fn(&str) -> String>> = vec![
        Box::new(|c: &str| c.to_string()),
        Box::new(|c: &str| c[..std::cmp::min(12, c.len())].to_string()),
        Box::new(|c: &str| c[..std::cmp::min(6, c.len())].to_string()),
        Box::new(|c: &str| c[..std::cmp::min(3, c.len())].to_string()),
        Box::new(|c: &str| c[..std::cmp::min(1, c.len())].to_string()),
    ];

    for map_fn in &maps {
        for keep in (1..=file_changes.len()).rev() {
            let arr = &file_changes[..keep];
            let s = serialize(arr, map_fn);
            if s.len() <= max_len {
                return s;
            }
        }
    }

    let minimal = file_changes
        .iter()
        .take(1)
        .map(|fc| {
            let filename = fc.file.split('/').last().unwrap_or(&fc.file).to_string();
            FileChange {
                file: filename,
                change: "mod".to_string(),
            }
        })
        .collect::<Vec<_>>();

    serialize(&minimal, &|_| "mod".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compress_to_json_empty() {
        let result = compress_to_json(&[], 400);
        assert_eq!(result, r#"{"files":[]}"#);
    }

    #[test]
    fn test_compress_to_json_single_file() {
        let changes = vec![FileChange {
            file: "src/main.rs".to_string(),
            change: "5+/2−".to_string(),
        }];
        let result = compress_to_json(&changes, 400);
        assert!(result.contains("main.rs"));
        assert!(result.contains("5+/2−"));
    }
}
