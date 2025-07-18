use blake2::{Blake2b512, Digest};

fn hash_pair(left: &[u8], right: &[u8]) -> [u8; 64] {
    let mut hasher = Blake2b512::new();
    hasher.update(left);
    hasher.update(right);
    let result = hasher.finalize();
    let mut hash = [0u8; 64];
    hash.copy_from_slice(&result);
    hash
}

pub fn compute_merkle_root(mut hashes: Vec<[u8; 64]>) -> [u8; 64] {
    if hashes.is_empty() {
        return [0u8; 64];
    }

    while hashes.len() > 1 {
        let mut next_level = Vec::new();
        for i in (0..hashes.len()).step_by(2) {
            let left = hashes[i];
            let right = if i + 1 < hashes.len() { hashes[i + 1] } else { left };
            next_level.push(hash_pair(&left, &right));
        }
        hashes = next_level;
    }

    hashes[0]
}

pub fn verify_merkle_proof(
    leaf: [u8; 64],
    proof: Vec<[u8; 64]>,
    merkle_root: [u8; 64],
    index: usize,
) -> bool {
    let mut computed = leaf;
    let mut i = index;

    for p in proof {
        computed = if i % 2 == 0 {
            hash_pair(&computed, &p)
        } else {
            hash_pair(&p, &computed)
        };
        i /= 2;
    }

    computed == merkle_root
}

/// Hash a wallet’s PubKey into a Merkle leaf
pub fn compute_leaf_from_wallet(pubkey: PubKey) -> [u8; 64] {
    let mut hasher = Blake2b512::new();
    hasher.update(pubkey.to_bytes());
    let result = hasher.finalize();
    let mut out = [0u8; 64];
    out.copy_from_slice(&result);
    out
}
