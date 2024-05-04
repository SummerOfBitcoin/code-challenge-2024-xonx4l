mod mempool;
mod transaction;
mod block;
mod mining;

use mempool::read_transactions;
use transaction::validate_transaction;
use block::{create_coinbase_transaction, construct_block};
use mining::mine_block;
use secp256k1::{Secp256k1};
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mempool_dir = "mempool";
    let transactions = read_transactions(mempool_dir)?;
    let mut valid_transactions = Vec::new();

    // Initialize the Secp256k1 context
    let secp256k1 = Secp256k1::new();

    // Filter valid transactions
    for tx in transactions {
        if let Some(witness) = validate_transaction(&tx, &secp256k1)? {
            valid_transactions.push((tx, witness));
        }
    }

    // Create the coinbase transaction
    let coinbase_tx = create_coinbase_transaction();

    // Construct the block
    let block_data = construct_block(&coinbase_tx, &valid_transactions);

    // Mine the block
    let difficulty_target = [
        0x00, 0x00, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00,
    ];
    let mined_block = mine_block(&block_data, &difficulty_target);

    // Write the mined block to output.txt
    fs::write("output.txt", mined_block)?;

    Ok(())
}