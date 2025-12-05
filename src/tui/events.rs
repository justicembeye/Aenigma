use std::io;

use crossterm::event::{self, Event, KeyCode};
use std::time::{Duration, Instant};
use crate::tui::app::{App, AppState, Focus};
use crate::game::{Difficulty, Player, SecretWord, Rarity, ControlType, Game};
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
                                                
                                                // Transition vers la saisie du nom du premier joueur
                                                app.current_state = AppState::SetupEnterPlayerName;
                                                app.input.clear();
                                                app.cursor_position = 0;
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

        AppState::Playing => {
            // Vérifier si c'est au tour de l'IA
            let mut is_ai_turn = false;
            if let Some(game) = &app.game {
                if let crate::game::ControlType::AI = game.current_player().control_type {
                    is_ai_turn = true;
                }
            }

            if is_ai_turn {
                // Logique IA
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
                             let result = game.process_word_guess(current_player_id, target_id, word.clone(), crate::game::GuessType::Normal);
                             app.last_action_result = result.clone();
                             
                             let lower_result = result.to_lowercase();
                             if lower_result.contains("gagné") || lower_result.contains("éliminé") || lower_result.contains("correct") {
                                  app.last_action_success = true;
                             } else {
                                  app.last_action_success = false;
                             }
                             app.game_log.push(format!("[Tour {}] IA (Devine): {}", game.get_current_round(), result));
                        }
                        crate::game::ai::AIAction::ProposeLetter(letter) => {
                            let result = game.process_letter_proposal(current_player_id, target_id, letter);
                            app.last_action_result = result.clone();
                            
                            let lower_result = result.to_lowercase();
                            if (lower_result.contains("correct") && !lower_result.contains("incorrect")) || lower_result.contains("trouvé") || lower_result.contains("gagné") || lower_result.contains("éliminé") || lower_result.contains("présente") {
                                app.last_action_success = true;
                            } else {
                                app.last_action_success = false;
                            }
                            app.game_log.push(format!("[Tour {}] IA: {}", game.get_current_round(), result));

                            // Vérifier si le mot est entièrement découvert (Coup de Grâce pour l'IA)
                            let new_threat_level = game.get_threat_level(target_id);
                            if !new_threat_level.contains('_') {
                                // L'IA enchaîne immédiatement avec l'élimination
                                let target_secret = game.players.iter().find(|p| p.id == target_id).map(|p| p.secret_word.content.clone()).unwrap_or_default();
                                let finish_result = game.process_word_guess(current_player_id, target_id, target_secret, crate::game::GuessType::Normal);
                                
                                app.last_action_result = finish_result.clone(); // On affiche le résultat final
                                app.last_action_success = true;
                                app.game_log.push(format!("[Tour {}] IA (FINISH HIM): {}", game.get_current_round(), finish_result));
                            }
                        }
                    }
                    
                    app.current_state = AppState::TurnResult;
                }
            } else {
                // Logique Joueur Humain
                if event::poll(Duration::from_millis(100))? {
                    if let Event::Key(key) = event::read()? {
                        match key.code {
                            KeyCode::Char('q') => app.running = false,
                            KeyCode::Esc => app.current_state = AppState::Paused,
                            KeyCode::Tab => {
                                app.current_focus = match app.current_focus {
                                    Focus::Actions => Focus::System,
                                    Focus::System => Focus::Actions,
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
                                }
                            }
                            KeyCode::Enter => {
                                match app.current_focus {
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
                                            match selected_index {
                                                0 => { // Proposer une lettre
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
                                                1 => { // Deviner un mot
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
                                                2 => { // Buzzer
                                                    if let Some(game) = &app.game {
                                                        let current_player = game.current_player();
                                                        if current_player.energy < 100 {
                                                            app.game_log.push(format!("⚠️ Buzz non chargé ! ({}%)", current_player.energy));
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
                                                3 => { // Passer son tour
                                                    if let Some(game) = &mut app.game {
                                                        game.next_turn();
                                                        app.last_action_success = true;
                                                        app.last_action_result = "Vous avez passé votre tour.".to_string();
                                                        app.game_log.push(format!("[Tour {}] {}: {}", game.get_current_round(), game.current_player().name, app.last_action_result));
                                                        if app.game_log.len() > 10 {
                                                            app.game_log.remove(0);
                                                        }
                                                        app.current_state = AppState::TurnResult;
                                                    }
                                                }
                                                _ => {}
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
                                                    app.running = false;
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
        }
        AppState::SelectTarget => {
             if event::poll(Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char('q') => app.running = false,
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
                                        } else {
                                            // MODE LOCAL (Code existant)
                                            let result = game.process_letter_proposal(current_player_id, target_id, letter);
                                            
                                            // On détermine si c'est un succès ou un échec pour la couleur du popup
                                            let lower_result = result.to_lowercase();
                                            if (lower_result.contains("correct") && !lower_result.contains("incorrect")) || lower_result.contains("trouvé") || lower_result.contains("gagné") || lower_result.contains("éliminé") {
                                                app.last_action_success = true;
                                            } else {
                                                app.last_action_success = false;
                                            }
                                            app.last_action_result = result.clone();
                                            app.game_log.push(format!("[Tour {}] {}: {}", game.get_current_round(), game.current_player().name, result));
                                            if app.game_log.len() > 10 {
                                                app.game_log.remove(0);
                                            }
                                            
                                            // Vérifier si le mot est entièrement découvert (Coup de Grâce)
                                            let threat_level = game.get_threat_level(target_id);
                                            if !threat_level.contains('_') {
                                                // COUP DE GRÂCE !
                                                app.last_action_success = true;
                                                app.game_log.push(format!("[Tour {}] {}: {} -> FINISH HIM !", game.get_current_round(), game.current_player().name, result));
                                                if app.game_log.len() > 10 {
                                                    app.game_log.remove(0);
                                                }
                                                
                                                // On passe directement en mode "Deviner" pour l'élimination
                                                app.current_state = AppState::InputGuess;
                                                app.input.clear();
                                                app.cursor_position = 0;
                                                // app.selected_target_id est déjà set
                                                return Ok(()); // On sort pour ne pas reset l'état en bas
                                            } else {
                                                // Fin du tour normale
                                                app.current_state = AppState::TurnResult;
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
                                        } else {
                                            // MODE LOCAL
                                            let guess_type = if app.playing_list_state.selected() == Some(2) {
                                                crate::game::GuessType::Buzz
                                            } else {
                                                crate::game::GuessType::Normal
                                            };

                                            let result = game.process_word_guess(current_player_id, target_id, app.input.clone(), guess_type);
                                            
                                            // On détermine si c'est un succès ou un échec pour la couleur du popup
                                            // On cherche des mots clés positifs
                                            let lower_result = result.to_lowercase();
                                            if (lower_result.contains("correct") && !lower_result.contains("incorrect")) || lower_result.contains("trouvé") || lower_result.contains("gagné") || lower_result.contains("éliminé") {
                                                app.last_action_success = true;
                                            } else {
                                                app.last_action_success = false;
                                            }
                                            app.last_action_result = result.clone();
                                            app.game_log.push(format!("[Tour {}] {}: {}", game.get_current_round(), game.current_player().name, result));
                                            if app.game_log.len() > 10 {
                                                app.game_log.remove(0);
                                            }
                                        }
                                        
                                        // Fin du tour
                                        app.current_state = AppState::TurnResult;
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
            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Enter => {
                            // C'est ici qu'on passe au tour suivant !
                            if let Some(game) = &mut app.game {
                                game.next_turn();
                            }
                            app.current_state = AppState::Playing;
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
                                3 => app.running = false, // Quitter Jeu
                                _ => {}
                            }
                        }
                        KeyCode::Esc => app.current_state = AppState::Playing, // Reprendre
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
                                
                                // Passage à la sélection du thème
                                app.current_state = AppState::SetupSelectTheme;
                                app.setup_list_items = dictionary::get_themes(app.difficulty).iter().map(|s| s.to_string()).collect();
                                app.setup_list_state.select(Some(0));
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
                                let themes = dictionary::get_themes(app.difficulty);
                                if selected < themes.len() {
                                    let selected_theme = themes[selected].to_string();
                                    app.current_setup_theme = selected_theme.clone();
                                    
                                    // Passage à la saisie du mot secret
                                    app.current_state = AppState::SetupEnterSecretWord;
                                    app.input.clear();
                                    app.cursor_position = 0;
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
        AppState::SetupEnterSecretWord => {
            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char(c) if c.is_alphabetic() => {
                            app.input.push(c.to_ascii_uppercase());
                            app.cursor_position += 1;
                        }
                        KeyCode::Backspace => {
                            if !app.input.is_empty() {
                                app.input.pop();
                                app.cursor_position -= 1;
                            }
                        }
                        KeyCode::Enter => {
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
                                            // En Normal/Difficile, l'IA choisit un thème aléatoire PARMI CEUX DISPONIBLES
                                            let themes = dictionary::get_themes(app.difficulty);
                                            let mut rng = rand::rng();
                                            themes.choose(&mut rng).unwrap_or(&"Technologie").to_string()
                                        }
                                    };
                                    
                                    let secret_word = dictionary::get_random_word(&ai_theme, app.difficulty);
                                    
                                    let mut ai_player = Player::new(2, "IA (Ordinateur)", secret_word);
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
                                    let new_game = Game::new(app.temp_players.clone());
                                    
                                    // Si on est en réseau (Host), on envoie l'état initial
                                    if let Some(client) = &app.network_client {
                                        if app.is_host {
                                            let msg = crate::network::protocol::NetworkMessage::GameInit(new_game.clone());
                                            client.send(msg).await;
                                        }
                                    }

                                    app.game = Some(new_game);
                                    app.current_state = AppState::Playing;
                                    app.input.clear();
                                    app.cursor_position = 0;
                                    app.start_time = Some(Instant::now());
                                } else {
                                    // Au tour du joueur suivant
                                    app.current_state = AppState::SetupEnterPlayerName;
                                    app.input.clear();
                                    app.cursor_position = 0;
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
                                            app.current_state = AppState::Setup; // On va au setup
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
                                    app.current_state = AppState::Lobby; 
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
            crate::network::protocol::NetworkMessage::Welcome { player_id, players: _ } => {
                app.my_player_id = Some(player_id);
                app.game_log.push(format!("[Réseau] Connecté avec l'ID {}", player_id));
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
                    match action_type {
                        crate::network::protocol::NetworkActionType::ProposeLetter => {
                            if let Some(tid) = target_id {
                                let letter = payload.chars().next().unwrap_or(' ');
                                let result = game.process_letter_proposal(player_id, tid, letter);
                                app.game_log.push(format!("[Réseau] {}", result));
                            }
                        }
                        crate::network::protocol::NetworkActionType::GuessWord => {
                            if let Some(tid) = target_id {
                                let result = game.process_word_guess(player_id, tid, payload, crate::game::GuessType::Normal);
                                app.game_log.push(format!("[Réseau] {}", result));
                            }
                        }
                        crate::network::protocol::NetworkActionType::Buzz => {
                            if let Some(tid) = target_id {
                                let result = game.process_word_guess(player_id, tid, payload, crate::game::GuessType::Buzz);
                                app.game_log.push(format!("[Réseau] {}", result));
                            }
                        }
                        crate::network::protocol::NetworkActionType::PassTurn => {
                            game.next_turn();
                            app.game_log.push(format!("[Réseau] Un joueur a passé son tour."));
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