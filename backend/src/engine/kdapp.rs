use crate::episode::{Episode, EpisodeError, PayloadMetadata};
use crate::game::{Game, GameCommand, GameCommandError};
use crate::pki::PubKey;
use crate::types::SuperblockEvent;
use crate::utils::merkle::merkle::{verify_merkle_proof, compute_leaf_from_wallet};

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
        Self {
            game: Game::default(),
        }
    }

    fn execute(
        &mut self,
        cmd: &Self::Command,
        _auth: Option<PubKey>,
        metadata: &PayloadMetadata,
    ) -> Result<Self::CommandRollback, EpisodeError<Self::CommandError>> {
        let game_cmd = if cmd.is_witness {
            if let (Some(root), Some(proof), Some(index), Some(auth)) =
                (&cmd.merkle_root, &cmd.proof, cmd.witness_index, _auth)
            {
                let leaf = compute_leaf_from_wallet(auth);
                if verify_merkle_proof(leaf, proof.clone(), *root, index) {
                    GameCommand::WitnessPoints { level: cmd.mu_level }
                } else {
                    return Err(EpisodeError::InternalError("Invalid Merkle witness proof".into()));
                }
            } else {
                return Err(EpisodeError::InternalError("Missing Merkle data for witness miner".into()));
            }
        } else {
            GameCommand::AddPoints { level: cmd.mu_level }
        };

        self.game
            .execute(&game_cmd, _auth, metadata)
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
