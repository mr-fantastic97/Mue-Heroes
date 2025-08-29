// backend/src/episode.rs

/// Metadata that travels with every command (e.g., block time).
#[derive(Debug, Clone, Copy)]
pub struct PayloadMetadata {
    pub accepting_time: u64, // unix seconds (or millis if you prefer)
}

/// Errors an Episode can return.
#[derive(Debug)]
pub enum EpisodeError<E> {
    CommandError(E),
    InternalError(String),
}

/// A stateful, per-session/per-wallet “episode” of your game.
pub trait Episode {
    type Command;
    type CommandError;
    type CommandRollback;

    fn initialize(
        participants: Vec<crate::state::pki::PubKey>,
        metadata: &PayloadMetadata,
    ) -> Self;

    fn execute(
        &mut self,
        cmd: &Self::Command,
        auth: Option<crate::state::pki::PubKey>,
        metadata: &PayloadMetadata,
    ) -> Result<Self::CommandRollback, EpisodeError<Self::CommandError>>;

    fn rollback(&mut self, rollback: Self::CommandRollback) -> bool;
}
