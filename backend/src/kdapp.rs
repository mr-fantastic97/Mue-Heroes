use crate::episode::{Episode, EpisodeError, PayloadMetadata};
use crate::game::{Game, GameCommand, GameCommandError}; // import Game engine
use crate::pki::PubKey;
use crate::types::SuperblockEvent; // Your custom μ-level event type


/// Kdapp state per wallet — wraps Game logic internally
#[derive(Default, Clone)]
pub struct MueHeroSession {
    game: Game, // 🧩 internally stores the scoring engine
}

/// Error wrapper to align with the Episode trait
#[derive(Debug)]
pub enum MueError {
    Game(GameCommandError),
}


/// Implements Episode for MueHeroSession (1 wallet = 1 game session)
impl Episode for MueHeroSession {
    type Command = SuperblockEvent;
    type CommandError = MueError;
    type CommandRollback = u32;

    /// New game session when wallet mines its first μ-level block
    fn initialize(_participants: Vec<PubKey>, _metadata: &PayloadMetadata) -> Self {
        Self {
            game: Game::default(),
        }
    }

    /// Routes the SuperblockEvent into the Game logic
    fn execute(
        &mut self,
        cmd: &Self::Command,            // e.g. SuperblockEvent { mu_level: 17 }
        _auth: Option<PubKey>,
        metadata: &PayloadMetadata,
    ) -> Result<Self::CommandRollback, EpisodeError<Self::CommandError>> {
        // Convert raw μ-level into GameCommand
        let game_cmd = GameCommand::AddPoints { level: cmd.mu_level };

        // Forward to inner game logic
        self.game
            .execute(&game_cmd, _auth, metadata)
            .map_err(|e| match e {
                EpisodeError::CommandError(inner) => EpisodeError::CommandError(MueError::Game(inner)),
                EpisodeError::InternalError(msg) => EpisodeError::InternalError(msg),
            })
    }


    /// Reverts a score if the μ-level block was reorged
    fn rollback(&mut self, rollback: u32) -> bool {
        self.game.rollback(rollback)
    }
}


impl MueHeroSession {
    pub fn get_score(&self) -> u32 {
        self.game.score
    }
}