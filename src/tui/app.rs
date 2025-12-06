use ratatui::widgets::ListState;
use crate::game::Player;
use std::time::Instant;

#[derive(PartialEq)]
pub enum Focus {
    Actions,
    System,
    Powers,
    GameLog,
    Notebook,
}

// Nos différents états d'application possibles
#[derive(PartialEq)]

pub enum AppState {
    Welcome,
    Setup,
    Playing,
    GameOver,
    SetupEnterPlayerName,
    SetupThemeSelectionMethod, // Nouvelle étape : Choix de la méthode de sélection du thème
    SetupSelectTheme,
    SetupSelectSubTheme, // Nouvelle étape : Choix du sous-thème
    SetupEnterSecretWord,
    // États de jeu interactifs
    SelectTarget,
    InputLetter,
    InputGuess,
    TurnResult, // Pour afficher le résultat de l'action
    Paused, // Nouvel état

    ConfirmQuit, // Confirmation de sortie
    SelectPower, // Menu des pouvoirs
    MultiplayerMenu,
    MultiplayerNameInput,
    JoinGame,
    Lobby,
    Rules,
    RespondToInquiry, // État interactif : On me demande si j'ai une lettre
    InputPosition, // Nouvelle étape : Saisir la position de la lettre
    SelectPowerTarget, // Choisir la cible du pouvoir
    SelectPowerLetter, // Choisir la lettre (pour Voyance)
}

#[derive(Clone, PartialEq, Debug)]
pub enum Action {
    ProposeLetter,
    GuessWord,
    Buzz,
    UsePower,
    SkipTurn,
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
    pub current_setup_sub_theme: Option<String>, // Sous-thème sélectionné
    pub players: Vec<Player>,
    pub game: Option<crate::game::Game>, // On stocke l'instance du jeu
    pub error_message: Option<String>,
    // Menu de jeu
    pub playing_list_state: ListState,
    pub playing_list_items: Vec<String>,
    pub available_actions: Vec<Action>, // Actions dynamiques
    pub notification: Option<(String, Instant)>, // Message, Timestamp
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
    pub my_name: String, // Mon nom
    pub connected_players: Vec<(u32, String)>, // ID, Nom
    pub pause_menu_index: usize, // Nouvel index pour le menu pause
    pub start_time: Option<Instant>, // Timer de jeu
    pub result_timer: Option<Instant>, // Timer pour l'affichage du résultat
    pub ai_thinking_timer: Option<Instant>, // Timer pour la réflexion de l'IA
    pub last_log_turn: usize, // Dernier tour affiché dans le log
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

    pub previous_state: Option<AppState>,
    
    pub power_list_state: ListState,
    pub power_list_items: Vec<String>,
    pub power_grid_state: ListState,

    // Interactive Inquiry
    pub pending_inquiry: Option<(u32, char)>, // (attacker_id, letter)

    // Autocomplete
    pub suggestions: Vec<String>,
    pub suggestion_index: usize,
    pub show_suggestions: bool,

    // Game Log Navigation
    pub log_scroll_offset: usize,

    // Detective Notebook Navigation
    pub selected_opponent_index: usize,

    // UI Animation
    pub time_frame: usize,
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

        let mut power_state = ListState::default();
        power_state.select(Some(0));

        let mut app = App {
            running: true,
            current_state: AppState::Welcome,
            // Welcome Menu
            welcome_list_state: welcome_state,
            welcome_list_items: vec![
                "Créer une partie (Hôte)".to_string(),
                "Rejoindre une partie (Client)".to_string(),
                "Quitter".to_string(),
            ],
            // Setup Menu
            setup_list_state: setup_state,
            setup_list_items: vec![
                "🤖  SOLO (vs IA)".to_string(),
                "⚔️  DUEL (2 Joueurs)".to_string(),
                "⚠️  TRIADE (3 Joueurs)".to_string(),
                "🛡️  CARRÉ (4 Joueurs)".to_string(),
            ],
            setup_step: 0,
            
            // Menu de jeu
            playing_list_state: playing_state,
            playing_list_items: Vec::new(), // Sera rempli par update_available_actions
            available_actions: Vec::new(),
            notification: None,
            // Focus et System Menu
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
            my_name: String::new(),
            connected_players: Vec::new(),
            pause_menu_index: 0,
            start_time: None,
            result_timer: None,
            ai_thinking_timer: None,
            last_log_turn: 0,
            last_action_result: String::new(),
            last_action_success: false,
            input_error: None,
            game_log: Vec::new(),
            
            is_solo: false,
            setup_player_index: 0,
            temp_players: Vec::new(),
            input: String::new(),
            cursor_position: 0,
            total_players: 0,
            current_setup_name: String::new(),
            current_setup_secret: String::new(),
            current_setup_theme: String::new(),
            current_setup_sub_theme: None,
            players: Vec::new(),
            game: None,
            error_message: None,
            
            // AI Persistence
            ai_opponent: None,
            difficulty: crate::game::Difficulty::Normal,

            // Network
            network_client: None,
            is_host: false,
            my_player_id: None,

            previous_state: None,
            
            power_list_state: power_state,
            power_list_items: vec![
                "🔮 Voyance".to_string(),
                "📏 Révélation".to_string(),
            ],
            power_grid_state: {
                let mut state = ListState::default();
                state.select(Some(0));
                state
            },
            pending_inquiry: None,
            
            suggestions: Vec::new(),
            suggestion_index: 0,
            show_suggestions: false,

            log_scroll_offset: 0,
            selected_opponent_index: 0,
            time_frame: 0,
        };
        app.update_available_actions();
        app
    }

    pub fn update_available_actions(&mut self) {
        let mut actions = vec![
            Action::ProposeLetter,
            Action::GuessWord,
            Action::Buzz, // Toujours affiché, désactivé si énergie < 100%
        ];

        // On ne retire plus Buzz de la liste, il est toujours là
        let mut energy = 0;
        if let Some(game) = &self.game {
            // En solo, on regarde le joueur 0 (Humain)
            // En multi, on regarderait my_player_id
            if let Some(player) = game.players.iter().find(|p| p.id == self.my_player_id.unwrap_or(1)) {
                energy = player.energy;
            }
        }

        // Notification si Buzz devient disponible (100% énergie)
        if energy >= 100 && !self.available_actions.contains(&Action::Buzz) {
            self.notification = Some(("⚡ BUZZER PRÊT !".to_string(), Instant::now()));
        }

        // NOTE: Pouvoirs ne sont plus dans le menu Actions
        // Ils apparaissent dans une grille séparée dans la zone d'interaction

        actions.push(Action::SkipTurn);

        self.available_actions = actions;
        self.playing_list_items = self.available_actions.iter().map(|a| match a {
            Action::ProposeLetter => "Proposer une lettre".to_string(),
            Action::GuessWord => "Deviner un mot".to_string(),
            Action::Buzz => "⚡ Buzz".to_string(),
            Action::UsePower => "Pouvoirs Spéciaux".to_string(), // Garde le mapping pour compatibilité
            Action::SkipTurn => "Passer son tour".to_string(),
        }).collect();
    }
}
