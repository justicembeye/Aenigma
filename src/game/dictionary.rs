use rand::prelude::IndexedRandom;
use serde::Deserialize;
use std::fs;
use std::path::Path;
use crate::game::{SecretWord, Rarity};

#[derive(Debug, Deserialize, Clone)]
pub struct SubTheme {
    pub name: String,
    pub words: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Theme {
    pub name: String,
    pub sub_themes: Vec<SubTheme>,
}

#[derive(Debug, Deserialize)]
struct DictionaryData {
    themes: Vec<Theme>,
}

pub fn load_dictionary() -> Vec<Theme> {
    let path = Path::new("assets/dictionary.json");
    if !path.exists() {
        eprintln!("Dictionary file not found at {:?}", path);
        return vec![];
    }

    let content = fs::read_to_string(path).unwrap_or_else(|_| {
        eprintln!("Error reading dictionary file at {:?}", path);
        "{}".to_string()
    });
    let data: DictionaryData = serde_json::from_str(&content).unwrap_or_else(|e| {
        eprintln!("Error parsing dictionary: {}", e);
        DictionaryData { themes: vec![] }
    });

    data.themes
}

pub fn get_all_themes() -> Vec<String> {
    let themes = load_dictionary();
    themes.into_iter().map(|t| t.name).collect()
}

pub fn get_sub_themes(theme_name: &str) -> Vec<String> {
    let themes = load_dictionary();
    if let Some(theme) = themes.iter().find(|t| t.name == theme_name) {
        theme.sub_themes.iter().map(|st| st.name.clone()).collect()
    } else {
        vec![]
    }
}

pub fn get_random_word(theme_name: &str, sub_theme_name: Option<&str>) -> SecretWord {
    let themes = load_dictionary();
    let mut rng = rand::rng();

    let mut candidate_word = "MYSTERE".to_string();

    if let Some(theme) = themes.iter().find(|t| t.name == theme_name) {
        if let Some(sub_name) = sub_theme_name {
            if sub_name == "Général (Tous)" {
                let all_words: Vec<&String> = theme.sub_themes.iter().flat_map(|st| &st.words).collect();
                if let Some(word) = all_words.choose(&mut rng) {
                    candidate_word = word.to_string();
                }
            } else if let Some(sub_theme) = theme.sub_themes.iter().find(|st| st.name == sub_name) {
                if let Some(word) = sub_theme.words.choose(&mut rng) {
                    candidate_word = word.to_string();
                }
            }
        } else {
             // Default to all words if no sub-theme specified
             let all_words: Vec<&String> = theme.sub_themes.iter().flat_map(|st| &st.words).collect();
             if let Some(word) = all_words.choose(&mut rng) {
                 candidate_word = word.to_string();
             }
        }
    }

    SecretWord {
        id: 0,
        content: candidate_word.clone(),
        length: candidate_word.len() as u32,
        theme: theme_name.to_string(),
        rarity: Rarity::Common, // Placeholder
        difficulty_score: 1, // Placeholder
    }
}

pub fn get_all_words() -> Vec<String> {
    let themes = load_dictionary();
    themes.into_iter()
        .flat_map(|t| t.sub_themes)
        .flat_map(|st| st.words)
        .collect()
}

pub fn validate_word(word: &str) -> bool {
    let upper_word = word.to_uppercase();
    get_all_words().contains(&upper_word)
}

pub fn get_suggestions(prefix: &str, theme_name: &str, sub_theme_name: Option<&str>) -> Vec<String> {
    let upper_prefix = prefix.to_uppercase();
    if upper_prefix.is_empty() {
        return vec![];
    }

    let themes = load_dictionary();
    let mut candidates = Vec::new();

    if let Some(theme) = themes.iter().find(|t| t.name == theme_name) {
        if let Some(sub_name) = sub_theme_name {
            if sub_name == "Général (Tous)" {
                for sub in &theme.sub_themes {
                    for word in &sub.words {
                        if word.starts_with(&upper_prefix) {
                            candidates.push(word.clone());
                        }
                    }
                }
            } else if let Some(sub_theme) = theme.sub_themes.iter().find(|st| st.name == sub_name) {
                for word in &sub_theme.words {
                    if word.starts_with(&upper_prefix) {
                        candidates.push(word.clone());
                    }
                }
            }
        } else {
             // Default to all words if no sub-theme specified
             for sub in &theme.sub_themes {
                for word in &sub.words {
                    if word.starts_with(&upper_prefix) {
                        candidates.push(word.clone());
                    }
                }
            }
        }
    }
    
    candidates.sort();
    candidates.dedup();
    candidates.into_iter().take(5).collect() // Limit to 5 suggestions
}
