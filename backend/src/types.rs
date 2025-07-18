use borsh::{BorshSerialize, BorshDeserialize};

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct SuperblockEvent {
    pub mu_level: u8,
    pub is_witness: bool,
    pub merkle_root: Option<[u8; 64]>,
    pub proof: Option<Vec<[u8; 64]>>,
    pub witness_index: Option<usize>,
    pub block_height: u64,
}
