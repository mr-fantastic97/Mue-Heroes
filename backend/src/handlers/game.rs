use borsh::{BorshDeserialize, BorshSerialize};
use crate::episode::{Episode, EpisodeError, PayloadMetadata};
use crate::pki::PubKey;

/// 🎮 Game State: Tracks cumulative score + last update timestamp per wallet
#[derive(Default, Clone, BorshSerialize, BorshDeserialize, Debug)]
pub struct Game {
    pub score: u32,
    pub last_update: Option<u64>,
}

/// 🧠 GameCommand: Wraps player actions (currently only `AddPoints`)
#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
pub enum GameCommand {
    AddPoints { level: u8 }, // Submit μ-level of mined superblock
}

/// 🚫 Command errors (e.g., invalid μ-level submitted)
#[derive(Debug)]
pub enum GameCommandError {
    InvalidLevel,
}

/// ✅ Implements Episode logic — each wallet is a self-contained Kdapp instance
impl Episode for Game {
    type Command = GameCommand;
    type CommandError = GameCommandError;
    type CommandRollback = u32; // We rollback by subtracting awarded points

    /// 🆕 Initializes a fresh Game state
    fn initialize(_participants: Vec<PubKey>, _metadata: &PayloadMetadata) -> Self {
        Game {
            score: 0,
            last_update: None,
        }
    }

    /// 🚀 Handles incoming commands and updates state
    fn execute(
        &mut self,
        cmd: &Self::Command,
        _auth: Option<PubKey>,
        metadata: &PayloadMetadata,
    ) -> Result<Self::CommandRollback, EpisodeError<Self::CommandError>> {
        match cmd {
            GameCommand::AddPoints { level } => {
                let points = match level {
                    15 => 10,
                    16 => 25,
                    17 => 50,
                    18 => 100,
                    19..=u8::MAX => 250,
                    _ => return Err(EpisodeError::CommandError(GameCommandError::InvalidLevel)),
                };

                self.score += points;
                self.last_update = Some(metadata.accepting_time);
                Ok(points)
            }
        }
    }

    /// 🔁 Reverts a previous command (e.g., chain reorg rollback)
    fn rollback(&mut self, rollback: u32) -> bool {
        if self.score >= rollback {
            self.score -= rollback;
            true
        } else {
            false
        }
    }
}

