//allows Kdapp state to persist per wallet during runtime.


use std::collections::HashMap;
use std::sync::Mutex;
use once_cell::sync::Lazy;
use crate::pki::PubKey;
use crate::kdapp::MueHeroSession;
pub mod pki;

pub static SESSIONS: Lazy<Mutex<HashMap<PubKey, MueHeroSession>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));
