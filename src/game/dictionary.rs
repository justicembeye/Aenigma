use rand::prelude::IndexedRandom;
use crate::game::{SecretWord, Rarity, Difficulty};

#[allow(dead_code)]
pub struct Theme {
    pub name: &'static str,
    pub words: Vec<(&'static str, u32)>, // (word, difficulty_score)
}

pub fn get_themes(difficulty: Difficulty) -> Vec<&'static str> {
    match difficulty {
        Difficulty::Easy => vec!["Pays", "Nature"],
        Difficulty::Normal => vec!["Pays", "Nature", "Technologie", "Espionnage"],
        Difficulty::Hard | Difficulty::Expert => vec!["Pays", "Nature", "Technologie", "Espionnage", "Mystère"],
    }
}

fn get_words_for_theme(theme: &str) -> Vec<(&'static str, u32)> {
    match theme {
        "Pays" => vec![
            ("FRANCE", 1), ("ESPAGNE", 1), ("ITALIE", 1), ("JAPON", 2),
            ("BRESIL", 2), ("CANADA", 1), ("CHINE", 1), ("INDE", 2),
            ("RUSSIE", 2), ("AUSTRALIE", 3), ("MEXIQUE", 2), ("EGYPTE", 2),
            ("ARGENTINE", 3), ("ALLEMAGNE", 2), ("SENEGAL", 3), ("MAROC", 2),
            ("FIDJI", 3), ("TOGO", 2), ("OMAN", 3), // Mots courts mais rares
        ],
        "Technologie" => vec![
            ("ROBOT", 1), ("CYBER", 2), ("LASER", 1), ("DRONE", 1),
            ("ALGORITHME", 3), ("INTERFACE", 2), ("RESEAU", 2),
            ("QUANTIQUE", 3), ("VIRTUEL", 2), ("SYSTEME", 1),
            ("FLUX", 3), ("BUG", 2), // Courts/Rares
        ],
        "Espionnage" => vec![
            ("AGENT", 1), ("SECRET", 1), ("CODE", 1), ("PISTE", 1),
            ("INFILTRE", 3), ("MISSION", 2), ("GADGET", 2),
            ("CRYPTAGE", 3), ("SILENCE", 2), ("OMBRE", 1),
            ("MOLE", 3), // Rare
        ],
        "Nature" => vec![
            ("ARBRE", 1), ("FLEUR", 1), ("OCEAN", 1), ("PLUIE", 1),
            ("MONTAGNE", 2), ("VOLCAN", 2), ("JUNGLE", 2),
            ("HORIZON", 3), ("TORNADE", 3), ("ECLIPSE", 3),
            ("LYNX", 3), ("ROC", 2), // Courts/Rares
        ],
        "Mystère" => vec![
            ("ENIGME", 2), ("DOUTE", 1), ("TRACE", 1), ("LUEUR", 1),
            ("INCONNU", 2), ("SECRET", 1), ("VERITE", 2),
            ("ILLUSION", 3), ("MIRAGE", 2), ("ABYSSE", 3),
            ("AME", 2), ("VIDE", 2), // Courts
        ],
        _ => vec![("CODE", 1)],
    }
}

pub fn get_random_word(theme: &str, difficulty: Difficulty) -> SecretWord {
    let words = get_words_for_theme(theme);

    // Filtrer les mots selon la difficulté
    let allowed_scores = match difficulty {
        Difficulty::Easy => vec![1],
        Difficulty::Normal => vec![1, 2],
        Difficulty::Hard => vec![2, 3],
        Difficulty::Expert => vec![3],
    };

    let filtered_words: Vec<&(&str, u32)> = words.iter()
        .filter(|(_, score)| allowed_scores.contains(score))
        .collect();
    
    // Si aucun mot ne correspond (fallback), on prend tout
    let pool = if filtered_words.is_empty() {
        words.iter().collect()
    } else {
        filtered_words
    };

    let mut rng = rand::rng();
    let (content, difficulty_score) = pool.choose(&mut rng).unwrap_or(&&("CODE", 1));

    SecretWord {
        id: 0, // Sera défini lors de l'attribution
        content: content.to_string(),
        length: content.len() as u32,
        theme: theme.to_string(),
        rarity: Rarity::Common, // TODO: Calculer la rareté
        difficulty_score: *difficulty_score,
    }
}

pub fn get_all_words() -> Vec<String> {
    let themes = vec!["Pays", "Technologie", "Espionnage", "Nature", "Mystère"];
    let mut all_words = Vec::new();
    for theme in themes {
        let words = get_words_for_theme(theme);
        for (word, _) in words {
            all_words.push(word.to_string());
        }
    }
    all_words
}

pub fn validate_word(word: &str) -> bool {
    let upper_word = word.to_uppercase();
    get_all_words().contains(&upper_word)
}
