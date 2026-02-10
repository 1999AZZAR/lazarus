pub mod encoder;
pub mod decoder;

use crc32fast::Hasher;

pub fn calculate_checksum(data: &[u8]) -> u32 {
    let mut hasher = Hasher::new();
    hasher.update(data);
    hasher.finalize()
}