use std::collections::HashMap;
use serde::{Serialize, Deserialize};

use rand::Rng;


const POINTS_NORMAL_GUESS: u32 = 2;
const POINTS_BUZZ_BONUS: u32 = 1;



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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ControlType {
    Human,
    AI,
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
    pub progress_on_opponents: HashMap<u32, String>,
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
    pub fn new(mut players: Vec<Player>) -> Game {
        let players_clone = players.clone();

        for player in players.iter_mut() {
            let mut progress_map: HashMap<u32, String> = HashMap::new();

            for opponent in &players_clone {
                if player.id != opponent.id {
                    let placeholder = "_".repeat(opponent.secret_word.length as usize);
                    progress_map.insert(opponent.id, placeholder);
                }
            }
            // On assigne le carnet de notes rempli au joueur actuel
            player.progress_on_opponents = progress_map;
        }
        
        Game {
            mode: GameMode::Classic,
            difficulty: Difficulty::Normal,
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





    // Nouvelle méthode pour l'UI : Retourne le résultat au lieu d'imprimer
    pub fn process_letter_proposal(&mut self, current_player_id: u32, target_id: u32, letter: char) -> String {
        let mut result_message = String::new();
        let mut letter_found = false;

        // On cherche le mot secret de l'adversaire
        let opponent_secret_word = match self.players.iter().find(|p| p.id == target_id) {
            Some(p) => p.secret_word.content.clone(),
            None => return "Erreur : Adversaire non trouvé.".to_string(),
        };

        // On trouve le joueur actuel pour pouvoir modifier son HashMap
        if let Some(current_player) = self.players.iter_mut().find(|p| p.id == current_player_id) {
            // On récupère une référence MODIFIABLE à la chaîne de progression
            if let Some(progress_string) = current_player.progress_on_opponents.get_mut(&target_id) {
                
                let secret_chars: Vec<char> = opponent_secret_word.to_uppercase().chars().collect();
                let mut progress_chars: Vec<char> = progress_string.chars().collect();

                // On parcourt le mot secret pour voir si la lettre s'y trouve
                for (i, secret_char) in secret_chars.iter().enumerate() {
                    if *secret_char == letter {
                        progress_chars[i] = letter;
                        letter_found = true;
                    }
                }

                if letter_found {
                    *progress_string = progress_chars.into_iter().collect();
                    current_player.energy = (current_player.energy + 10).min(100);
                    result_message = format!("La lettre '{}' a été trouvée ! (+10% Énergie)", letter);
                } else {
                    result_message = format!("La lettre '{}' n'est pas dans le mot.", letter);
                }
            }
        }
        result_message
    }

    // Ancienne méthode CLI (gardée pour compatibilité si besoin, ou à supprimer)


    // Nouvelle méthode pour l'UI
    pub fn process_word_guess(&mut self, current_player_id: u32, target_id: u32, guessed_word: String, guess_type: GuessType) -> String {
        let mut result_message;
        
        // On cherche l'adversaire cible
        // Note: On doit le faire en deux temps pour éviter les problèmes d'emprunt mutable si on modifie self.players
        let target_player_data = self.players.iter().find(|p| p.id == target_id).map(|p| (p.secret_word.content.clone(), p.name.clone()));

        if let Some((secret_word, target_name)) = target_player_data {
            let is_correct = guessed_word.trim().to_uppercase() == secret_word.to_uppercase();

            if is_correct {
                // C'est GAGNÉ !
                
                // On révèle le mot dans le carnet de TOUS les joueurs (sauf le joueur cible lui-même)
                for p in self.players.iter_mut() {
                    if p.id != target_id {
                        p.progress_on_opponents.insert(target_id, secret_word.clone());
                    }
                }

                // On marque le joueur cible comme éliminé
                if let Some(tp) = self.players.iter_mut().find(|p| p.id == target_id) {
                    tp.is_eliminated = true;
                }

                // Points et Message
                match guess_type {
                    GuessType::Normal => {
                        result_message = format!("Correct ! Vous avez deviné le mot de {}. {} a été éliminé !", target_name, target_name);
                        // TODO: Ajouter des points
                    }
                    GuessType::Buzz => {
                        result_message = format!("Correct ! BUZZ RÉUSSI sur le mot de {}. {} a été éliminé !", target_name, target_name);
                        // TODO: Ajouter des points bonus
                    }
                }
            } else {
                // C'est PERDU
                match guess_type {
                    GuessType::Normal => {
                        result_message = format!("Incorrect. Le mot de {} n'est pas '{}'.", target_name, guessed_word);
                    }
                    GuessType::Buzz => {
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
                                // Récupérer ou initialiser le masque actuel pour le joueur fautif
                                let mask = p.progress_on_opponents.entry(current_player_id).or_insert_with(|| "_".repeat(word_len as usize));
                                
                                // Mettre à jour le masque
                                let mut new_mask_chars: Vec<char> = mask.chars().collect();
                                if reveal_index < new_mask_chars.len() {
                                    new_mask_chars[reveal_index] = revealed_char;
                                }
                                *mask = new_mask_chars.into_iter().collect();
                            }
                        }

                        result_message = format!("Incorrect ! BUZZ RATÉ. Pénalité : Vous révélez le '{}' à tout le monde !", revealed_char);
                        
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
                    result_message.push_str(" (+50% Énergie !)");
                    
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
            result_message = "Erreur : Adversaire non trouvé.".to_string();
        }
        result_message
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
                    // progress est une string comme "_A_ON"
                    for (i, char_progress) in progress.chars().enumerate() {
                        if char_progress != '_' && i < known_chars.len() {
                            known_chars[i] = char_progress;
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