use std::fmt;
use std::hash::Hash;
use std::cmp::Eq; 
use serde::{Serialize, Deserialize};
use borsh::{BorshDeserialize, BorshSerialize};
use hex;

#[derive(Clone, PartialEq, Eq, Hash, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct PubKey([u8; 32]);


impl PubKey {
    pub fn new(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }
    
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

}


impl fmt::Display for PubKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Format as hex or simulate kaspa: prefix
        write!(f, "kaspa:{}", hex::encode(self.0))
    }
}