use crate::agent::Agent;

// Serde this struct back to the frontend
pub struct Summary {
    king: Agent,
    times_played: u64,
}

impl Summary {
    pub fn new(king: Agent, times_played: u64) -> Self {
        Self { king, times_played }
    }
}
