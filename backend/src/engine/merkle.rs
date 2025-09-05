//backend/src/engine/merkle.rs

#[cfg(feature = "proofs")]
use blake2::{Blake2b512, Digest};

#[cfg(feature = "proofs")]
use crate::state::pki::PubKey;

#[cfg(feature = "proofs")]
fn hash_pair(left: &[u8], right: &[u8]) -> [u8; 64] {
    let mut hasher = Blake2b512::new();
    hasher.update(left);
    hasher.update(right);
    let result = hasher.finalize();
    let mut out = [0u8; 64];
    out.copy_from_slice(&result[..]);
    out
}

#[cfg(feature = "proofs")]
pub fn compute_merkle_root(mut hashes: Vec<[u8; 64]>) -> [u8; 64] {
    if hashes.is_empty() { return [0u8; 64]; }
    while hashes.len() > 1 {
        let mut next = Vec::with_capacity((hashes.len() + 1) / 2);
        for i in (0..hashes.len()).step_by(2) {
            let l = hashes[i];
            let r = if i + 1 < hashes.len() { hashes[i + 1] } else { l };
            next.push(hash_pair(&l, &r));
        }
        hashes = next;
    }
    hashes[0]
}

#[cfg(feature = "proofs")]
pub fn verify_merkle_proof(
    mut leaf: [u8; 64],
    proof: Vec<[u8; 64]>,
    merkle_root: [u8; 64],
    mut index: usize,
) -> bool {
    for p in proof {
        leaf = if index % 2 == 0 { hash_pair(&leaf, &p) } else { hash_pair(&p, &leaf) };
        index /= 2;
    }
    leaf == merkle_root
}

#[cfg(feature = "proofs")]
pub fn compute_leaf_from_wallet(pubkey: &PubKey) -> [u8; 64] {
    let mut hasher = Blake2b512::new();
    hasher.update(pubkey.as_bytes());
    let result = hasher.finalize();
    let mut out = [0u8; 64];
    out.copy_from_slice(&result[..]);
    out
}
