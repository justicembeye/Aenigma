use std::io;

use crossterm::event::{self, Event, KeyCode};
use std::time::{Duration, Instant};
use crate::tui::app::{App, AppState, Focus, Action};
use crate::game::{Difficulty, Player, SecretWord, Rarity, ControlType, Game};
use crate::game::events::GameEventType;
use crate::game::dictionary;
use rand::prelude::IndexedRandom;

pub async fn handle_events(app: &mut App) -> io::Result<()> {
    match app.current_state {
        AppState::Welcome => {
            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char('q') => app.running = false,
                        KeyCode::Up => {
                            let i = app.welcome_list_state.selected().unwrap_or(0);
                            if i > 0 {
                                app.welcome_list_state.select(Some(i - 1));
                            }
                        }
                        KeyCode::Down => {
                            let i = app.welcome_list_state.selected().unwrap_or(0);
                            if i < app.welcome_list_items.len() - 1 {
                                app.welcome_list_state.select(Some(i + 1));
                            }
                        }
                        KeyCode::Enter => {
                            let i = app.welcome_list_state.selected().unwrap_or(0);
                            match i {
                                0 => app.current_state = AppState::Setup, // Nouvelle Partie
                                1 => app.current_state = AppState::MultiplayerMenu, // Multijoueur
                                2 => app.running = false, // Quitter
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        AppState::Setup => {
            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    match app.setup_step {
                        0 => { // Choix du nombre de joueurs (Grille 2x2)
                            let count = app.setup_list_items.len();
                            let cols = 2;
                            let selected = app.setup_list_state.selected().unwrap_or(0);

                            match key.code {
                                KeyCode::Right => {
                                    if selected + 1 < count {
                                        app.setup_list_state.select(Some(selected + 1));
                                    }
                                }
                                KeyCode::Left => {
                                    if selected > 0 {
                                        app.setup_list_state.select(Some(selected - 1));
                                    }
                                }
                                KeyCode::Down => {
                                    if selected + cols < count {
                                        app.setup_list_state.select(Some(selected + cols));
                                    }
                                }
                                KeyCode::Up => {
                                    if selected >= cols {
                                        app.setup_list_state.select(Some(selected - cols));
                                    }
                                }
                                KeyCode::Enter => {
                                    if let Some(selected) = app.setup_list_state.selected() {
                                        // 0: Solo (vs IA) -> 2 joueurs (dont 1 IA)
                                        // 1: Duel -> 2 joueurs
                                        // 2: Triade -> 3 joueurs
                                        // 3: Carré -> 4 joueurs
                                        app.total_players = match selected {
                                            0 => 2, // Solo
                                            1 => 2, // Duel
                                            2 => 3, // Triade
                                            3 => 4, // Carré
                                            _ => 2,
                                        };
                                        app.is_solo = selected == 0;
                                        
                                        // Passage à l'étape suivante (Difficulté)
                                        app.setup_step = 1;
                                        app.setup_list_items = vec![
                                            "Facile".to_string(),
                                            "Normal".to_string(),
                                            "Difficile".to_string(),
                                            "Expert".to_string(),
                                        ];
                                        app.setup_list_state.select(Some(1)); // Normal par défaut
                                    }
                                }
                                KeyCode::Esc | KeyCode::Char('q') => {
                                    app.current_state = AppState::Welcome;
                                }
                                _ => {}
                            }
                        }
                        _ => { // Autres étapes (Liste verticale classique)
                            match key.code {
                                KeyCode::Down => {
                                    let i = match app.setup_list_state.selected() {
                                        Some(i) => {
                                            if i >= app.setup_list_items.len() - 1 {
                                                0
                                            } else {
                                                i + 1
                                            }
                                        }
                                        None => 0,
                                    };
                                    app.setup_list_state.select(Some(i));
                                }
                                KeyCode::Up => {
                                    let i = match app.setup_list_state.selected() {
                                        Some(i) => {
                                            if i == 0 {
                                                app.setup_list_items.len() - 1
                                            } else {
                                                i - 1
                                            }
                                        }
                                        None => 0,
                                    };
                                    app.setup_list_state.select(Some(i));
                                }
                                KeyCode::Enter => {
                                    if let Some(_selected) = app.setup_list_state.selected() {
                                        match app.setup_step {
                                            1 => { // Difficulté -> Lancement
                                                // On sauvegarde la difficulté
                                                if let Some(selected_index) = app.setup_list_state.selected() {
                                                    app.difficulty = match selected_index {
                                                        0 => crate::game::Difficulty::Easy,
                                                        1 => crate::game::Difficulty::Normal,
                                                        2 => crate::game::Difficulty::Hard,
                                                        3 => crate::game::Difficulty::Expert,
                                                        _ => crate::game::Difficulty::Normal,
                                                    };
                                                }

                                                // Initialisation de la configuration des joueurs
                                                app.temp_players.clear();
                                                app.setup_player_index = 0;
                                                
                                                if app.is_solo {
                                                    // Mode Solo : On choisit d'abord la méthode de sélection du thème
                                                    app.current_state = AppState::SetupThemeSelectionMethod;
                                                    app.setup_list_items = vec![
                                                        "Je choisis le thème".to_string(),
                                                        "L'adversaire choisit".to_string(),
                                                        "Aléatoire".to_string(),
                                                    ];
                                                    app.setup_list_state.select(Some(0));
                                                } else {
                                                    // Transition vers la saisie du nom du premier joueur
                                                    app.current_state = AppState::SetupEnterPlayerName;
                                                    app.input.clear();
                                                    app.cursor_position = 0;
                                                }
                                            }
                                            _ => {}
                                        }
                                    }
                                }
                                KeyCode::Esc | KeyCode::Char('q') => {
                                    // Retour à l'étape précédente
                                    if app.setup_step > 0 {
                                        app.setup_step -= 1;
                                        match app.setup_step {
                                            0 => {
                                                app.setup_list_items = vec![
                                                    "🤖  SOLO (vs IA)".to_string(),
                                                    "⚔️  DUEL (2 Joueurs)".to_string(),
                                                    "⚠️  TRIADE (3 Joueurs)".to_string(),
                                                    "🛡️  CARRÉ (4 Joueurs)".to_string(),
                                                ];
                                                let index = if app.total_players <= 2 { 0 } else { app.total_players as usize - 1 };
                                                app.setup_list_state.select(Some(index));
                                            }
                                            _ => {}
                                        }
                                    } else {
                                        app.current_state = AppState::Welcome;
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
        }
        AppState::SelectPower => {
            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Esc => app.current_state = AppState::Playing,
                        KeyCode::Up => {
                            let items_number = app.power_list_items.len();
                            if items_number > 0 {
                                let current_index = app.power_list_state.selected().unwrap_or(0);
                                let new_index = if current_index > 0 {
                                    current_index - 1
                                } else {
                                    items_number - 1
                                };
                                app.power_list_state.select(Some(new_index));
                            }
                        }
                        KeyCode::Down => {
                            let items_number = app.power_list_items.len();
                            if items_number > 0 {
                                let current_index = app.power_list_state.selected().unwrap_or(0);
                                let new_index = if current_index < items_number - 1 {
                                    current_index + 1
                                } else {
                                    0
                                };
                                app.power_list_state.select(Some(new_index));
                            }
                        }
                        KeyCode::Enter => {
                            if let Some(selected) = app.power_list_state.selected() {
                                // 0: Voyance, 1: Révélation
                                // On prépare la liste des cibles
                                app.target_list_items.clear();
                                if let Some(game) = &app.game {
                                    let current_player_id = game.current_player().id;
                                    for player in &game.players {
                                        if player.id != current_player_id && !player.is_eliminated {
                                            app.target_list_items.push((player.id, player.name.clone()));
                                        }
                                    }
                                }
                                
                                if !app.target_list_items.is_empty() {
                                    // On sélectionne la première cible par défaut
                                    app.target_list_state.select(Some(0));
                                    app.current_state = AppState::SelectPowerTarget;
                                } else {
                                    app.notification = Some(("Aucune cible disponible !".to_string(), Instant::now()));
                                    app.current_state = AppState::Playing;
                                }
                            }
                        }


                        _ => {}
                    }
                }
            }
        }

        AppState::SelectPowerTarget => {
            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Esc => app.current_state = AppState::SelectPower,
                        KeyCode::Up => {
                            let items_number = app.target_list_items.len();
                            if items_number > 0 {
                                let current_index = app.target_list_state.selected().unwrap_or(0);
                                let new_index = if current_index > 0 { current_index - 1 } else { items_number - 1 };
                                app.target_list_state.select(Some(new_index));
                            }
                        }
                        KeyCode::Down => {
                            let items_number = app.target_list_items.len();
                            if items_number > 0 {
                                let current_index = app.target_list_state.selected().unwrap_or(0);
                                let new_index = if current_index < items_number - 1 { current_index + 1 } else { 0 };
                                app.target_list_state.select(Some(new_index));
                            }
                        }
                        KeyCode::Enter => {
                            if let Some(selected_target_idx) = app.target_list_state.selected() {
                                if selected_target_idx < app.target_list_items.len() {
                                    app.selected_target_id = Some(app.target_list_items[selected_target_idx].0);
                                    
                                    // Vérifier quel pouvoir est sélectionné
                                    if let Some(power_idx) = app.power_list_state.selected() {
                                        match power_idx {
                                            0 => { // Voyance -> Demander une lettre
                                                app.input.clear();
                                                app.cursor_position = 0;
                                                app.current_state = AppState::SelectPowerLetter;
                                            }
                                            1 => { // Révélation -> Exécuter direct
                                                if let Some(game) = &mut app.game {
                                                    let event = game.use_power(app.my_player_id.unwrap_or(0), app.selected_target_id.unwrap(), crate::game::PowerType::Revelation);
                                                    app.last_action_result = event.public_log.clone();
                                                    app.last_action_success = true;
                                                    app.game_log.push(format!("> {}", event.public_log));
                                                    app.current_state = AppState::TurnResult;
                                                    app.result_timer = Some(Instant::now());
                                                }
                                            }
                                            _ => {}
                                        }
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        AppState::SelectPowerLetter => {
            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Esc => app.current_state = AppState::SelectPowerTarget,
                        KeyCode::Char(c) if c.is_alphabetic() => {
                             if let Some(game) = &mut app.game {
                                 let event = game.use_power(app.my_player_id.unwrap_or(0), app.selected_target_id.unwrap(), crate::game::PowerType::Voyance(c));
                                 app.last_action_result = event.public_log.clone();
                                 app.last_action_success = true;
                                 app.game_log.push(format!("> {}", event.public_log));
                                 app.current_state = AppState::TurnResult;
                                 app.result_timer = Some(Instant::now());
                             }
                        }
                        _ => {}
                    }
                }
            }
        }

        AppState::Playing => {


            // Vérifier si c'est au tour de l'IA
            let mut is_ai_turn = false;
            if let Some(game) = &app.game {
                if let crate::game::ControlType::AI = game.current_player().control_type {
                    is_ai_turn = true;
                }
            }

            if is_ai_turn {
                // Logique IA avec délai
                if app.ai_thinking_timer.is_none() {
                    app.ai_thinking_timer = Some(Instant::now());
                }

                if let Some(timer) = app.ai_thinking_timer {
                    if timer.elapsed() >= Duration::from_millis(2000) { // Délai de réflexion réduit à 2s pour fluidité
                        // Le délai est passé, l'IA joue
                        if let Some(game) = &mut app.game {
                            let current_player_id = game.current_player().id;
                            // Cible : le joueur humain (pour l'instant on suppose 1vs1 Solo)
                            let target_id = game.players.iter().find(|p| p.id != current_player_id).map(|p| p.id).unwrap_or(0);
                            
                            let threat_level = game.get_threat_level(target_id);

                            // L'IA décide de son action
                            let action = if let Some(ai) = &mut app.ai_opponent {
                                ai.decide_action(&threat_level)
                            } else {
                                crate::game::ai::AIAction::ProposeLetter('?') // Fallback
                            };

                            match action {
                                crate::game::ai::AIAction::GuessWord(word) => {
                                     // Gestion du Log Groupé (Local - Mot)
                                    if game.get_current_round() > app.last_log_turn {
                                        app.game_log.push(format!("> [Tour {}]", game.get_current_round()));
                                        app.last_log_turn = game.get_current_round();
                                    }
                                     let event = game.process_word_guess(current_player_id, target_id, word.clone(), crate::game::GuessType::Normal);
                                     
                                     // On récupère le message public pour le log
                                     let log_msg = event.public_log.clone();
                                     app.last_action_result = log_msg.clone();
                                     
                                     if event.event_type == GameEventType::WordGuessed {
                                          app.last_action_success = true;
                                     } else {
                                          app.last_action_success = false;
                                     }
                                     // Gestion du Log Groupé (IA)
                                     if game.get_current_round() > app.last_log_turn {
                                         app.game_log.push(format!("> [Tour {}]", game.get_current_round()));
                                         app.last_log_turn = game.get_current_round();
                                     }
                                     app.game_log.push(format!("IA : {}", log_msg));

                                     // Reset timers
                                     app.ai_thinking_timer = None;
                                     
                                     // Check for Game Over
                                     if let Some(winner_id) = game.check_for_winner() {
                                         app.current_state = AppState::GameOver;
                                         app.last_action_result = format!("VICTOIRE ! {} a gagné !", game.get_player_name(winner_id));
                                     } else {
                                         // Show result
                                         app.current_state = AppState::TurnResult;
                                         app.result_timer = Some(Instant::now());
                                     }
                                }
                                crate::game::ai::AIAction::ProposeLetter(letter) => {
                                    // 1. L'IA initie l'enquête
                                    let event = game.initiate_inquiry(current_player_id, target_id, letter);
                                    let log_msg = event.public_log.clone();
                                    app.last_action_result = log_msg.clone();
                                    
                                    // 2. Si la cible est un HUMAIN LOCAL, on passe en mode "RespondToInquiry"
                                    let target_is_human_local = if let Some(target) = game.players.iter().find(|p| p.id == target_id) {
                                        target.control_type == ControlType::Human
                                    } else {
                                        false
                                    };

                                    if target_is_human_local {
                                        // C'est au joueur humain de répondre !
                                        app.last_action_result = format!("IA 🕵️ Moi : '{}' ?", letter);
                                        // On loggue aussi dans le journal
                                        app.game_log.push(format!("IA : {}", log_msg));
                                        
                                        // On passe en mode interactif
                                        app.current_state = AppState::RespondToInquiry;
                                        app.pending_inquiry = Some((current_player_id, letter));
                                        app.pause_menu_index = 0; // Par défaut sur OUI (ou NON, à voir)
                                        
                                        // On arrête le timer de l'IA, c'est au joueur de jouer
                                        app.ai_thinking_timer = None;
                                        app.result_timer = None; 
                                        
                                        // On ne passe PAS en TurnResult, on attend la réponse du joueur
                                    } else {
                                        // Cible IA vs IA (Simulation)
                                        // On résout immédiatement (ou avec un petit délai si on veut)
                                        // L'IA répond toujours la vérité pour l'instant
                                        // TODO: Faire mentir l'IA selon sa personnalité
                                        // Pour l'instant on passe None pour la position (l'IA ne précise pas encore)
                                        let event = game.resolve_inquiry(current_player_id, target_id, letter, true, vec![]);
                                        let log_msg = event.public_log.clone();
                                        app.last_action_result = log_msg.clone();
                                        app.last_action_success = event.event_type == GameEventType::LetterFound;
                                        
                                        app.game_log.push(format!("IA : {}", log_msg));
                                        
                                        // Fin du tour normale
                                        app.current_state = AppState::TurnResult;
                                        app.result_timer = Some(Instant::now());
                                        app.ai_thinking_timer = None;
                                        
                                        // Vérifier Coup de Grâce
                                        let new_threat_level = game.get_threat_level(target_id);
                                        if !new_threat_level.contains('_') {
                                            let target_secret = game.players.iter().find(|p| p.id == target_id).map(|p| p.secret_word.content.clone()).unwrap_or_default();
                                            let finish_event = game.process_word_guess(current_player_id, target_id, target_secret, crate::game::GuessType::Normal);
                                            app.last_action_result = finish_event.public_log.clone();
                                            app.last_action_success = true;
                                            app.game_log.push(format!("IA 💀 {}", finish_event.public_log));
                                        }
                                    }
                                }
                            }
                        }
                    }
                }



                }

                // Logique Joueur Humain (toujours active si ce n'est pas le tour de l'IA)
                // Logique Joueur Humain
                if event::poll(Duration::from_millis(100))? {
                    if let Event::Key(key) = event::read()? {
                        match key.code {
                            KeyCode::Char('q') => {
                                app.previous_state = Some(AppState::Playing);
                                app.current_state = AppState::ConfirmQuit;
                                app.pause_menu_index = 1; // Par défaut sur "Non"
                            }
                            KeyCode::Esc => app.current_state = AppState::Paused,
                            KeyCode::Tab => {
                                // Vérifier si la grille de pouvoirs est visible
                                let should_show_powers = if let Some(game) = &app.game {
                                    let my_id = app.my_player_id.unwrap_or(1);
                                    if let Some(player) = game.players.iter().find(|p| p.id == my_id) {
                                        player.energy >= 100 && game.difficulty != crate::game::Difficulty::Easy
                                    } else {
                                        false
                                    }
                                } else {
                                    false
                                };

                                app.current_focus = match app.current_focus {
                                    Focus::Actions => {
                                        if should_show_powers {
                                            Focus::Powers
                                        } else {
                                            Focus::Notebook
                                        }
                                    },
                                    Focus::Powers => Focus::Notebook,
                                    Focus::Notebook => Focus::System,
                                    Focus::System => Focus::GameLog,
                                    Focus::GameLog => Focus::Actions,
                                };
                            }
                            KeyCode::Down => {
                                match app.current_focus {
                                    Focus::Actions => {
                                        let items_number = app.playing_list_items.len();
                                        if items_number > 0 {
                                            let current_index = app.playing_list_state.selected().unwrap_or(0);
                                            let new_index = if current_index < items_number - 1 {
                                                current_index + 1
                                            } else {
                                                0
                                            };
                                            app.playing_list_state.select(Some(new_index));
                                        }
                                    }
                                    Focus::System => {
                                        let items_number = app.system_list_items.len();
                                        if items_number > 0 {
                                            let current_index = app.system_list_state.selected().unwrap_or(0);
                                            let new_index = if current_index < items_number - 1 {
                                                current_index + 1
                                            } else {
                                                0
                                            };
                                            app.system_list_state.select(Some(new_index));
                                        }
                                    }
                                    Focus::Powers => {
                                        // Powers grid uses Left/Right, not Up/Down
                                    }
                                    Focus::GameLog => {
                                        // Down ne fait plus rien pour le log
                                    }
                                    Focus::Notebook => {}
                                }
                            }
                            KeyCode::Right => {
                                 if app.current_focus == Focus::System {
                                    let items_number = app.system_list_items.len();
                                    if items_number > 0 {
                                        let current_index = app.system_list_state.selected().unwrap_or(0);
                                        let new_index = if current_index < items_number - 1 {
                                            current_index + 1
                                        } else {
                                            0
                                        };
                                        app.system_list_state.select(Some(new_index));
                                    }
                                 } else if app.current_focus == Focus::Powers {
                                    let items_number = app.power_list_items.len();
                                    if items_number > 0 {
                                        let current_index = app.power_grid_state.selected().unwrap_or(0);
                                        let new_index = if current_index < items_number - 1 {
                                            current_index + 1
                                        } else {
                                            0
                                        };
                                        app.power_grid_state.select(Some(new_index));
                                    }
                                 } else if app.current_focus == Focus::GameLog {
                                    // Right = Tour plus ancien (augmenter offset)
                                    app.log_scroll_offset = app.log_scroll_offset.saturating_add(1);
                                 } else if app.current_focus == Focus::Notebook {
                                    // Carousel Navigation (Next Opponent)
                                    if let Some(game) = &app.game {
                                        let opponents_count = game.players.len().saturating_sub(1);
                                        if opponents_count > 1 {
                                            app.selected_opponent_index = (app.selected_opponent_index + 1) % opponents_count;
                                        }
                                    }
                                 }
                            }
                            KeyCode::Left => {
                                 if app.current_focus == Focus::System {
                                    let items_number = app.system_list_items.len();
                                    if items_number > 0 {
                                        let current_index = app.system_list_state.selected().unwrap_or(0);
                                        let new_index = if current_index > 0 {
                                            current_index - 1
                                        } else {
                                            items_number - 1
                                        };
                                        app.system_list_state.select(Some(new_index));
                                    }
                                 } else if app.current_focus == Focus::Powers {
                                    let items_number = app.power_list_items.len();
                                    if items_number > 0 {
                                        let current_index = app.power_grid_state.selected().unwrap_or(0);
                                        let new_index = if current_index > 0 {
                                            current_index - 1
                                        } else {
                                            items_number - 1
                                        };
                                        app.power_grid_state.select(Some(new_index));
                                    }
                                 } else if app.current_focus == Focus::GameLog {
                                    // Left = Tour plus récent (diminuer offset)
                                    app.log_scroll_offset = app.log_scroll_offset.saturating_sub(1);
                                 } else if app.current_focus == Focus::Notebook {
                                    // Carousel Navigation (Previous Opponent)
                                    if let Some(game) = &app.game {
                                        let opponents_count = game.players.len().saturating_sub(1);
                                        if opponents_count > 1 {
                                            if app.selected_opponent_index == 0 {
                                                app.selected_opponent_index = opponents_count - 1;
                                            } else {
                                                app.selected_opponent_index -= 1;
                                            }
                                        }
                                    }
                                 }
                            }
                            KeyCode::Up => {
                                match app.current_focus {
                                    Focus::Actions => {
                                        let items_number = app.playing_list_items.len();
                                        if items_number > 0 {
                                            let current_index = app.playing_list_state.selected().unwrap_or(0);
                                            let new_index = if current_index > 0 {
                                                current_index - 1
                                            } else {
                                                items_number - 1
                                            };
                                            app.playing_list_state.select(Some(new_index));
                                        }
                                    }
                                    Focus::System => {
                                        let items_number = app.system_list_items.len();
                                        if items_number > 0 {
                                            let current_index = app.system_list_state.selected().unwrap_or(0);
                                            let new_index = if current_index > 0 {
                                                current_index - 1
                                            } else {
                                                items_number - 1
                                            };
                                            app.system_list_state.select(Some(new_index));
                                        }
                                    }
                                    Focus::Powers => {
                                        // Powers grid uses Left/Right, not Up/Down
                                    }
                                    Focus::GameLog => {
                                        // Up ne fait plus rien pour le log
                                    }
                                    Focus::Notebook => {}
                                }
                            }
                            KeyCode::Enter => {
                                match app.current_focus {
                                    Focus::Notebook => {}
                                    Focus::Powers => {
                                        // Transition vers SelectPower avec le pouvoir sélectionné
                                        if let Some(selected_index) = app.power_grid_state.selected() {
                                            app.power_list_state.select(Some(selected_index));
                                            app.current_state = AppState::SelectPower;
                                        }
                                    }
                                    Focus::Actions => {
                                        // Vérification du tour en multijoueur
                                        let mut is_my_turn = true;
                                        if let Some(game) = &app.game {
                                            if let Some(my_id) = app.my_player_id {
                                                if game.current_player().id != my_id {
                                                    is_my_turn = false;
                                                }
                                            }
                                        }

                                        if !is_my_turn {
                                            app.last_action_result = "Ce n'est pas votre tour !".to_string();
                                            app.last_action_success = false;
                                        } else if let Some(selected_index) = app.playing_list_state.selected() {
                                            if selected_index < app.available_actions.len() {
                                                match app.available_actions[selected_index] {
                                                    Action::ProposeLetter => {
                                                        // On peuple la liste des cibles
                                                        app.target_list_items.clear();
                                                        if let Some(game) = &app.game {
                                                            let current_player_id = game.current_player().id;
                                                            for player in &game.players {
                                                                if player.id != current_player_id && !player.is_eliminated {
                                                                    app.target_list_items.push((player.id, player.name.clone()));
                                                                }
                                                            }
                                                        }
                                                        
                                                        // Si une seule cible, on la sélectionne automatiquement
                                                        if app.target_list_items.len() == 1 {
                                                            app.selected_target_id = Some(app.target_list_items[0].0);
                                                            app.current_state = AppState::InputLetter;
                                                            app.input.clear();
                                                            app.cursor_position = 0;
                                                        } else {
                                                            app.current_state = AppState::SelectTarget;
                                                            app.target_list_state.select(Some(0));
                                                        }
                                                    }
                                                    Action::GuessWord => {
                                                        // On peuple la liste des cibles (idem)
                                                        app.target_list_items.clear();
                                                        if let Some(game) = &app.game {
                                                            let current_player_id = game.current_player().id;
                                                            for player in &game.players {
                                                                if player.id != current_player_id && !player.is_eliminated {
                                                                    app.target_list_items.push((player.id, player.name.clone()));
                                                                }
                                                            }
                                                        }
                                                        
                                                        // Si une seule cible, on la sélectionne automatiquement
                                                        if app.target_list_items.len() == 1 {
                                                            app.selected_target_id = Some(app.target_list_items[0].0);
                                                            app.current_state = AppState::InputGuess;
                                                            app.input.clear();
                                                            app.cursor_position = 0;
                                                        } else {
                                                            app.current_state = AppState::SelectTarget;
                                                            app.target_list_state.select(Some(0));
                                                        }
                                                    }
                                                    Action::Buzz => {
                                                        if let Some(game) = &app.game {
                                                            let current_player = game.current_player();
                                                            if current_player.energy < 100 {
                                                                app.game_log.push(format!("⚠️ Buzz non prêt ({}%)", current_player.energy));
                                                                if app.game_log.len() > 10 {
                                                                    app.game_log.remove(0);
                                                                }
                                                            } else {
                                                                // On peuple la liste des cibles (idem)
                                                                app.target_list_items.clear();
                                                                let current_player_id = current_player.id;
                                                                for player in &game.players {
                                                                    if player.id != current_player_id && !player.is_eliminated {
                                                                        app.target_list_items.push((player.id, player.name.clone()));
                                                                    }
                                                                }
                                                                
                                                                // Si une seule cible, on la sélectionne automatiquement
                                                                if app.target_list_items.len() == 1 {
                                                                    app.selected_target_id = Some(app.target_list_items[0].0);
                                                                    app.current_state = AppState::InputGuess;
                                                                    app.input.clear();
                                                                    app.cursor_position = 0;
                                                                } else {
                                                                    app.current_state = AppState::SelectTarget;
                                                                    app.target_list_state.select(Some(0));
                                                                }
                                                            }
                                                        }
                                                    }
                                                    Action::UsePower => {
                                                        app.current_state = AppState::SelectPower;
                                                        app.power_list_state.select(Some(0));
                                                    }
                                                    Action::SkipTurn => {
                                                        if let Some(game) = &mut app.game {
                                                            // Ajouter un marqueur de tour au log AVANT next_turn
                                                            app.game_log.push(format!("> [Tour {}]", game.get_current_round()));
                                                            
                                                            game.next_turn();
                                                            app.update_available_actions(); // Mise à jour des actions
                                                            
                                                            // Passer directement à Playing pour que l'IA joue
                                                            app.current_state = AppState::Playing;
                                                            app.ai_thinking_timer = None;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    Focus::System => {
                                        if let Some(selected_index) = app.system_list_state.selected() {
                                            match selected_index {
                                                0 => { // Pause
                                                    app.current_state = AppState::Paused;
                                                }
                                                1 => { // Quitter
                                                    app.previous_state = Some(AppState::Playing);
                                                    app.current_state = AppState::ConfirmQuit;
                                                    app.pause_menu_index = 1; // Par défaut sur "Non"
                                                }
                                                _ => {}
                                            }

                                         }
                                    }
                                    Focus::GameLog => {
                                        // Pas d'action sur Enter pour le log
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }
        }
        AppState::SelectTarget => {
             if event::poll(Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char('q') => {
                            app.previous_state = Some(AppState::SelectTarget); // Ou Playing ? SelectTarget est un sous-état de Playing
                            app.current_state = AppState::ConfirmQuit;
                            app.pause_menu_index = 1; // Par défaut sur "Non"
                        }
                        KeyCode::Esc => app.current_state = AppState::Playing,
                        KeyCode::Up => {
                            let items_len = app.target_list_items.len();
                            if items_len > 0 {
                                let current = app.target_list_state.selected().unwrap_or(0);
                                // Grid 2 colonnes : Up = -2
                                let next = if current >= 2 { current - 2 } else { current };
                                app.target_list_state.select(Some(next));
                            }
                        }
                        KeyCode::Down => {
                            let items_len = app.target_list_items.len();
                            if items_len > 0 {
                                let current = app.target_list_state.selected().unwrap_or(0);
                                // Grid 2 colonnes : Down = +2
                                let next = if current + 2 < items_len { current + 2 } else { current };
                                app.target_list_state.select(Some(next));
                            }
                        }
                        KeyCode::Left => {
                            let items_len = app.target_list_items.len();
                            if items_len > 0 {
                                let current = app.target_list_state.selected().unwrap_or(0);
                                let next = if current > 0 { current - 1 } else { items_len - 1 };
                                app.target_list_state.select(Some(next));
                            }
                        }
                        KeyCode::Right => {
                            let items_len = app.target_list_items.len();
                            if items_len > 0 {
                                let current = app.target_list_state.selected().unwrap_or(0);
                                let next = if current < items_len - 1 { current + 1 } else { 0 };
                                app.target_list_state.select(Some(next));
                            }
                        }
                        KeyCode::Enter => {
                             if let Some(selected_index) = app.target_list_state.selected() {
                                 if selected_index < app.target_list_items.len() {
                                     let (target_id, _) = app.target_list_items[selected_index];
                                     app.selected_target_id = Some(target_id);
                                     
                                     // Transition vers l'étape suivante selon l'action choisie précédemment
                                     match app.playing_list_state.selected() {
                                         Some(0) => {
                                             app.current_state = AppState::InputLetter;
                                            app.input.clear();
                                            app.cursor_position = 0;
                                             app.input.clear();
                                             app.cursor_position = 0;
                                         }
                                         Some(1) | Some(2) => {
                                             app.current_state = AppState::InputGuess;
                                            app.input.clear();
                                            app.cursor_position = 0;
                                             app.input.clear();
                                             app.cursor_position = 0;
                                         }
                                         _ => app.current_state = AppState::Playing,
                                     }
                                 }
                             }
                        }
                        _ => {}
                    }
                }
             }
        }
        AppState::InputLetter => {
            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char(c) if c.is_alphabetic() => {
                            // On limite à 1 caractère
                            if app.input.len() < 1 {
                                app.input.push(c.to_ascii_uppercase());
                                app.cursor_position += 1;
                            }
                        }
                        KeyCode::Backspace => {
                            if !app.input.is_empty() {
                                app.input.pop();
                                if app.cursor_position > 0 {
                                    app.cursor_position -= 1;
                                }
                            }
                        }
                        KeyCode::Enter => {
                            if app.input.trim().is_empty() {
                                app.input_error = Some("Veuillez entrer une lettre.".to_string());
                            } else {
                                app.input_error = None;
                                // On exécute l'action
                                if let Some(game) = &mut app.game {
                                    let current_player_id = game.current_player().id;
                                    if let Some(target_id) = app.selected_target_id {
                                        let letter = app.input.chars().next().unwrap();

                                        // MODE RÉSEAU
                                        if let Some(client) = &app.network_client {
                                            let action = crate::network::protocol::NetworkMessage::Action {
                                                player_id: current_player_id,
                                                action_type: crate::network::protocol::NetworkActionType::ProposeLetter,
                                                target_id: Some(target_id),
                                                payload: letter.to_string(),
                                            };

                                            // Problème: on est dans une fonction async, mais on doit appeler send.
                                            // Client::send est async.
                                            // On doit cloner le client ou utiliser une ref.
                                            // App est &mut, donc on peut.
                                            let msg = action.clone();
                                            // On ne peut pas await ici facilement si on est dans un match ? Si handle_events est async, si.
                                            client.send(msg).await;
                                            
                                            // On passe en attente (ou TurnResult direct, en attendant le retour réseau)
                                            app.current_state = AppState::TurnResult;
                                            app.last_action_result = "Envoi au serveur...".to_string();
                                            app.result_timer = Some(Instant::now());
                                        } else {
                                            // MODE LOCAL (Code existant)
                                            // 1. On initie l'enquête
                                            let event = game.initiate_inquiry(current_player_id, target_id, letter);
                                            
                                            // 2. Si la cible est un joueur humain LOCAL, on passe en mode "Réponse"
                                            // En solo, le joueur 1 (index 0) est local.
                                            // Si c'est l'IA qui est visée, elle répondra automatiquement (dans la boucle principale)
                                            
                                            let target_is_human_local = if let Some(target) = game.players.iter().find(|p| p.id == target_id) {
                                                target.control_type == ControlType::Human
                                            } else {
                                                false
                                            };

                                            if target_is_human_local {
                                                // C'est à MOI de répondre !
                                                // Mais attendez, si JE suis le joueur courant, je ne peux pas m'interroger moi-même.
                                                // Donc ce cas n'arrive que si je joue en "Hotseat" (plusieurs humains sur le même PC).
                                                // Pour l'instant, assumons Solo vs AI.
                                                // Si je vise l'IA, l'événement est InquiryInitiated.
                                                
                                                app.last_action_result = "En attente de la réponse de l'IA...".to_string();
                                                app.last_action_success = true; // Neutre pour l'instant
                                                
                                                // On ajoute au log
                                                let msg = event.get_message_for(current_player_id);
                                                app.game_log.push(format!("> {}", msg));
                                                
                                                // On passe le tour à l'IA pour qu'elle réponde (simulation)
                                                // On utilise un timer pour simuler la réflexion
                                                app.ai_thinking_timer = Some(Instant::now());
                                                app.current_state = AppState::TurnResult; // On attend
                                                app.result_timer = None; // Pas de timer auto pour fermer, c'est l'IA qui déclenchera la suite
                                                
                                                // On stocke l'enquête en cours pour que l'IA sache quoi faire
                                                app.pending_inquiry = Some((current_player_id, letter));

                                            } else {
                                                // Cible IA : On simule la réponse
                                                app.last_action_result = "Interrogatoire en cours...".to_string();
                                                app.last_action_success = true;
                                                
                                                let msg = event.get_message_for(current_player_id);
                                                app.game_log.push(format!("> {}", msg));

                                                app.ai_thinking_timer = Some(Instant::now());
                                                app.current_state = AppState::TurnResult;
                                                app.result_timer = None; // On attend la réponse de l'IA
                                                app.pending_inquiry = Some((current_player_id, letter));
                                            }
                                        }
                                    }
                                }
                                app.input.clear();
                                app.cursor_position = 0;
                            }
                        }
                        KeyCode::Esc => {
                            app.input.clear();
                            app.cursor_position = 0;
                            app.current_state = AppState::Playing;
                        }
                        _ => {}
                    }
                }
            }
        }
        AppState::InputGuess => {
             if event::poll(Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char(c) => {
                            app.input.insert(app.cursor_position, c);
                            app.cursor_position += 1;
                        }
                        KeyCode::Backspace => {
                            if app.cursor_position > 0 {
                                app.input.remove(app.cursor_position - 1);
                                app.cursor_position -= 1;
                            }
                        }
                        KeyCode::Enter => {
                            if app.input.trim().is_empty() {
                                app.input_error = Some("Veuillez entrer un mot.".to_string());
                            } else {
                                app.input_error = None;
                                if let Some(game) = &mut app.game {
                                    let current_player_id = game.current_player().id;
                                    if let Some(target_id) = app.selected_target_id {
                                        // MODE RÉSEAU
                                        if let Some(client) = &app.network_client {
                                            let action_type = if app.playing_list_state.selected() == Some(2) {
                                                crate::network::protocol::NetworkActionType::Buzz
                                            } else {
                                                crate::network::protocol::NetworkActionType::GuessWord
                                            };

                                            let action = crate::network::protocol::NetworkMessage::Action {
                                                player_id: current_player_id,
                                                action_type,
                                                target_id: Some(target_id),
                                                payload: app.input.clone(),
                                            };
                                            client.send(action).await;
                                            
                                            app.current_state = AppState::TurnResult;
                                            app.last_action_result = "Envoi au serveur...".to_string();
                                            app.result_timer = Some(Instant::now());
                                        } else {
                                            // MODE LOCAL
                                            let guess_type = if app.playing_list_state.selected() == Some(2) {
                                                crate::game::GuessType::Buzz
                                            } else {
                                                crate::game::GuessType::Normal
                                            };

                                            let event = game.process_word_guess(current_player_id, target_id, app.input.clone(), guess_type);
                                            
                                            // On détermine si c'est un succès ou un échec pour la couleur du popup
                                            if event.event_type == GameEventType::WordGuessed {
                                                app.last_action_success = true;
                                            } else {
                                                app.last_action_success = false;
                                            }
                                            
                                            // En local, on est le joueur actif
                                            let msg = event.get_message_for(current_player_id);
                                            app.last_action_result = msg.clone();
                                            
                                            // Gestion du Log Groupé
                                            if game.get_current_round() > app.last_log_turn {
                                                app.game_log.push(format!("> [Tour {}]", game.get_current_round()));
                                                app.last_log_turn = game.get_current_round();
                                            }
                                            app.game_log.push(format!("{}: {}", game.current_player().name, msg));
                                            if app.game_log.len() > 10 {
                                                app.game_log.remove(0);
                                            }
                                        }
                                        
                                        // Fin du tour
                                        app.current_state = AppState::TurnResult;
                                        app.result_timer = Some(Instant::now());
                                    }
                                }
                                app.input.clear();
                                app.cursor_position = 0;
                            }
                        }

                        KeyCode::Esc => {
                            app.input.clear();
                            app.cursor_position = 0;
                            app.current_state = AppState::Playing;
                        }
                        _ => {}
                    }
                }
             }
        }
        AppState::TurnResult => {
            // Auto-dismiss après 2 secondes (plus lisible)
            if let Some(timer) = app.result_timer {
                if timer.elapsed() >= Duration::from_millis(2000) {
                     if let Some(game) = &mut app.game {
                        // Ajouter marqueur de tour AVANT next_turn
                        app.game_log.push(format!("> [Tour {}]", game.get_current_round()));
                        game.next_turn();
                    }
                    app.current_state = AppState::Playing;
                    app.update_available_actions(); // Mise à jour des actions au nouveau tour
                    app.result_timer = None;
                    
                    // On vérifie si le jeu est fini
                    if let Some(game) = &mut app.game {
                        if let Some(_) = game.check_for_winner() {
                            game.final_duration = Some(game.start_time.elapsed());
                            app.current_state = AppState::GameOver;
                        }
                    }
                }
            } else if let Some(timer) = app.ai_thinking_timer {
                // L'IA réfléchit pour répondre à une enquête
                if timer.elapsed() >= Duration::from_millis(2000) {
                     if let Some((attacker_id, letter)) = app.pending_inquiry {
                         if let Some(game) = &mut app.game {
                             // L'IA répond
                             let target_id = game.players.iter().find(|p| p.id != attacker_id).map(|p| p.id).unwrap_or(1);
                             
                             // On vérifie si l'IA a la lettre pour qu'elle réponde la vérité
                             let ai_player = game.players.iter().find(|p| p.id == target_id);
                             let ai_has_letter = ai_player
                                 .map(|p| p.secret_word.content.to_uppercase().contains(letter))
                                 .unwrap_or(false);

                             // Calculer les positions de la lettre si l'IA l'a
                             let mut ai_positions = Vec::new();
                             if ai_has_letter {
                                 if let Some(ai_p) = ai_player {
                                     let secret = ai_p.secret_word.content.to_uppercase();
                                     for (idx, c) in secret.chars().enumerate() {
                                         if c == letter {
                                             ai_positions.push(idx + 1); // Position 1-indexed
                                         }
                                     }
                                 }
                             }

                             // Résolution avec positions
                             let event = game.resolve_inquiry(attacker_id, target_id, letter, ai_has_letter, ai_positions);
                             let log_msg = event.public_log.clone();
                             app.last_action_result = log_msg.clone();
                             app.last_action_success = event.event_type == GameEventType::LetterFound;
                             
                             app.game_log.push(format!("> {}", log_msg));
                             
                             // On active le timer pour fermer le popup
                             app.result_timer = Some(Instant::now());
                             app.ai_thinking_timer = None;
                             app.pending_inquiry = None;
                         }
                     }
                }
            }

            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Enter => {
                            // C'est ici qu'on passe au tour suivant !
                            if let Some(game) = &mut app.game {
                                game.next_turn();
                            }
                            app.current_state = AppState::Playing;
                            app.update_available_actions(); // Mise à jour des actions au nouveau tour
                            app.result_timer = None;
                            // On vérifie si le jeu est fini
                            if let Some(game) = &mut app.game {
                                if let Some(_) = game.check_for_winner() {
                                    game.final_duration = Some(game.start_time.elapsed());
                                    app.current_state = AppState::GameOver;
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        AppState::GameOver => {
             if event::poll(Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Enter | KeyCode::Esc | KeyCode::Char('q') => {
                            // Retour au menu principal
                            app.current_state = AppState::Welcome;
                            app.welcome_list_items = vec![
                                "Créer une partie (Hôte)".to_string(),
                                "Rejoindre une partie (Client)".to_string(),
                                "Quitter".to_string(),
                            ];
                            // On pourrait réinitialiser le jeu ici si nécessaire
                        }
                        _ => {}
                    }
                }
             }
        }
        AppState::Paused => {
            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Up => {
                            if app.pause_menu_index > 0 {
                                app.pause_menu_index -= 1;
                            } else {
                                app.pause_menu_index = 3; // Wrap around
                            }
                        }
                        KeyCode::Down => {
                            if app.pause_menu_index < 3 {
                                app.pause_menu_index += 1;
                            } else {
                                app.pause_menu_index = 0; // Wrap around
                            }
                        }
                        KeyCode::Enter => {
                            match app.pause_menu_index {
                                0 => app.current_state = AppState::Playing, // Reprendre
                                1 => app.current_state = AppState::Rules,   // Aide
                                2 => {
                                    app.current_state = AppState::Welcome; // Menu Principal
                                    app.welcome_list_items = vec![
                                        "Créer une partie (Hôte)".to_string(),
                                        "Rejoindre une partie (Client)".to_string(),
                                        "Quitter".to_string(),
                                    ];
                                }
                                3 => { // Quitter Jeu
                                    app.previous_state = Some(AppState::Paused);
                                    app.current_state = AppState::ConfirmQuit;
                                    app.pause_menu_index = 1; // Par défaut sur "Non"
                                }
                                _ => {}
                            }
                        }
                        KeyCode::Esc => app.current_state = AppState::Playing, // Reprendre
                        _ => {}
                    }
                }
            }
        }
        AppState::ConfirmQuit => {
            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Left | KeyCode::Right => {
                            // Bascule entre 0 (Oui) et 1 (Non)
                            if app.pause_menu_index == 0 {
                                app.pause_menu_index = 1;
                            } else {
                                app.pause_menu_index = 0;
                            }
                        }
                        KeyCode::Enter => {
                            if app.pause_menu_index == 0 {
                                // Oui -> Quitter
                                app.running = false;
                            } else {
                                // Non -> Retour
                                app.current_state = app.previous_state.take().unwrap_or(AppState::Playing);
                            }
                        }
                        KeyCode::Esc => {
                            // Annuler -> Retour
                            app.current_state = app.previous_state.take().unwrap_or(AppState::Playing);
                        }
                        _ => {}
                    }
                }
            }
        }
        AppState::Rules => {
            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Esc | KeyCode::Enter => app.current_state = AppState::Paused,
                        _ => {}
                    }
                }
            }
        }
        AppState::SetupEnterPlayerName => {
            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char(c) => {
                            app.input.push(c);
                            app.cursor_position += 1;
                        }
                        KeyCode::Backspace => {
                            if !app.input.is_empty() {
                                app.input.pop();
                                app.cursor_position -= 1;
                            }
                        }
                        KeyCode::Enter => {
                            if app.input.trim().is_empty() {
                                app.input_error = Some("Veuillez entrer un nom.".to_string());
                            } else {
                                app.input_error = None;
                                // Création temporaire du joueur
                                let secret_word = SecretWord {
                                    id: 0,
                                    content: "????".to_string(),
                                    length: 4,
                                    theme: "Inconnu".to_string(),
                                    rarity: Rarity::Common,
                                    difficulty_score: 1,
                                };
                                let player = Player::new(app.setup_player_index as u32, &app.input, secret_word);
                                app.temp_players.push(player);
                                app.current_setup_name = app.input.clone();
                                
                                // Passage à l'étape suivante
                                if app.is_solo {
                                    // En solo, le thème est déjà choisi, on va directement au mot secret
                                    app.current_state = AppState::SetupEnterSecretWord;
                                } else {
                                    // En multi, on choisit le thème maintenant
                                    app.current_state = AppState::SetupSelectTheme;
                                    app.setup_list_items = dictionary::get_all_themes();
                                    app.setup_list_state.select(Some(0));
                                }
                                app.input.clear();
                                app.cursor_position = 0;
                            }
                        }
                        KeyCode::Esc => {
                            // Retour au setup précédent (Difficulté)
                            app.setup_step = 1;
                            app.current_state = AppState::Setup;
                            app.setup_list_items = vec![
                                "Facile".to_string(),
                                "Normal".to_string(),
                                "Difficile".to_string(),
                                "Expert".to_string(),
                            ];
                            app.setup_list_state.select(Some(1));
                        }
                        _ => {}
                    }
                }
            }
        }
        AppState::SetupThemeSelectionMethod => {
            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Down => {
                            let i = match app.setup_list_state.selected() {
                                Some(i) => {
                                    if i >= app.setup_list_items.len() - 1 {
                                        0
                                    } else {
                                        i + 1
                                    }
                                }
                                None => 0,
                            };
                            app.setup_list_state.select(Some(i));
                        }
                        KeyCode::Up => {
                            let i = match app.setup_list_state.selected() {
                                Some(i) => {
                                    if i == 0 {
                                        app.setup_list_items.len() - 1
                                    } else {
                                        i - 1
                                    }
                                }
                                None => 0,
                            };
                            app.setup_list_state.select(Some(i));
                        }
                        KeyCode::Enter => {
                            if let Some(selected) = app.setup_list_state.selected() {
                                match selected {
                                    0 => { // Je choisis le thème
                                        app.current_state = AppState::SetupSelectTheme;
                                        app.setup_list_items = dictionary::get_all_themes();
                                        app.setup_list_state.select(Some(0));
                                    }
                                    1 | 2 => { // L'adversaire choisit OU Aléatoire
                                        // Choix aléatoire
                                        let themes = dictionary::get_all_themes();
                                        let mut rng = rand::rng();
                                        let selected_theme = themes.choose(&mut rng).unwrap_or(&"Technologie".to_string()).to_string();
                                        app.current_setup_theme = selected_theme.clone();
                                        
                                        // Choix aléatoire du sous-thème
                                        let sub_themes = dictionary::get_sub_themes(&selected_theme);
                                        if !sub_themes.is_empty() {
                                            let selected_sub = sub_themes.choose(&mut rng).cloned();
                                            app.current_setup_sub_theme = selected_sub;
                                        } else {
                                            app.current_setup_sub_theme = None;
                                        }

                                        // Transition vers la saisie du nom (on saute la sélection manuelle)
                                        app.current_state = AppState::SetupEnterPlayerName;
                                        app.input.clear();
                                        app.cursor_position = 0;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        KeyCode::Esc => {
                            // Retour au setup précédent (Difficulté)
                            app.setup_step = 1;
                            app.current_state = AppState::Setup;
                            app.setup_list_items = vec![
                                "Facile".to_string(),
                                "Normal".to_string(),
                                "Difficile".to_string(),
                                "Expert".to_string(),
                            ];
                            app.setup_list_state.select(Some(1));
                        }
                        _ => {}
                    }
                }
            }
        }
        AppState::SetupSelectTheme => {
            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Down => {
                            let i = match app.setup_list_state.selected() {
                                Some(i) => {
                                    if i >= app.setup_list_items.len() - 1 {
                                        0
                                    } else {
                                        i + 1
                                    }
                                }
                                None => 0,
                            };
                            app.setup_list_state.select(Some(i));
                        }
                        KeyCode::Up => {
                            let i = match app.setup_list_state.selected() {
                                Some(i) => {
                                    if i == 0 {
                                        app.setup_list_items.len() - 1
                                    } else {
                                        i - 1
                                    }
                                }
                                None => 0,
                            };
                            app.setup_list_state.select(Some(i));
                        }
                        KeyCode::Enter => {
                            if let Some(selected) = app.setup_list_state.selected() {
                                let themes = dictionary::get_all_themes();
                                if selected < themes.len() {
                                    let selected_theme = themes[selected].to_string();
                                    app.current_setup_theme = selected_theme.clone();
                                    
                                    // Vérifier s'il y a des sous-thèmes
                                    let sub_themes = dictionary::get_sub_themes(&selected_theme);
                                    if !sub_themes.is_empty() {
                                        app.current_state = AppState::SetupSelectSubTheme;
                                        app.setup_list_items = vec!["Général (Tous)".to_string()];
                                        app.setup_list_items.extend(sub_themes);
                                        app.setup_list_state.select(Some(0));
                                    } else {
                                        app.current_setup_sub_theme = None;
                                        // Passage à l'étape suivante
                                        if app.is_solo {
                                            app.current_state = AppState::SetupEnterPlayerName;
                                        } else {
                                            app.current_state = AppState::SetupEnterSecretWord;
                                        }
                                        app.input.clear();
                                        app.cursor_position = 0;
                                    }
                                }
                            }
                        }
                        KeyCode::Esc => {
                            // Retour à la saisie du nom
                            app.temp_players.pop(); // On retire le joueur en cours de création
                            app.current_state = AppState::SetupEnterPlayerName;
                            app.input.clear();
                            app.cursor_position = 0;
                        }
                        _ => {}
                    }
                }
            }
        }
        AppState::SetupSelectSubTheme => {
            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Down => {
                            let i = match app.setup_list_state.selected() {
                                Some(i) => if i >= app.setup_list_items.len() - 1 { 0 } else { i + 1 },
                                None => 0,
                            };
                            app.setup_list_state.select(Some(i));
                        }
                        KeyCode::Up => {
                            let i = match app.setup_list_state.selected() {
                                Some(i) => if i == 0 { app.setup_list_items.len() - 1 } else { i - 1 },
                                None => 0,
                            };
                            app.setup_list_state.select(Some(i));
                        }
                        KeyCode::Enter => {
                            if let Some(selected) = app.setup_list_state.selected() {
                                if selected < app.setup_list_items.len() {
                                    let selected_sub = app.setup_list_items[selected].clone();
                                    app.current_setup_sub_theme = Some(selected_sub);

                                    // Passage à l'étape suivante
                                    if app.is_solo {
                                        app.current_state = AppState::SetupEnterPlayerName;
                                    } else {
                                        app.current_state = AppState::SetupEnterSecretWord;
                                    }
                                    app.input.clear();
                                    app.cursor_position = 0;
                                }
                            }
                        }
                        KeyCode::Esc => {
                            // Retour au choix du thème
                            app.current_state = AppState::SetupSelectTheme;
                            app.setup_list_items = dictionary::get_all_themes();
                            app.setup_list_state.select(Some(0));
                        }
                        _ => {}
                    }
                }
            }
        }
        AppState::SetupEnterSecretWord => {
            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Tab => {
                            // Trigger Autocomplete
                            let suggestions = dictionary::get_suggestions(&app.input, &app.current_setup_theme, app.current_setup_sub_theme.as_deref());
                            if !suggestions.is_empty() {
                                app.suggestions = suggestions;
                                app.show_suggestions = true;
                                app.suggestion_index = 0;
                            } else {
                                app.show_suggestions = false;
                            }
                        }
                        KeyCode::Down if app.show_suggestions => {
                            if app.suggestion_index < app.suggestions.len() - 1 {
                                app.suggestion_index += 1;
                            } else {
                                app.suggestion_index = 0;
                            }
                        }
                        KeyCode::Up if app.show_suggestions => {
                            if app.suggestion_index > 0 {
                                app.suggestion_index -= 1;
                            } else {
                                app.suggestion_index = app.suggestions.len() - 1;
                            }
                        }
                        KeyCode::Esc if app.show_suggestions => {
                            app.show_suggestions = false;
                        }
                        KeyCode::Char(c) if c.is_alphabetic() => {
                            app.input.push(c.to_ascii_uppercase());
                            app.cursor_position += 1;
                            app.show_suggestions = false; // Hide on typing
                        }
                        KeyCode::Backspace => {
                            if !app.input.is_empty() {
                                app.input.pop();
                                app.cursor_position -= 1;
                                app.show_suggestions = false; // Hide on typing
                            }
                        }
                        KeyCode::Enter => {
                            if app.show_suggestions {
                                // Select suggestion
                                if app.suggestion_index < app.suggestions.len() {
                                    app.input = app.suggestions[app.suggestion_index].clone();
                                    app.cursor_position = app.input.len();
                                    app.show_suggestions = false;
                                }
                            } else {
                                let word_len = app.input.trim().len();
                            let (min_len, max_len) = match app.difficulty {
                                Difficulty::Easy => (3, 6),
                                Difficulty::Normal => (4, 8),
                                Difficulty::Hard => (5, 12),
                                Difficulty::Expert => (6, 20),
                            };

                            if word_len < min_len || word_len > max_len {
                                app.input_error = Some(format!("Mode {:?}: Le mot doit faire entre {} et {} lettres.", app.difficulty, min_len, max_len));
                            } else if !dictionary::validate_word(&app.input) {
                                app.input_error = Some("Mot inconnu ou mal orthographié (Limité au dictionnaire du jeu).".to_string());
                            } else {
                                app.input_error = None;
                                // On met à jour le mot secret du joueur actuel
                                // Validation du mot par rapport au thème (basique pour l'instant)
                                if let Some(player) = app.temp_players.last_mut() {
                                    player.secret_word.content = app.input.clone();
                                    player.secret_word.length = app.input.len() as u32;
                                    player.secret_word.theme = app.current_setup_theme.clone();
                                }

                                app.setup_player_index += 1;
                                // Vérification: avons-nous configuré tous les joueurs humains ?
                                let mut done = false;
                                
                                if app.is_solo {
                                    // Mode Solo: on a configuré le joueur 1 (index 0).
                                    // Le joueur 2 est une IA, on le génère automatiquement.
                                    
                                    // Logique du thème de l'IA selon la difficulté
                                    let ai_theme = match app.difficulty {
                                        Difficulty::Easy => {
                                            // En Facile, l'IA prend le même thème que le joueur
                                            if let Some(p1) = app.temp_players.first() {
                                                p1.secret_word.theme.clone()
                                            } else {
                                                "Technologie".to_string()
                                            }
                                        }
                                        _ => {
                                            // En Normal/Difficile, l'IA choisit un thème aléatoire
                                            let themes = dictionary::get_all_themes();
                                            let mut rng = rand::rng();
                                            themes.choose(&mut rng).unwrap_or(&"Technologie".to_string()).to_string()
                                        }
                                    };
                                    
                                    // Choix du sous-thème IA (aléatoire ou "Tout")
                                    let ai_sub_theme = {
                                        let subs = dictionary::get_sub_themes(&ai_theme);
                                        if !subs.is_empty() {
                                            let mut rng = rand::rng();
                                            Some(subs.choose(&mut rng).unwrap().clone())
                                        } else {
                                            None
                                        }
                                    };

                                    let secret_word = dictionary::get_random_word(&ai_theme, ai_sub_theme.as_deref());
                                    
                                    let mut ai_player = Player::new(1, "IA (Ordinateur)", secret_word);
                                    ai_player.control_type = ControlType::AI;
                                    
                                    app.temp_players.push(ai_player);
                                    
                                    // Initialisation de l'IA persistante avec la difficulté choisie
                                    app.ai_opponent = Some(crate::game::ai::AI::new(app.difficulty.clone()));
                                    
                                    done = true;
                                } else {  // Mode Multijoueur
                                    if app.setup_player_index >= app.total_players as usize {
                                        done = true;
                                    }
                                }

                                if done {
                                    // Lancement du jeu
                                    let new_game = Game::new(app.temp_players.clone(), app.difficulty);
                                    
                                    // Si on est en réseau (Host), on envoie l'état initial
                                    if let Some(client) = &app.network_client {
                                        if app.is_host {
                                            let msg = crate::network::protocol::NetworkMessage::GameInit(new_game.clone());
                                            client.send(msg).await;
                                        }
                                    }

                                    app.game = Some(new_game);
                                    app.current_state = AppState::Playing;
                                    app.update_available_actions(); // Initialisation des actions
                                    app.input.clear();
                                    app.cursor_position = 0;
                                    app.start_time = Some(Instant::now());
                                    
                                    // En local, le joueur est toujours l'ID 0
                                    if app.my_player_id.is_none() {
                                        app.my_player_id = Some(0);
                                    }
                                } else {
                                    // Au tour du joueur suivant
                                    app.current_state = AppState::SetupEnterPlayerName;
                                    app.input.clear();
                                    app.cursor_position = 0;
                                }
                            }
                        }
                    }
                        KeyCode::Esc => {
                            // Retour à la saisie du nom (on annule le joueur en cours)
                            app.temp_players.pop();
                            app.current_state = AppState::SetupEnterPlayerName;
                            app.input.clear(); 
                            app.cursor_position = 0;
                        }
                        _ => {}
                    }
                }
            }
        }
        AppState::MultiplayerMenu => {
            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Up => {
                            let i = match app.welcome_list_state.selected() {
                                Some(i) => if i == 0 { app.welcome_list_items.len() - 1 } else { i - 1 },
                                None => 0,
                            };
                            app.welcome_list_state.select(Some(i));
                        }
                        KeyCode::Down => {
                            let i = match app.welcome_list_state.selected() {
                                Some(i) => if i >= app.welcome_list_items.len() - 1 { 0 } else { i + 1 },
                                None => 0,
                            };
                            app.welcome_list_state.select(Some(i));
                        }
                        KeyCode::Enter => {
                             match app.welcome_list_state.selected() {
                                Some(0) => {
                                    // Héberger
                                    app.is_host = true;
                                    
                                    // 1. Lancer le serveur en arrière-plan
                                    let server = crate::network::server::Server::new(8080);
                                    tokio::spawn(async move {
                                        if let Err(e) = server.run().await {
                                            eprintln!("Erreur serveur: {}", e);
                                        }
                                    });

                                    // 2. Se connecter en tant que client (attendre un peu que le serveur démarre)
                                    tokio::time::sleep(Duration::from_millis(100)).await;
                                    match crate::network::client::Client::connect("127.0.0.1:8080".to_string()).await {
                                        Ok(client) => {
                                            app.network_client = Some(client);
                                            app.current_state = AppState::MultiplayerNameInput; // On demande le nom
                                            app.input.clear();
                                            app.cursor_position = 0;
                                        }
                                        Err(e) => {
                                            app.error_message = Some(format!("Erreur connexion: {}", e));
                                        }
                                    }
                                }
                                Some(1) => {
                                    // Rejoindre
                                    app.is_host = false;
                                    app.current_state = AppState::JoinGame;
                                    app.input.clear();
                                    app.cursor_position = 0;
                                }
                                Some(2) => {
                                    // Retour
                                    app.current_state = AppState::Welcome;
                                    app.welcome_list_items = vec![
                                        "Créer une partie (Hôte)".to_string(),
                                        "Rejoindre une partie (Client)".to_string(),
                                        "Quitter".to_string(),
                                    ];
                                    app.welcome_list_state.select(Some(0));
                                }
                                _ => {}
                            }
                        }
                        KeyCode::Esc => {
                            app.current_state = AppState::Welcome;
                             app.welcome_list_items = vec![
                                "Créer une partie (Hôte)".to_string(),
                                "Rejoindre une partie (Client)".to_string(),
                                "Quitter".to_string(),
                            ];
                            app.welcome_list_state.select(Some(0));
                        }
                        _ => {}
                    }
                }
            }
        }
        AppState::JoinGame => {
             if event::poll(Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char(c) => {
                            app.input.insert(app.cursor_position, c);
                            app.cursor_position += 1;
                        }
                        KeyCode::Backspace => {
                            if app.cursor_position > 0 {
                                app.input.remove(app.cursor_position - 1);
                                app.cursor_position -= 1;
                            }
                        }
                        KeyCode::Left => {
                            if app.cursor_position > 0 {
                                app.cursor_position -= 1;
                            }
                        }
                        KeyCode::Right => {
                            if app.cursor_position < app.input.len() {
                                app.cursor_position += 1;
                            }
                        }
                        KeyCode::Enter => {
                            let addr = app.input.clone();
                            match crate::network::client::Client::connect(addr).await {
                                Ok(client) => {
                                    app.network_client = Some(client);
                                    app.is_host = false;
                                    // TODO: Attendre que l'hôte lance la partie ou configure
                                    // Pour l'instant on va au setup mais en mode "Client" (restreint)
                                    app.current_state = AppState::MultiplayerNameInput; 
                                    app.input.clear();
                                    app.cursor_position = 0; 
                                }
                                Err(e) => {
                                    app.input_error = Some(format!("Erreur: {}", e));
                                }
                            }
                        }
                        KeyCode::Esc => {
                            app.current_state = AppState::MultiplayerMenu;
                        }
                        _ => {}
                    }
                }
            }
        }
        AppState::MultiplayerNameInput => {
            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char(c) => {
                            app.input.insert(app.cursor_position, c);
                            app.cursor_position += 1;
                        }
                        KeyCode::Backspace => {
                            if app.cursor_position > 0 {
                                app.input.remove(app.cursor_position - 1);
                                app.cursor_position -= 1;
                            }
                        }
                        KeyCode::Enter => {
                            if !app.input.trim().is_empty() {
                                app.my_name = app.input.trim().to_string();
                                
                                // Envoyer le message Join
                                if let Some(client) = &app.network_client {
                                    let msg = crate::network::protocol::NetworkMessage::Join { name: app.my_name.clone() };
                                    client.send(msg).await;
                                }

                                if app.is_host {
                                    app.current_state = AppState::Setup;
                                } else {
                                    app.current_state = AppState::Lobby;
                                }
                                app.input.clear();
                                app.cursor_position = 0;
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        AppState::Lobby => {
            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Esc => {
                            // Quitter le lobby
                            app.network_client = None;
                            app.current_state = AppState::MultiplayerMenu;
                        }
                        _ => {}
                    }
                }
            }
        }
        AppState::RespondToInquiry => {
            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Left | KeyCode::Right => {
                            // Bascule entre 0 (Oui) et 1 (Non)
                            app.pause_menu_index = 1 - app.pause_menu_index;
                        }
                        KeyCode::Enter => {
                            // Validation de la réponse
                            if let Some((attacker_id, letter)) = app.pending_inquiry {
                                let claimed_has_letter = app.pause_menu_index == 0; // 0 = OUI
                                
                                if claimed_has_letter {
                                    // Si on dit OUI, on doit préciser la position
                                    app.current_state = AppState::InputPosition;
                                    app.input.clear();
                                    app.cursor_position = 0;
                                } else {
                                    // Si on dit NON, on résout tout de suite
                                    if let Some(game) = &mut app.game {
                                        let event = game.resolve_inquiry(attacker_id, app.my_player_id.unwrap_or(0), letter, false, vec![]);
                                        
                                        app.last_action_success = event.event_type == GameEventType::LetterFound;
                                        let msg = event.get_message_for(app.my_player_id.unwrap_or(0));
                                        app.last_action_result = msg.clone();
                                        app.game_log.push(format!("> {}", msg));
                                        
                                        app.pending_inquiry = None;
                                        app.current_state = AppState::Playing;
                                        game.next_turn();
                                        app.update_available_actions();
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        AppState::InputPosition => {
            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char(c) if c.is_digit(10) || c == ' ' || c == ',' => {
                            app.input.push(c);
                            app.cursor_position += 1;
                        }
                        KeyCode::Backspace => {
                            if !app.input.is_empty() {
                                app.input.pop();
                                app.cursor_position -= 1;
                            }
                        }
                        KeyCode::Enter => {
                            // Parsing des positions multiples (ex: "1, 3")
                            let parts: Vec<&str> = app.input.split([',', ' '])
                                .filter(|s| !s.trim().is_empty())
                                .collect();
                            
                            let mut positions = Vec::new();
                            let mut valid = true;
                            
                            for part in parts {
                                if let Ok(pos) = part.trim().parse::<usize>() {
                                    positions.push(pos);
                                } else {
                                    valid = false;
                                    break;
                                }
                            }

                            if valid && !positions.is_empty() {
                                if let Some((attacker_id, letter)) = app.pending_inquiry {
                                    if let Some(game) = &mut app.game {
                                        let event = game.resolve_inquiry(attacker_id, app.my_player_id.unwrap_or(0), letter, true, positions);
                                        
                                        app.last_action_success = event.event_type == GameEventType::LetterFound;
                                        let msg = event.get_message_for(app.my_player_id.unwrap_or(0));
                                        app.last_action_result = msg.clone();
                                        app.game_log.push(format!("> {}", msg));
                                        
                                        app.pending_inquiry = None;
                                        app.current_state = AppState::Playing;
                                        game.next_turn();
                                        app.update_available_actions();
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    Ok(())
}

pub async fn handle_network_events(app: &mut App) -> io::Result<()> {
    // On sort le client temporairement pour éviter les soucis d'emprunt
    let mut client = if let Some(c) = app.network_client.take() {
        c
    } else {
        return Ok(());
    };

    while let Some(msg) = client.try_recv() {
        match msg {
            crate::network::protocol::NetworkMessage::Join { name } => {
                if app.is_host {
                    let new_id = app.connected_players.len() as u32;
                    app.connected_players.push((new_id, name.clone()));
                    
                    let msg = crate::network::protocol::NetworkMessage::Welcome { 
                        new_player_id: new_id, 
                        new_player_name: name.clone(),
                        all_players: app.connected_players.clone() 
                    };
                    client.send(msg).await;
                    app.game_log.push(format!("[Hôte] {} a rejoint (ID: {})", name, new_id));
                }
            }
            crate::network::protocol::NetworkMessage::Welcome { new_player_id, new_player_name, all_players } => {
                app.connected_players = all_players;
                app.game_log.push(format!("[Réseau] {} a rejoint la partie !", new_player_name));
                
                if new_player_name == app.my_name {
                    app.my_player_id = Some(new_player_id);
                    app.game_log.push(format!("Vous êtes connecté avec l'ID {}", new_player_id));
                }
            }
            crate::network::protocol::NetworkMessage::GameInit(game) => {
                app.game = Some(game);
                app.current_state = AppState::Playing;
                app.start_time = Some(Instant::now()); // On reset le timer localement
                app.game_log.push("[Réseau] La partie commence !".to_string());
            }
            crate::network::protocol::NetworkMessage::GameOver { winner_id: _winner_id, winner_name } => {
                if let Some(game) = &mut app.game {
                    game.is_over = true;
                    if game.final_duration.is_none() {
                        game.final_duration = Some(game.start_time.elapsed());
                    }
                }
                app.current_state = AppState::GameOver;
                app.game_log.push(format!("[Réseau] PARTIE TERMINÉE ! Vainqueur : {}", winner_name));
            }
            crate::network::protocol::NetworkMessage::Action { player_id, action_type, target_id, payload } => {
                // On applique l'action
                let mut winner_found = None;
                
                if let Some(game) = &mut app.game {
                    // Gestion du Log Groupé (Réseau)
                    if game.get_current_round() > app.last_log_turn {
                        app.game_log.push(format!("> [Tour {}]", game.get_current_round()));
                        app.last_log_turn = game.get_current_round();
                    }

                    match action_type {
                        crate::network::protocol::NetworkActionType::ProposeLetter => {
                            if let Some(tid) = target_id {
                                let letter = payload.chars().next().unwrap_or(' ');
                                // Pour le réseau, on résout immédiatement (pas d'interaction pour l'instant)
                                // On considère que le réseau a validé (ou on fait confiance)
                                // TODO: Supporter la position dans le protocole réseau
                                let event = game.resolve_inquiry(player_id, tid, letter, true, vec![]);
                                
                                // Filtrage du message selon qui je suis
                                let msg = if let Some(my_id) = app.my_player_id {
                                    event.get_message_for(my_id)
                                } else {
                                    event.public_log.clone()
                                };
                                
                                app.game_log.push(format!("[Réseau] {}", msg));
                            }
                        }
                        crate::network::protocol::NetworkActionType::GuessWord => {
                            if let Some(tid) = target_id {
                                let event = game.process_word_guess(player_id, tid, payload, crate::game::GuessType::Normal);
                                
                                // Filtrage du message selon qui je suis
                                let msg = if let Some(my_id) = app.my_player_id {
                                    event.get_message_for(my_id)
                                } else {
                                    event.public_log.clone()
                                };

                                app.game_log.push(format!("[Réseau] {}", msg));
                            }
                        }
                        crate::network::protocol::NetworkActionType::Buzz => {
                            if let Some(tid) = target_id {
                                let event = game.process_word_guess(player_id, tid, payload, crate::game::GuessType::Buzz);
                                
                                // Filtrage du message selon qui je suis
                                let msg = if let Some(my_id) = app.my_player_id {
                                    event.get_message_for(my_id)
                                } else {
                                    event.public_log.clone()
                                };

                                app.game_log.push(format!("[Réseau] {}", msg));
                            }
                        }
                        crate::network::protocol::NetworkActionType::PassTurn => {
                            game.next_turn();
                            app.game_log.push(format!("[Réseau] Un joueur a passé son tour."));
                        }
                        crate::network::protocol::NetworkActionType::UsePower => {
                             app.game_log.push(format!("[Réseau] Pouvoir utilisé."));
                        }
                    }
                    
                    // Vérification de la victoire (Seulement si Host)
                    if app.is_host {
                        if let Some(wid) = game.check_for_winner() {
                            let wname = game.players.iter().find(|p| p.id == wid).map(|p| p.name.clone()).unwrap_or("Inconnu".to_string());
                            winner_found = Some((wid, wname));
                        }
                    }
                }

                // Si l'hôte a détecté un vainqueur, on envoie le message GameOver
                if let Some((wid, wname)) = winner_found {
                    let msg = crate::network::protocol::NetworkMessage::GameOver { winner_id: wid, winner_name: wname };
                    client.send(msg).await;
                }
            }
            _ => {}
        }
    }

    // On remet le client
    app.network_client = Some(client);
    Ok(())
}