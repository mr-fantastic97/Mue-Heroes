use borsh::{BorshSerialize, BorshDeserialize};

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct SuperblockEvent {
    pub mu_level: u8,
    pub witness_miner: bool,
}