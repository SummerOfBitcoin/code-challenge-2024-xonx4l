use sha2::{Sha256, Digest};

pub fn mine_block(block_data: &[u8], difficulty_target: &[u8; 32]) -> Vec<u8> {
    let mut nonce: u32 = 0;
    let mut mined_block = block_data.to_vec();

    loop {
        // Update the nonce in the block header
        mined_block[76..80].copy_from_slice(&nonce.to_le_bytes());

        // Calculate the block hash
        let mut hasher = Sha256::new();
        hasher.update(&mined_block);
        let mut hash = hasher.finalize_reset();
        hasher.update(&hash);
        let block_hash = hasher.finalize();

        // Check if the block hash meets the difficulty target
        if block_hash.as_slice() < difficulty_target {
            break;
        }

        nonce += 1;
    }

    mined_block
}