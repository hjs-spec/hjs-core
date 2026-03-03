//! Cryptographic operations for HJS

use sha2::{Sha256, Digest};

/// Compute SHA-256 hash
pub fn sha256(data: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}

/// Format hash as hex string with algorithm prefix
pub fn format_hash(hash: &[u8]) -> String {
    let hex_string = hash.iter()
        .map(|b| format!("{:02x}", b))
        .collect::<String>();
    format!("sha256:{}", hex_string)
}
