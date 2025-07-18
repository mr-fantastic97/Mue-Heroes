use borsh::{BorshDeserialize, BorshSerialize};
use crate::episode::{Episode, EpisodeError, PayloadMetadata};
use crate::pki::PubKey;

/// 🎮 Game state: tracked per-wallet in the Kdapp engine
#[derive(Default, Clone, BorshSerialize, BorshDeserialize, Debug)]
pub struct Game {
    pub score: u32,
    pub last_update: Option<u64>,
    pub last_mu: Option<u8>,
}

/// Commands a player can issue (currently only adds points via μ-level)
#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
pub enum GameCommand {
    AddPoints { level: u8 },
    WitnessPoints { level: u8 },
}

/// Command-specific errors
#[derive(Debug)]
pub enum GameCommandError {
    InvalidLevel,
}

impl Game {
    /// Maps μ-levels to leaderboard titles (used in display layers)
    pub fn rank_from_level(level: u8) -> &'static str {
        match level {
            15 => "🧭 μScout",
            16..=17 => "🔨 μForged",
            18 => "🦁 μLegend",
            19..=20 => "🧙 μMythic",
            21..=u8::MAX => "🦍 μHonorius",
            _ => "❓ Unknown", // Defensive fallback for unexpected μ-levels
        }
    }
}

impl Episode for Game {
    type Command = GameCommand;
    type CommandError = GameCommandError;
    type CommandRollback = u32;

    /// Initializes a new wallet session
    fn initialize(_participants: Vec<PubKey>, _metadata: &PayloadMetadata) -> Self {
        Game {
            score: 0,
            last_update: None,
        }
    }

    /// Handles command execution (e.g. scoring a mined superblock)
    fn execute(
    &mut self,
    cmd: &Self::Command,
    _auth: Option<PubKey>,
    metadata: &PayloadMetadata,
) -> Result<Self::CommandRollback, EpisodeError<Self::CommandError>> {
    match cmd {
        GameCommand::AddPoints { level } => {
            let points = match level {
                15 => 20,         // μScout
                16..=17 => 45,    // μForged
                18 => 100,        // μLegend
                19..=20 => 250,   // μMythic
                21..=u8::MAX => 500, // μHonorius
                _ => return Err(EpisodeError::CommandError(GameCommandError::InvalidLevel)),
            };

            self.score += points;
            self.last_update = Some(metadata.accepting_time);
            Ok(points)
        },

        GameCommand::WitnessPoints { level } => {
            if *level >= 15 {
                self.score += 5; // μOracle reward
                self.last_update = Some(metadata.accepting_time);
                Ok(5)
            } else {
                Err(EpisodeError::CommandError(GameCommandError::InvalidLevel))
            }
        },
    }
}


    /// Reverts a command (On chain reorg)
    fn rollback(&mut self, rollback: u32) -> bool {
        if self.score >= rollback {
            self.score -= rollback;
            true
        } else {
            false
        }
    }
}
