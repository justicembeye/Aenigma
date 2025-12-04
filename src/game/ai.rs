use crate::game::{Difficulty};
use rand::prelude::IndexedRandom;

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
        // Fréquence des lettres en français (plus fréquent au moins fréquent)
        let frequency_order = "ESAITNRULODCPMVQFBGHJXYZWK";
        
        // On cherche la première lettre de la liste de fréquence qui n'a pas encore été essayée
        let choice = frequency_order.chars()
            .find(|c| !self.tried_letters.contains(c));

        // Si toutes les lettres fréquentes sont prises (cas rare/fin de partie), on prend au hasard parmi ce qui reste
        let final_choice = if let Some(c) = choice {
            c
        } else {
             let alphabet: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
             let available: Vec<char> = alphabet.into_iter()
                .filter(|c| !self.tried_letters.contains(c))
                .collect();
             
             let mut rng = rand::rng();
             *available.choose(&mut rng).unwrap_or(&'A')
        };
        
        self.tried_letters.push(final_choice);
        final_choice
    }
    

    pub fn decide_action(&mut self, target_pattern: &str) -> AIAction {
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
        if candidates.len() == 1 {
            // Un seul candidat : on vérifie si on a assez d'infos pour ne pas frustrer le joueur
            let revealed_count = known_chars.iter().filter(|&&c| c != '_').count();
            let min_revealed = (pattern_len as f32 / 2.0).ceil() as usize; // Au moins la moitié
            
            // Si on n'a pas assez de lettres (et que le mot fait plus de 2 lettres), on fait semblant de chercher
            if revealed_count < min_revealed && pattern_len > 2 {
                let candidate = &candidates[0];
                // On cherche une lettre manquante
                for c in candidate.chars() {
                    if !known_chars.contains(&c) && !self.tried_letters.contains(&c) {
                        self.tried_letters.push(c);
                        return AIAction::ProposeLetter(c);
                    }
                }
            }

            // Sinon, on devine le mot !
            return AIAction::GuessWord(candidates[0].clone());
        }
        
        // Si on a peu de candidats (ex: 2 ou 3), on pourrait tenter de deviner au hasard si on est joueur,
        // mais pour l'instant on reste prudent : on cherche la meilleure lettre pour discriminer.
        
        if !candidates.is_empty() {
             // On cherche la lettre la plus fréquente parmi les candidats (qui n'est pas encore connue)
             let mut letter_counts = std::collections::HashMap::new();
             for word in &candidates {
                 for c in word.chars() {
                     if !self.tried_letters.contains(&c) && !known_chars.contains(&c) {
                         *letter_counts.entry(c).or_insert(0) += 1;
                     }
                 }
             }
             
             // On prend la lettre qui apparaît dans le plus de mots candidats
             if let Some((best_letter, _)) = letter_counts.iter().max_by_key(|&(_, count)| count) {
                 self.tried_letters.push(*best_letter);
                 return AIAction::ProposeLetter(*best_letter);
             }
        }

        // Fallback : on propose une lettre selon la fréquence globale
        let letter = self.guess_letter(target_pattern);
        AIAction::ProposeLetter(letter)
    }
}

pub enum AIAction {
    ProposeLetter(char),
    GuessWord(String),
}
