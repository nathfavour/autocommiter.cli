use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Gitmoji {
    pub emoji: String,
    pub code: String,
    pub description: String,
    pub keywords: Vec<String>,
}

lazy_static::lazy_static! {
    static ref GITMOJIS: Vec<Gitmoji> = vec![
        // Original 20 emoji
        Gitmoji {
            emoji: "🎨".to_string(),
            code: ":art:".to_string(),
            description: "Improve structure/format".to_string(),
            keywords: vec!["format", "structure", "style", "lint"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        },
        Gitmoji {
            emoji: "⚡".to_string(),
            code: ":zap:".to_string(),
            description: "Improve performance".to_string(),
            keywords: vec!["performance", "speed", "optimize", "fast"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        },
        Gitmoji {
            emoji: "🔥".to_string(),
            code: ":fire:".to_string(),
            description: "Remove code/files".to_string(),
            keywords: vec!["remove", "delete", "clean", "unused"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        },
        Gitmoji {
            emoji: "🐛".to_string(),
            code: ":bug:".to_string(),
            description: "Fix bug".to_string(),
            keywords: vec!["fix", "bug", "issue", "error", "crash"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        },
        Gitmoji {
            emoji: "✨".to_string(),
            code: ":sparkles:".to_string(),
            description: "New feature".to_string(),
            keywords: vec!["feature", "new", "add", "implement"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        },
        Gitmoji {
            emoji: "📝".to_string(),
            code: ":memo:".to_string(),
            description: "Add documentation".to_string(),
            keywords: vec!["docs", "documentation", "comment", "readme"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        },
        Gitmoji {
            emoji: "🚀".to_string(),
            code: ":rocket:".to_string(),
            description: "Deploy stuff".to_string(),
            keywords: vec!["deploy", "release", "publish", "launch"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        },
        Gitmoji {
            emoji: "💅".to_string(),
            code: ":nail_care:".to_string(),
            description: "Polish code".to_string(),
            keywords: vec!["polish", "refine", "improve"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        },
        Gitmoji {
            emoji: "✅".to_string(),
            code: ":white_check_mark:".to_string(),
            description: "Add tests".to_string(),
            keywords: vec!["test", "tests", "testing"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        },
        Gitmoji {
            emoji: "🔐".to_string(),
            code: ":lock:".to_string(),
            description: "Security fix".to_string(),
            keywords: vec!["security", "auth", "encrypt"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        },
        Gitmoji {
            emoji: "⬆️".to_string(),
            code: ":arrow_up:".to_string(),
            description: "Upgrade dependencies".to_string(),
            keywords: vec!["upgrade", "update", "dependency", "dependencies"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        },
        Gitmoji {
            emoji: "⬇️".to_string(),
            code: ":arrow_down:".to_string(),
            description: "Downgrade dependencies".to_string(),
            keywords: vec!["downgrade"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        },
        Gitmoji {
            emoji: "📦".to_string(),
            code: ":package:".to_string(),
            description: "Update packages".to_string(),
            keywords: vec!["package", "npm", "yarn", "bundler"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        },
        Gitmoji {
            emoji: "🔧".to_string(),
            code: ":wrench:".to_string(),
            description: "Configuration".to_string(),
            keywords: vec!["config", "configuration", "settings"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        },
        Gitmoji {
            emoji: "🌐".to_string(),
            code: ":globe_with_meridians:".to_string(),
            description: "i18n/localization".to_string(),
            keywords: vec!["i18n", "translation", "locale", "language"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        },
        Gitmoji {
            emoji: "♿".to_string(),
            code: ":wheelchair:".to_string(),
            description: "Accessibility".to_string(),
            keywords: vec!["accessibility", "a11y", "aria"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        },
        Gitmoji {
            emoji: "🚨".to_string(),
            code: ":rotating_light:".to_string(),
            description: "Fix warnings".to_string(),
            keywords: vec!["warning", "lint"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        },
        Gitmoji {
            emoji: "🔍".to_string(),
            code: ":mag:".to_string(),
            description: "SEO".to_string(),
            keywords: vec!["seo"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        },
        Gitmoji {
            emoji: "🍎".to_string(),
            code: ":apple:".to_string(),
            description: "macOS fix".to_string(),
            keywords: vec!["macos", "mac", "apple"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        },
        Gitmoji {
            emoji: "🐧".to_string(),
            code: ":penguin:".to_string(),
            description: "Linux fix".to_string(),
            keywords: vec!["linux", "ubuntu"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        },
        // Additional emoji for various languages and frameworks
        Gitmoji {
            emoji: "🐍".to_string(),
            code: ":snake:".to_string(),
            description: "Python changes".to_string(),
            keywords: vec!["python", "django", "flask", "pip", "pytorch"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        },
        Gitmoji {
            emoji: "📚".to_string(),
            code: ":books:".to_string(),
            description: "Node.js/JavaScript".to_string(),
            keywords: vec!["node", "npm", "javascript", "express", "typescript"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        },
        Gitmoji {
            emoji: "🦀".to_string(),
            code: ":crab:".to_string(),
            description: "Rust changes".to_string(),
            keywords: vec!["rust", "cargo", "tokio", "wasm"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        },
        Gitmoji {
            emoji: "☕".to_string(),
            code: ":coffee:".to_string(),
            description: "Java changes".to_string(),
            keywords: vec!["java", "spring", "maven", "gradle", "jvm"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        },
        Gitmoji {
            emoji: "🐳".to_string(),
            code: ":whale:".to_string(),
            description: "Docker changes".to_string(),
            keywords: vec!["docker", "container", "dockerfile", "image"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        },
    ];
}

fn calculate_fuzzy_score(commit_message: &str, gitmoji: &Gitmoji) -> u32 {
    let msg = commit_message.to_lowercase();
    let mut score = 0u32;

    for keyword in &gitmoji.keywords {
        if msg.contains(keyword) {
            score += 40;
        }
        if keyword.len() >= 3 && msg.contains(&keyword[..3]) {
            score += 10;
        }
    }

    for word in gitmoji.description.to_lowercase().split_whitespace() {
        if word.len() > 2 && msg.contains(word) {
            score += 15;
        }
    }

    std::cmp::min(score, 100)
}

pub fn find_best_gitmoji(commit_message: &str) -> Option<Gitmoji> {
    if commit_message.trim().is_empty() {
        return None;
    }

    let mut best_gitmoji: Option<Gitmoji> = None;
    let mut best_score = 30u32;

    for gitmoji in GITMOJIS.iter() {
        let score = calculate_fuzzy_score(commit_message, gitmoji);
        if score > best_score {
            best_score = score;
            best_gitmoji = Some(gitmoji.clone());
        }
    }

    best_gitmoji
}

pub fn get_random_gitmoji() -> Gitmoji {
    let mut rng = rand::thread_rng();
    GITMOJIS
        .choose(&mut rng)
        .cloned()
        .unwrap_or_else(|| Gitmoji {
            emoji: "⭐".to_string(),
            code: ":star:".to_string(),
            description: "General".to_string(),
            keywords: vec![],
        })
}

pub fn prepend_gitmoji(commit_message: &str, gitmoji: &Gitmoji) -> String {
    format!("{} {}", gitmoji.emoji, commit_message)
}

pub fn get_gitmojified_message(commit_message: &str) -> String {
    let best_match = find_best_gitmoji(commit_message);
    let gitmoji = best_match.unwrap_or_else(get_random_gitmoji);
    prepend_gitmoji(commit_message, &gitmoji)
}
