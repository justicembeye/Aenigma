use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum NetworkMessage {
    Join { name: String },
    Welcome { player_id: u32, players: Vec<String> }, // ID assigné, liste des noms
    GameInit(crate::game::Game),
    GameStart,
    Action { 
        player_id: u32, 
        action_type: NetworkActionType,
        target_id: Option<u32>,
        payload: String 
    },
    StateUpdate {
        current_turn: usize,
        logs: Vec<String>,
        // On enverra une version "vue publique" du jeu plus tard
    },
    GameOver { winner_id: u32, winner_name: String },
    Chat { sender: String, content: String },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum NetworkActionType {
    ProposeLetter,
    GuessWord,
    Buzz,
    PassTurn,
}
