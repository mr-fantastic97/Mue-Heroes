// backend/src/engine/game.rs

use crate::episode::PayloadMetadata;
use crate::state::pki::PubKey;

#[derive(Default, Clone, Debug)]
pub struct Game {
    pub score: u32,
}

#[derive(Debug, Clone)]
pub enum GameCommand {
    AddPoints { level: u8 },
    WitnessPoints { level: u8 },
}

impl Game {
    pub fn execute(
        &mut self,
        cmd: &GameCommand,
        _auth: Option<PubKey>,
        _metadata: &PayloadMetadata,
    ) -> u32 {
        let delta = match cmd {
            GameCommand::AddPoints { level } => Self::points_for_level(*level),
            GameCommand::WitnessPoints { level } => Self::points_for_level(*level) / 2,
        };
        self.score += delta;
        delta
    }

    pub fn rollback(&mut self, rollback: u32) -> bool {
        if self.score >= rollback {
            self.score -= rollback;
            true
        } else {
            false
        }
    }

    /// Scoring table — higher μ gives more points (no tiers/emojis)
    fn points_for_level(mu: u8) -> u32 {
        match mu {
            15 => 15,
            16 => 25,
            17 => 40,
            18 => 70,
            19 => 120,
            20 => 200,
            21..=u8::MAX => 400,
            _ => 0,
        }
    }
}
