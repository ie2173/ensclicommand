

use crate::keccak256;

pub fn namehash(name: &str) ->[u8;32] {
    if name.is_empty() {
        return [0u8;32];
    }
    let mut node = [0u8;32];
    let parts: Vec<&str> = name.split('.').rev().collect();
    for part in parts {
        let label_hash = keccak256::keccak256(part.as_bytes());
        let mut to_hash = [0u8;64];
        to_hash[..32].copy_from_slice(&node);
        to_hash[32..].copy_from_slice(&label_hash);
        node = keccak256::keccak256(&to_hash);
    }
    node
}