use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChatRole { User, Assistant, System }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatSession {
    pub id: String,
    pub history: Vec<(ChatRole, String)>,
}

impl ChatSession {
    pub fn new() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            history: Vec::new(),
        }
    }
}

// === AXON_COMPAT: ChatSession::push ===
impl ChatSession {
    /// Compatibility: old code expects ChatSession::push(role, msg)
    pub fn push(&mut self, role: ChatRole, msg: String) {
        self.history.push((role, msg));
    }
}


