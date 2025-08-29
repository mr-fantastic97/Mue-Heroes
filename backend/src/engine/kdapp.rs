// backend/src/engine/kdapp.rs

use crate::episode::{Episode, EpisodeError, PayloadMetadata};
use crate::engine::game::{Game, GameCommand, GameCommandError};
use crate::state::pki::PubKey;
use crate::state::types::SuperblockEvent;
use crate::engine::merkle::{verify_merkle_proof, compute_leaf_from_wallet};

#[derive(Default, Clone)]
pub struct MueHeroSession {
    game: Game,
}

#[derive(Debug)]
pub enum MueError {
    Game(GameCommandError),
}

impl Episode for MueHeroSession {
    type Command = SuperblockEvent;
    type CommandError = MueError;
    type CommandRollback = u32;

    fn initialize(_participants: Vec<PubKey>, _metadata: &PayloadMetadata) -> Self {
        Self { game: Game::default() }
    }

    fn execute(
        &mut self,
        cmd: &Self::Command,
        auth: Option<PubKey>,
        metadata: &PayloadMetadata,
    ) -> Result<Self::CommandRollback, EpisodeError<Self::CommandError>> {
        let game_cmd = if cmd.is_witness {
            // For now, skip strict proof validation — just award witness points
            GameCommand::WitnessPoints { level: cmd.mu_level }
        } else {
            GameCommand::AddPoints { level: cmd.mu_level }
        };

        self.game
            .execute(&game_cmd, auth, metadata)
            .map_err(|e| match e {
                EpisodeError::CommandError(inner) => EpisodeError::CommandError(MueError::Game(inner)),
                EpisodeError::InternalError(msg) => EpisodeError::InternalError(msg),
            })
    }

    fn rollback(&mut self, rollback: u32) -> bool {
        self.game.rollback(rollback)
    }
}

impl MueHeroSession {
    pub fn get_score(&self) -> u32 {
        self.game.score
    }

    pub fn get_rank(&self) -> String {
        Game::rank_from_score(self.game.score).to_string()
    }
}
