use crate::game::{Difficulty};
use rand::prelude::IndexedRandom;
use rand::Rng;

pub struct AI {
    #[allow(dead_code)]
    pub difficulty: Difficulty,
    // Mémoire de l'IA (lettres essayées, etc.)
    pub tried_letters: Vec<char>,
}

impl AI {
    pub fn new(difficulty: Difficulty) -> Self {
        AI {
            difficulty,
            tried_letters: Vec::new(),
        }
    }

    pub fn guess_letter(&mut self, _target_pattern: &str) -> char {
        let mut rng = rand::rng();
        
        // Stratégie selon la difficulté
        let use_frequency = match self.difficulty {
            Difficulty::Easy => rng.random_bool(0.4), // 40% de chance d'utiliser la fréquence (très bête)
            Difficulty::Normal => rng.random_bool(0.8), // 80% de chance (humain)
            Difficulty::Hard | Difficulty::Expert => true, // Toujours optimal
        };

        let alphabet: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
        let available: Vec<char> = alphabet.into_iter()
            .filter(|c| !self.tried_letters.contains(c))
            .collect();

        if available.is_empty() {
            return 'A'; // Should not happen
        }

        let final_choice = if use_frequency {
            // Fréquence des lettres en français
            let frequency_order = "ESAITNRULODCPMVQFBGHJXYZWK";
            frequency_order.chars()
                .find(|c| !self.tried_letters.contains(c))
                .unwrap_or_else(|| *available.choose(&mut rng).unwrap())
        } else {
            // Choix totalement aléatoire
            *available.choose(&mut rng).unwrap()
        };
        
        self.tried_letters.push(final_choice);
        final_choice
    }
    

    pub fn decide_action(&mut self, target_pattern: &str) -> AIAction {
        // Gestion du Brouillard de Guerre : Si on ne connaît pas la longueur, on ne peut que proposer des lettres
        if target_pattern.contains('?') {
            let letter = self.guess_letter(target_pattern);
            return AIAction::ProposeLetter(letter);
        }

        // 1. Analyser le pattern (ex: "I_L_S_ON")
        let pattern_len = target_pattern.len();
        let known_chars: Vec<char> = target_pattern.chars().collect();
        
        // 2. Récupérer les candidats possibles dans le dictionnaire
        let all_words = crate::game::dictionary::get_all_words();
        let candidates: Vec<String> = all_words.into_iter()
            .filter(|w| {
                if w.len() != pattern_len { return false; }
                // Vérifier si le mot correspond au pattern connu
                w.chars().zip(known_chars.iter()).all(|(wc, &kc)| {
                    kc == '_' || kc == wc
                })
            })
            .collect();

        // 3. Prise de décision
        if !candidates.is_empty() {
            let mut should_guess = false;
            let mut guess_word = candidates[0].clone();

            match self.difficulty {
                Difficulty::Easy => {
                    // En Facile, l'IA tente de deviner si elle a quelques candidats, même avec peu de certitude
                    // Elle a 30% de chance de tenter un guess si < 5 candidats
                    if candidates.len() < 5 && rand::rng().random_bool(0.3) {
                        should_guess = true;
                        // Elle choisit un mot au hasard parmi les candidats (risque d'erreur)
                        guess_word = candidates.choose(&mut rand::rng()).unwrap().clone();
                    }
                }
                Difficulty::Normal => {
                    // En Normal, l'IA tente si elle a 1 ou 2 candidats
                    if candidates.len() <= 2 {
                        should_guess = true;
                        guess_word = candidates.choose(&mut rand::rng()).unwrap().clone();
                    }
                }
                Difficulty::Hard | Difficulty::Expert => {
                    // En Difficile/Expert, l'IA ne tente que si elle est sûre (1 seul candidat)
                    // ou si le ratio de lettres révélées est très élevé
                    if candidates.len() == 1 {
                        let revealed_count = known_chars.iter().filter(|&&c| c != '_').count();
                        let revealed_ratio = revealed_count as f32 / pattern_len as f32;
                        if revealed_ratio >= 0.6 { // Seuil de confiance
                            should_guess = true;
                            guess_word = candidates[0].clone();
                        }
                    }
                }
            }

            if should_guess {
                return AIAction::GuessWord(guess_word);
            } else if candidates.len() == 1 {
                 // Si on a un seul candidat mais qu'on n'a pas osé deviner (ex: Hard mais ratio faible),
                 // on cherche les lettres manquantes de ce candidat unique
                 let candidate = &candidates[0];
                 for c in candidate.chars() {
                    if !known_chars.contains(&c) && !self.tried_letters.contains(&c) {
                        self.tried_letters.push(c);
                        return AIAction::ProposeLetter(c);
                    }
                }
            }
        }
        
        // Si on a peu de candidats, stratégie avancée pour Hard/Expert
        if !candidates.is_empty() && (matches!(self.difficulty, Difficulty::Hard | Difficulty::Expert)) {
             // On cherche la lettre la plus discriminante
             let mut letter_counts = std::collections::HashMap::new();
             for word in &candidates {
                 for c in word.chars() {
                     if !self.tried_letters.contains(&c) && !known_chars.contains(&c) {
                         *letter_counts.entry(c).or_insert(0) += 1;
                     }
                 }
             }
             
             // On prend la lettre qui apparaît dans le plus de mots candidats (pour éliminer le max de possibilités ou trouver)
             if let Some((best_letter, _)) = letter_counts.iter().max_by_key(|&(_, count)| count) {
                 self.tried_letters.push(*best_letter);
                 return AIAction::ProposeLetter(*best_letter);
             }
        }

        // Fallback : on propose une lettre selon la fréquence globale et la difficulté
        let letter = self.guess_letter(target_pattern);
        AIAction::ProposeLetter(letter)
    }
}

pub enum AIAction {
    ProposeLetter(char),
    GuessWord(String),
}
