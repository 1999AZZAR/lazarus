use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LazarusHeader {
    pub magic: [u8; 4],         // "LZRS"
    pub original_size: u64,
    pub block_size: u32,
    pub total_blocks: u32,
    pub density: f32,
    pub fingerprints: Vec<u32>, // CRC-32 for each block
}
