use ratatui::widgets::ListState;
use crate::game::Player;
use std::time::Instant;

#[derive(PartialEq)]
pub enum Focus {
    Actions,
    System,
}

// Nos différents états d'application possibles
#[derive(PartialEq)]

pub enum AppState {
    Welcome,
    Setup,
    Playing,
    GameOver,
    SetupEnterPlayerName,
    SetupSelectTheme,
    SetupEnterSecretWord,
    // États de jeu interactifs
    SelectTarget,
    InputLetter,
    InputGuess,
    TurnResult, // Pour afficher le résultat de l'action
    Paused, // Nouvel état
    MultiplayerMenu,
    JoinGame,
    Lobby,
    Rules,
}

pub struct App {
    pub running: bool,
    pub current_state: AppState,
    // Welcome Menu
    pub welcome_list_state: ListState,
    pub welcome_list_items: Vec<String>,
    // Setup Menu
    pub setup_list_state: ListState,
    pub setup_list_items: Vec<String>,
    pub setup_step: usize, // 0: Joueurs, 1: Difficulté (Thème déplacé)
    pub input: String,
    pub cursor_position: usize,
    pub total_players: usize,
    pub current_setup_name: String,
    #[allow(dead_code)]
    pub current_setup_secret: String,
    pub current_setup_theme: String,
    pub players: Vec<Player>,
    pub game: Option<crate::game::Game>, // On stocke l'instance du jeu
    pub error_message: Option<String>,
    // Menu de jeu
    pub playing_list_state: ListState,
    pub playing_list_items: Vec<String>,
    // Focus et System Menu
    pub current_focus: Focus,
    pub system_list_state: ListState,
    pub is_solo: bool,
    pub setup_player_index: usize,
    pub temp_players: Vec<crate::game::Player>,
    pub system_list_items: Vec<String>,
    // États temporaires pour les actions
    pub target_list_state: ListState,
    pub target_list_items: Vec<(u32, String)>, // (id, name)
    pub selected_target_id: Option<u32>,
    pub pause_menu_index: usize, // Nouvel index pour le menu pause
    pub start_time: Option<Instant>, // Timer de jeu
    pub last_action_result: String,
    pub last_action_success: bool,
    pub input_error: Option<String>,
    pub game_log: Vec<String>,
    
    // AI Persistence
    pub ai_opponent: Option<crate::game::ai::AI>,
    pub difficulty: crate::game::Difficulty,

    // Network
    pub network_client: Option<crate::network::client::Client>,
    pub is_host: bool,
    pub my_player_id: Option<u32>,
}
impl App {
    pub fn new() -> App {
        let mut setup_state = ListState::default();
        setup_state.select(Some(0));

        let mut welcome_state = ListState::default();
        welcome_state.select(Some(0));

        let mut playing_state = ListState::default();
        playing_state.select(Some(0));

        let mut system_state = ListState::default();
        system_state.select(Some(0));

        let mut target_state = ListState::default();
        target_state.select(Some(0));

        App {
            running: true,
            current_state: AppState::Welcome,
            welcome_list_state: welcome_state,
            welcome_list_items: vec![
                "Créer une partie (Hôte)".to_string(),
                "Rejoindre une partie (Client)".to_string(),
                "Quitter".to_string(),
            ],
            setup_list_state: setup_state,
            setup_list_items: vec![
                "🤖  SOLO (vs IA)".to_string(),
                "⚔️  DUEL (2 Joueurs)".to_string(),
                "⚠️  TRIADE (3 Joueurs)".to_string(),
                "🛡️  CARRÉ (4 Joueurs)".to_string(),
            ],
            setup_step: 0,
            is_solo: false,
            setup_player_index: 0,
            temp_players: Vec::new(),
            input: String::new(),
            cursor_position: 0,
            total_players: 0,
            current_setup_name: String::new(),
            players: Vec::new(),
            current_setup_secret: String::new(),
            current_setup_theme: String::new(),


            game: None,
            error_message: None,
            playing_list_state: playing_state,
            playing_list_items: vec![
                "Proposer une lettre".to_string(),
                "Deviner un mot".to_string(),
                "Buzzer !".to_string(),
                "Passer son tour".to_string(),
            ],
            current_focus: Focus::Actions,
            system_list_state: system_state,
            system_list_items: vec![
                "Reprendre".to_string(),
                "Règles".to_string(),
                "Quitter".to_string(),
            ],
            target_list_state: target_state,
            target_list_items: Vec::new(),
            selected_target_id: None,
            pause_menu_index: 0,
            start_time: None,
            last_action_result: String::new(),
            last_action_success: false,
            input_error: None,
            game_log: Vec::new(),
            ai_opponent: None,
            difficulty: crate::game::Difficulty::Normal,
            network_client: None,
            is_host: false,
            my_player_id: None,
        }
    }
}
