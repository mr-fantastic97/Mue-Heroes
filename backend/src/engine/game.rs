use crate::episode::{PayloadMetadata, EpisodeError};
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

#[derive(Debug)]
pub enum GameCommandError {
    InvalidCommand,
}

impl Game {
    pub fn execute(
        &mut self,
        cmd: &GameCommand,
        _auth: Option<PubKey>,
        _metadata: &PayloadMetadata,
    ) -> Result<u32, EpisodeError<GameCommandError>> {
        match cmd {
            GameCommand::AddPoints { level } => {
                self.score += *level as u32;
                Ok(*level as u32)
            }
            GameCommand::WitnessPoints { level } => {
                self.score += (*level as u32) / 2;
                Ok((*level as u32) / 2)
            }
        }
    }

    pub fn rollback(&mut self, rollback: u32) -> bool {
        if self.score >= rollback {
            self.score -= rollback;
            true
        } else {
            false
        }
    }

    /// Tier label based on μ-level and whether the highest μ was mined.
    pub fn rank_from_level(mu_level: u8, is_mined: bool) -> &'static str {
        if !is_mined { return "🧾 μOracle"; }
        match mu_level {
            15 => "🧭 μScout",
            16..=17 => "🔨 μForged",
            18 => "🦁 μLegend",
            19..=20 => "🧙 μMythic",
            21..=u8::MAX => "🦍 μHonorius",
            _ => "❓ Unknown",
        }
    }

    pub fn rank_from_score(score: u32) -> &'static str {
        match score {
            0..=19    => "🧭 μScout",
            20..=64   => "🔨 μForged",
            65..=164  => "🦁 μLegend",
            165..=414 => "🧙 μMythic",
            _         => "🦍 μHonorius",
        }
    }
}
