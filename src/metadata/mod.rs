use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LazarusHeader {
    pub magic: [u8; 4],
    pub original_size: u64,
    pub block_size: u32,
    pub total_blocks: u32,
    pub density: f32,
    pub fingerprints: Vec<u32>,        // DNA of Original Data
    pub has_recovery: bool,
    pub recovery_len: u64,
    pub compressed_fingerprints: Vec<u32>, // DNA of Compressed Data (for Repair)
}
