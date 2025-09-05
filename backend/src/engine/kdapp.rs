// backend/src/engine/kdapp.rs

use crate::episode::{Episode, EpisodeError, PayloadMetadata};
use crate::engine::game::{Game, GameCommand};
use crate::state::pki::PubKey;
use crate::state::types::SuperblockEvent;

#[derive(Default, Clone)]
pub struct MueHeroSession {
    game: Game,
}

impl Episode for MueHeroSession {
    type Command = SuperblockEvent;
    type CommandError = ();          // no custom command errors for now
    type CommandRollback = u32;

    fn initialize(_participants: Vec<PubKey>, _metadata: &PayloadMetadata) -> Self {
        Self { game: Game::default() }
    }

    fn execute(
        &mut self,
        cmd: &Self::Command,
        _auth: Option<PubKey>,
        _metadata: &PayloadMetadata,
    ) -> Result<Self::CommandRollback, EpisodeError<Self::CommandError>> {
        let game_cmd = if cmd.is_witness {
            GameCommand::WitnessPoints { level: cmd.mu_level }
        } else {
            GameCommand::AddPoints { level: cmd.mu_level }
        };

        let delta = self.game.execute(&game_cmd, _auth, _metadata);
        Ok(delta)
    }

    fn rollback(&mut self, rollback: u32) -> bool {
        self.game.rollback(rollback)
    }
}

impl MueHeroSession {
    pub fn get_score(&self) -> u32 {
        self.game.score
    }

    // Optional: keep for UI compatibility, tierless (just return score as text)
    pub fn get_rank(&self) -> String {
        self.game.score.to_string()
    }
}
