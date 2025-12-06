use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GameEventType {
    LetterFound,
    LetterNotFound,
    WordGuessed,
    BuzzFailed,
    TurnPassed,
    PowerUsed,
    Error,
    InquiryInitiated,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameEvent {
    pub event_type: GameEventType,
    pub public_log: String,
    pub private_logs: HashMap<u32, String>,
    pub actor_id: u32,
    pub target_id: Option<u32>,
}

impl GameEvent {
    pub fn new(
        event_type: GameEventType, 
        actor_id: u32, 
        target_id: Option<u32>, 
        public_log: String
    ) -> Self {
        GameEvent {
            event_type,
            public_log,
            private_logs: HashMap::new(),
            actor_id,
            target_id,
        }
    }

    pub fn with_private_log(mut self, player_id: u32, message: String) -> Self {
        self.private_logs.insert(player_id, message);
        self
    }

    pub fn get_message_for(&self, player_id: u32) -> String {
        // Si un message privé existe pour ce joueur, on le retourne
        if let Some(msg) = self.private_logs.get(&player_id) {
            return msg.clone();
        }
        // Sinon, on retourne le message public
        self.public_log.clone()
    }
}
