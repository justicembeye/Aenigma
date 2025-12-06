use std::collections::HashMap;
use serde::{Serialize, Deserialize};

use rand::Rng;
use rand::prelude::IndexedRandom;
use crate::game::events::{GameEvent, GameEventType};



const POINTS_NORMAL_GUESS: u32 = 2;
const POINTS_BUZZ_BONUS: u32 = 1;



#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum PowerType {
    Revelation,
    Voyance(char),
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)] // On ajoute Copy car c'est une petite enum
pub enum GuessType {
    Normal,
    Buzz,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GameMode {
    Classic,
    Specialist,
    Chaos,
    FreeCreation,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Difficulty {
    Easy,
    Normal,
    Hard,
    Expert,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PlayerStatus {
    Active,
    Eliminated,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecretWord {
    pub id: u32,
    pub content: String,
    pub theme: String,
    pub rarity: Rarity,
    pub difficulty_score: u32,
    pub length: u32,
}
impl SecretWord {
    #[allow(dead_code)]
    pub fn new(id: u32, content: &str) -> Self {
        SecretWord {
            id,
            content: content.to_ascii_uppercase(),
            theme: String::from("Général"),
            rarity: Rarity::Common,
            difficulty_score: 1,
            length: content.len() as u32,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ControlType {
    Human,
    AI,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Progress {
    pub mask: String,
    pub found_letters: Vec<char>,
    pub missed_letters: Vec<char>, // Lettres essayées mais absentes
    pub length_known: bool,
    pub positions_known: bool,
    pub found_letter_positions: std::collections::HashMap<char, Vec<usize>>,
}

impl Progress {
    pub fn new(length: usize, difficulty: Difficulty) -> Self {
        let (length_known, positions_known) = match difficulty {
            Difficulty::Easy => (true, true),
            Difficulty::Normal | Difficulty::Hard | Difficulty::Expert => (false, false),
        };

        Progress {
            mask: if length_known { "_".repeat(length) } else { "???".to_string() },
            found_letters: Vec::new(),
            missed_letters: Vec::new(),
            length_known,
            positions_known,
            found_letter_positions: std::collections::HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub id: u32,
    pub name: String,
    pub secret_word: SecretWord,
    #[allow(dead_code)]
    pub status: PlayerStatus,
    pub control_type: ControlType,
    //pub actions_history: HashMap<String, u32>,
    pub score: u32,
    pub progress_on_opponents: HashMap<u32, Progress>,
    pub energy: u32, // 0 à 100, permet de Buzzer quand plein
    pub is_eliminated: bool,
    pub turns_played: u32,
    pub correct_guesses: u32,
}
impl Player {
    pub fn new(id: u32, name: &str, secret_word: SecretWord) -> Self {
        Player {
            id,
            name: name.to_string(),
            secret_word,
            status: PlayerStatus::Active,
            control_type: ControlType::Human,
            //actions_history: HashMap::new(),
            score: 0,
            progress_on_opponents: HashMap::new(),
            energy: 0, // Commence à 0
            is_eliminated: false,
            turns_played: 0,
            correct_guesses: 0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Game {
    #[allow(dead_code)]
    pub mode: GameMode,
    #[allow(dead_code)]
    pub difficulty: Difficulty,
    pub players: Vec<Player>,
    pub current_turn: usize,
    pub is_over: bool,
    #[serde(skip, default = "std::time::Instant::now")]
    pub start_time: std::time::Instant,
    pub final_duration: Option<std::time::Duration>,
}
impl Game {
    pub fn new(mut players: Vec<Player>, difficulty: Difficulty) -> Game {
        let players_clone = players.clone();

        for player in players.iter_mut() {
            let mut progress_map: HashMap<u32, Progress> = HashMap::new();

            for opponent in &players_clone {
                if player.id != opponent.id {
                    let progress = Progress::new(opponent.secret_word.length as usize, difficulty);
                    progress_map.insert(opponent.id, progress);
                }
            }
            // On assigne le carnet de notes rempli au joueur actuel
            player.progress_on_opponents = progress_map;
        }
        
        Game {
            mode: GameMode::Classic,
            difficulty,
            players,
            current_turn: 0,
            is_over: false,
            start_time: std::time::Instant::now(),
            final_duration: None,
        }
    }



    pub fn current_player(&self) -> &Player {
        // La magie du modulo pour ne jamais dépasser le nombre de joueurs !
        &self.players[self.current_turn % self.players.len()]
    }

    pub fn next_turn(&mut self) {
        self.current_turn += 1;
        let mut loop_count = 0;
        while self.current_player().is_eliminated && loop_count < self.players.len() {
            self.current_turn += 1;
            loop_count += 1;
        }
        
        // Recharger l'énergie du joueur actif
        let player_index = self.current_turn % self.players.len();
        if let Some(player) = self.players.get_mut(player_index) {
            player.energy = (player.energy + 10).min(100);
        }
    }

    pub fn get_current_round(&self) -> usize {
        (self.current_turn / self.players.len()) + 1
    }





    pub fn get_player_name(&self, player_id: u32) -> String {
        self.players.iter()
            .find(|p| p.id == player_id)
            .map(|p| p.name.clone())
            .unwrap_or_else(|| "Inconnu".to_string())
    }

    // Étape 1 : Initier l'enquête (Le joueur demande "As-tu la lettre X ?")
    pub fn initiate_inquiry(&mut self, current_player_id: u32, target_id: u32, letter: char) -> GameEvent {
        // Vérifier que la cible existe
        let target_name = match self.players.iter().find(|p| p.id == target_id) {
            Some(p) => p.name.clone(),
            None => return GameEvent::new(GameEventType::Error, current_player_id, Some(target_id), "Cible introuvable".to_string()),
        };

        // On ne vérifie PAS encore si la lettre est présente. On envoie juste la demande.
        let public_msg = format!("{} interroge {} sur la lettre '{}'...", self.get_player_name(current_player_id), target_name, letter);
        let private_msg_actor = format!("Vous demandez à {} s'il possède la lettre '{}'. En attente de réponse...", target_name, letter);
        let private_msg_target = format!("{} vous demande : 'Avez-vous la lettre {} dans votre mot ?'", self.get_player_name(current_player_id), letter);

        GameEvent::new(GameEventType::InquiryInitiated, current_player_id, Some(target_id), public_msg)
            .with_private_log(current_player_id, private_msg_actor)
            .with_private_log(target_id, private_msg_target)
    }

    // Étape 2 : Résoudre l'enquête (La cible a répondu)
    // has_letter : La réponse donnée par le joueur (Vrai ou Faux, si on permet le bluff plus tard)
    // Pour l'instant (Mode Immersion), on vérifie la VRAIE présence, mais on structure pour le futur.
    pub fn resolve_inquiry(&mut self, current_player_id: u32, target_id: u32, letter: char, _claimed_has_letter: bool, proof: Vec<usize>) -> GameEvent {
        let mut letter_found = false;
        
        // On cherche le mot secret de l'adversaire (Vérité absolue)
        let (opponent_secret_word, target_name) = match self.players.iter().find(|p| p.id == target_id) {
            Some(p) => (p.secret_word.content.clone(), p.name.clone()),
            None => return GameEvent::new(GameEventType::Error, current_player_id, Some(target_id), "Erreur : Adversaire non trouvé.".to_string()),
        };

        // On trouve le joueur actuel pour mettre à jour ses notes
        if let Some(current_player) = self.players.iter_mut().find(|p| p.id == current_player_id) {
            let current_player_name = current_player.name.clone();
            
            if let Some(progress) = current_player.progress_on_opponents.get_mut(&target_id) {
                let secret_chars: Vec<char> = opponent_secret_word.to_uppercase().chars().collect();
                
                // Vérification de la présence réelle
                for (i, secret_char) in secret_chars.iter().enumerate() {
                    if *secret_char == letter {
                        letter_found = true;
                        if progress.positions_known {
                            let mut mask_chars: Vec<char> = progress.mask.chars().collect();
                            if i < mask_chars.len() {
                                mask_chars[i] = letter;
                                progress.mask = mask_chars.into_iter().collect();
                            }
                        }
                    }
                }

                if letter_found {
                    if !progress.positions_known {
                        if !progress.found_letters.contains(&letter) {
                            progress.found_letters.push(letter);
                            progress.found_letters.sort();
                        }
                        
                        // Enregistrement des positions fournies (Preuve)
                        if !proof.is_empty() {
                            progress.found_letter_positions.insert(letter, proof.clone());
                        }
                    }
                    current_player.energy = (current_player.energy + 10).min(100);
                    
                    let public_msg = format!("{} : J'ai '{}'", target_name, letter);
                    let private_msg_actor = format!("{} a '{}' ! (+10% Énergie)", target_name, letter);
                    let private_msg_target = format!("Moi : J'ai '{}'", letter);

                    return GameEvent::new(GameEventType::LetterFound, current_player_id, Some(target_id), public_msg)
                        .with_private_log(current_player_id, private_msg_actor)
                        .with_private_log(target_id, private_msg_target);
                } else {
                    if !progress.missed_letters.contains(&letter) {
                        progress.missed_letters.push(letter);
                        progress.missed_letters.sort();
                    }
                    
                    current_player.turns_played += 1;

                    let public_msg = format!("{} : Pas de '{}'", target_name, letter);
                    let private_msg_actor = format!("{} n'a pas '{}'.", target_name, letter);
                    let private_msg_target = format!("Moi : Pas de '{}'", letter);

                    return GameEvent::new(GameEventType::LetterNotFound, current_player_id, Some(target_id), public_msg)
                        .with_private_log(current_player_id, private_msg_actor)
                        .with_private_log(target_id, private_msg_target);
                }
            }
        }
        GameEvent::new(GameEventType::Error, current_player_id, Some(target_id), "Erreur interne".to_string())
    }

    // Ancienne méthode CLI (gardée pour compatibilité si besoin, ou à supprimer)


    // Nouvelle méthode pour l'UI
    pub fn process_word_guess(&mut self, current_player_id: u32, target_id: u32, guessed_word: String, guess_type: GuessType) -> GameEvent {
        let event_type;
        let public_msg;
        let mut private_msg_actor;
        let private_msg_target;
        
        let current_player_name = if let Some(p) = self.players.iter().find(|p| p.id == current_player_id) {
            p.name.clone()
        } else {
            "Inconnu".to_string()
        };
        
        // On cherche l'adversaire cible
        // Note: On doit le faire en deux temps pour éviter les problèmes d'emprunt mutable si on modifie self.players
        let target_player_data = self.players.iter().find(|p| p.id == target_id).map(|p| (p.secret_word.content.clone(), p.name.clone()));

        if let Some((secret_word, target_name)) = target_player_data {
            let is_correct = guessed_word.trim().to_uppercase() == secret_word.to_uppercase();

            if is_correct {
                // C'est GAGNÉ !
                event_type = GameEventType::WordGuessed;
                
                // On révèle le mot dans le carnet de TOUS les joueurs (sauf le joueur cible lui-même)
                for p in self.players.iter_mut() {
                    if p.id != target_id {
                        if let Some(progress) = p.progress_on_opponents.get_mut(&target_id) {
                            progress.mask = secret_word.clone();
                            progress.length_known = true;
                            progress.positions_known = true;
                            progress.found_letters.clear(); // Plus besoin de la liste si on a le mot
                        }
                    }
                }

                // On marque le joueur cible comme éliminé
                if let Some(tp) = self.players.iter_mut().find(|p| p.id == target_id) {
                    tp.is_eliminated = true;
                }

                // Points et Message
                match guess_type {
                    GuessType::Normal => {
                        public_msg = format!("{} 💀 {} (Mot trouvé !)", current_player_name, target_name);
                        private_msg_actor = format!("Succès ! {} éliminé.", target_name);
                        private_msg_target = format!("{} a trouvé '{}' ! Éliminé.", current_player_name, secret_word);
                        // TODO: Ajouter des points
                    }
                    GuessType::Buzz => {
                        public_msg = format!("{} ⚡💀 {} (BUZZ !)", current_player_name, target_name);
                        private_msg_actor = format!("BUZZ RÉUSSI ! {} éliminé.", target_name);
                        private_msg_target = format!("{} a BUZZÉ '{}' ! Éliminé.", current_player_name, secret_word);
                        // TODO: Ajouter des points bonus
                    }
                }
            } else {
                // C'est PERDU
                match guess_type {
                    GuessType::Normal => {
                        event_type = GameEventType::LetterNotFound; // Ou WordNotGuessed si on veut distinguer
                        public_msg = format!("{} : Mot raté sur {}", current_player_name, target_name);
                        private_msg_actor = format!("Incorrect. Ce n'est pas '{}'.", guessed_word);
                        private_msg_target = format!("{} a raté le mot '{}'.", current_player_name, guessed_word);
                    }
                    GuessType::Buzz => {
                        event_type = GameEventType::BuzzFailed;
                        // PÉNALITÉ DE BUZZ : Révélation forcée d'une lettre
                        // 1. Récupérer le mot du joueur actuel
                        let (secret_word, word_len) = if let Some(p) = self.players.iter().find(|p| p.id == current_player_id) {
                            (p.secret_word.content.clone(), p.secret_word.length)
                        } else {
                            ("????".to_string(), 4)
                        };

                        // 2. Choisir une position aléatoire
                        let mut rng = rand::rng();
                        let reveal_index = rng.random_range(0..word_len) as usize;
                        let revealed_char = secret_word.chars().nth(reveal_index).unwrap_or('?');

                        // 3. Révéler cette lettre à TOUS les adversaires
                        for p in self.players.iter_mut() {
                            if p.id != current_player_id {
                                let progress = p.progress_on_opponents.entry(current_player_id).or_insert_with(|| Progress::new(word_len as usize, Difficulty::Normal));
                                
                                if !progress.length_known {
                                    progress.mask = "_".repeat(word_len as usize);
                                    progress.length_known = true;
                                    progress.positions_known = true;
                                }

                                let mut new_mask_chars: Vec<char> = progress.mask.chars().collect();
                                if reveal_index < new_mask_chars.len() {
                                    new_mask_chars[reveal_index] = revealed_char;
                                }
                                progress.mask = new_mask_chars.into_iter().collect();
                            }
                        }

                        public_msg = format!("{} ⚡❌ {} (BUZZ RATÉ !)", current_player_name, target_name);
                        private_msg_actor = format!("BUZZ RATÉ ! Ce n'est pas '{}'. Pénalité : '{}' révélé.", guessed_word, revealed_char);
                        private_msg_target = format!("{} a raté son BUZZ ! Il révèle '{}'.", current_player_name, revealed_char);
                        
                        // Reset Energy
                        if let Some(current_player) = self.players.iter_mut().find(|p| p.id == current_player_id) {
                             current_player.energy = 0;
                        }
                    }
                }
            }
            
            // Calcul du bonus de difficulté AVANT d'emprunter self.players en mutable
            let difficulty_bonus = self.players.iter().find(|p| p.id == target_id).map(|p| p.secret_word.difficulty_score).unwrap_or(1);

            // Mise à jour des stats du joueur (Succès ou Échec, ça compte comme un tour)
            if let Some(current_player) = self.players.iter_mut().find(|p| p.id == current_player_id) {
                current_player.turns_played += 1;
                if is_correct {
                    current_player.correct_guesses += 1;
                    
                    // Bonus d'énergie pour l'élimination
                    current_player.energy = (current_player.energy + 50).min(100);
                    private_msg_actor.push_str(" (+50% Énergie !)");
                    
                    // Calcul du score dynamique
                    // Base : 2 points (Normal)
                    // Bonus Difficulté : +1 à +3 selon le mot
                    // Bonus Buzz : +1
                    
                    let points_earned = match guess_type {
                        GuessType::Normal => POINTS_NORMAL_GUESS + difficulty_bonus,
                        GuessType::Buzz => POINTS_NORMAL_GUESS + difficulty_bonus + POINTS_BUZZ_BONUS,
                    };
                    
                    current_player.score += points_earned;
                }
            }
        } else {
            return GameEvent::new(
                GameEventType::Error, 
                current_player_id, 
                Some(target_id), 
                "Erreur : Adversaire non trouvé.".to_string()
            );
        }
        
        GameEvent::new(event_type, current_player_id, Some(target_id), public_msg)
            .with_private_log(current_player_id, private_msg_actor)
            .with_private_log(target_id, private_msg_target)
    }



    pub fn use_power(&mut self, current_player_id: u32, target_id: u32, power_type: PowerType) -> GameEvent {
        let current_player_name = if let Some(p) = self.players.iter().find(|p| p.id == current_player_id) {
            p.name.clone()
        } else {
            "Inconnu".to_string()
        };

        let target_name = if let Some(p) = self.players.iter().find(|p| p.id == target_id) {
            p.name.clone()
        } else {
            "Inconnu".to_string()
        };

        let (cost, power_name) = match power_type {
            PowerType::Revelation => (50, "Révélation"),
            PowerType::Voyance(_) => (50, "Voyance"),
        };

        // 1. Vérifier l'énergie
        if let Some(current_player) = self.players.iter_mut().find(|p| p.id == current_player_id) {
            if current_player.energy < cost {
                 return GameEvent::new(
                    GameEventType::Error, 
                    current_player_id, 
                    Some(target_id), 
                    format!("Pas assez d'énergie ! (Requis: {}%)", cost)
                );
            }
            current_player.energy -= cost;
        }

        // 2. Appliquer l'effet
        let mut private_msg_actor = String::new();
        let mut private_msg_target = String::new();
        let mut public_msg = format!("{} a utilisé le pouvoir {} sur {} !", current_player_name, power_name, target_name);

        // On a besoin du mot secret de la cible
        let (secret_word, word_len) = if let Some(p) = self.players.iter().find(|p| p.id == target_id) {
            (p.secret_word.content.clone(), p.secret_word.length)
        } else {
            return GameEvent::new(GameEventType::Error, current_player_id, Some(target_id), "Cible introuvable".to_string());
        };

        // On modifie la progression du joueur actuel
        if let Some(current_player) = self.players.iter_mut().find(|p| p.id == current_player_id) {
            let progress = current_player.progress_on_opponents.entry(target_id).or_insert_with(|| Progress::new(word_len as usize, Difficulty::Normal));

            match power_type {
                PowerType::Revelation => {
                    // Révèle la longueur (remplace ??? par _ _ _)
                    if !progress.length_known {
                        progress.mask = "_".repeat(word_len as usize);
                        progress.length_known = true;
                        
                        // On réapplique les lettres déjà trouvées
                        for c in &progress.found_letters {
                             if let Some(positions) = progress.found_letter_positions.get(c) {
                                 let mut mask_chars: Vec<char> = progress.mask.chars().collect();
                                 for &pos in positions {
                                     if pos > 0 && pos <= mask_chars.len() {
                                         mask_chars[pos - 1] = *c;
                                     }
                                 }
                                 progress.mask = mask_chars.into_iter().collect();
                             }
                        }

                        private_msg_actor = format!("RÉVÉLATION : Le mot de {} fait {} lettres.", target_name, word_len);
                        private_msg_target = format!("{} a utilisé RÉVÉLATION sur vous ! Il connaît la longueur de votre mot.", current_player_name);
                    } else {
                        private_msg_actor = "RÉVÉLATION : Vous connaissiez déjà la longueur.".to_string();
                    }
                }
                PowerType::Voyance(letter) => {
                    // Vérifie si la lettre est présente
                    let upper_letter = letter.to_ascii_uppercase();
                    let secret_upper = secret_word.to_uppercase();
                    
                    if secret_upper.contains(upper_letter) {
                        // Présente !
                        if !progress.found_letters.contains(&upper_letter) {
                            progress.found_letters.push(upper_letter);
                            progress.found_letters.sort();
                        }
                        
                        // Trouver les positions
                        let mut positions = Vec::new();
                        for (i, c) in secret_upper.chars().enumerate() {
                            if c == upper_letter {
                                positions.push(i + 1);
                            }
                        }
                        progress.found_letter_positions.insert(upper_letter, positions.clone());
                        
                        // Mettre à jour le masque si longueur connue
                        if progress.length_known {
                             let mut mask_chars: Vec<char> = progress.mask.chars().collect();
                             for &pos in &positions {
                                 if pos > 0 && pos <= mask_chars.len() {
                                     mask_chars[pos - 1] = upper_letter;
                                 }
                             }
                             progress.mask = mask_chars.into_iter().collect();
                        }
                        
                        private_msg_actor = format!("VOYANCE : La lettre '{}' est présente !", upper_letter);
                        private_msg_target = format!("{} a utilisé VOYANCE sur la lettre '{}' et l'a trouvée !", current_player_name, upper_letter);
                    } else {
                        // Absente
                        if !progress.missed_letters.contains(&upper_letter) {
                            progress.missed_letters.push(upper_letter);
                            progress.missed_letters.sort();
                        }
                        private_msg_actor = format!("VOYANCE : La lettre '{}' est ABSENTE.", upper_letter);
                        private_msg_target = format!("{} a utilisé VOYANCE sur la lettre '{}' (Absente).", current_player_name, upper_letter);
                    }
                }
            };
        }

        GameEvent::new(GameEventType::PowerUsed, current_player_id, Some(target_id), public_msg)
            .with_private_log(current_player_id, private_msg_actor)
            .with_private_log(target_id, private_msg_target)
    }

    pub fn get_threat_level(&self, player_id: u32) -> String {
        // 1. Récupérer le mot secret du joueur
        let secret_word = match self.players.iter().find(|p| p.id == player_id) {
            Some(p) => &p.secret_word.content,
            None => return "???".to_string(),
        };

        let secret_chars: Vec<char> = secret_word.to_uppercase().chars().collect();
        let mut known_chars = vec!['_'; secret_chars.len()];

        // 2. Parcourir tous les AUTRES joueurs pour voir ce qu'ils ont trouvé
        for opponent in &self.players {
            if opponent.id != player_id {
                if let Some(progress) = opponent.progress_on_opponents.get(&player_id) {
                    // progress est une struct Progress
                    // Si le masque est "???", on ne peut rien déduire pour le niveau de menace (ou on ignore)
                    if progress.length_known {
                        for (i, char_progress) in progress.mask.chars().enumerate() {
                            if char_progress != '_' && i < known_chars.len() {
                                known_chars[i] = char_progress;
                            }
                        }
                    }
                }
            }
        }

        // 3. Retourner la chaîne combinée (ex: "G _ B _ N")
        known_chars.into_iter().collect::<String>()
    }


    pub fn check_for_winner(&mut self) -> Option<u32> {
        let mut active_players_count = 0;
        let mut winner_id: Option<u32> = None;

        // On parcourt tous les joueurs
        for player in &self.players {
            // Si un joueur est actif...
            if !player.is_eliminated {
                // ... on incrémente le compteur...
                active_players_count += 1;
                // ... et on mémorise son ID.
                winner_id = Some(player.id);
            }
        }

        // Après avoir compté, on vérifie le résultat
        if active_players_count == 1 {
            // S'il ne reste qu'un joueur, on retourne son ID
            if self.final_duration.is_none() {
                self.final_duration = Some(self.start_time.elapsed());
                self.is_over = true;
            }
            return winner_id;
        } else {
            // Sinon, il n'y a pas encore de vainqueur
            return None;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::events::GameEventType;

    fn create_test_game() -> Game {
        let p1 = Player::new(1, "Alice", SecretWord::new(1, "POMME"));
        let p2 = Player::new(2, "Bob", SecretWord::new(2, "TARTE"));
        let p3 = Player::new(3, "Charlie", SecretWord::new(3, "CITRON"));
        Game::new(vec![p1, p2, p3], Difficulty::Normal)
    }

    #[test]
    fn test_letter_found_privacy() {
        let mut game = create_test_game();
        
        // Alice (1) cherche 'A' chez Bob (2) -> Trouvé (TARTE)
        let event = game.process_letter_proposal(1, 2, 'A');

        assert_eq!(event.event_type, GameEventType::LetterFound);
        
        // Message pour Alice (Actor)
        let msg_alice = event.get_message_for(1);
        assert!(msg_alice.contains("trouvée"), "Alice should know she found it");
        assert!(msg_alice.contains("Bob"), "Alice should know it's Bob");

        // Message pour Bob (Target)
        let msg_bob = event.get_message_for(2);
        assert!(msg_bob.contains("Alice"), "Bob should know it's Alice");
        assert!(msg_bob.contains("trouvé"), "Bob should know it was found");

        // Message pour Charlie (Observer)
        let msg_charlie = event.get_message_for(3);
        // Le message public doit être générique ou moins précis
        assert!(msg_charlie.contains("Alice"), "Public should know actor");
        assert!(msg_charlie.contains("Bob"), "Public should know target");
        // Mais ne doit PAS contenir le détail "confidentiel" si on voulait le cacher (ici on a choisi de dire "trouvé" publiquement, 
        // mais vérifions que les messages sont bien différents)
        assert_ne!(msg_alice, msg_charlie, "Private message should differ from public");
    }

    #[test]
    fn test_buzz_fail_privacy() {
        let mut game = create_test_game();
        
        // Alice (1) Buzz "POIRE" chez Bob (2) -> Raté (TARTE)
        // Force energy for buzz
        game.players[0].energy = 100;
        
        let event = game.process_word_guess(1, 2, "POIRE".to_string(), crate::game::GuessType::Buzz);

        assert_eq!(event.event_type, GameEventType::BuzzFailed);

        // Message pour Alice
        let msg_alice = event.get_message_for(1);
        assert!(msg_alice.contains("Pénalité"), "Alice should see penalty info");

        // Message pour Bob
        let msg_bob = event.get_message_for(2);
        assert!(msg_bob.contains("raté"), "Bob should know Alice failed");

        // Message Public
        let msg_public = event.public_log;
        assert!(msg_public.contains("révélée"), "Public should know a letter was revealed");
    }
}