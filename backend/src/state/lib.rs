use std::collections::HashMap;
use std::sync::RwLock;
use once_cell::sync::Lazy;

use crate::state::pki::PubKey;
use crate::engine::kdapp::MueHeroSession;

pub mod pki;
pub mod types;

/// Global session manager: tracks per-wallet game sessions
pub static SESSIONS: Lazy<RwLock<HashMap<PubKey, MueHeroSession>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));
