use transaction::validate_transaction;

pub fn create_coinbase_transaction() -> Vec<u8> {
    let mut tx = vec![
        // version
        0x01, 0x00, 0x00, 0x00,
        // input count
        0x01,
        // input
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0xff, 0xff, 0xff, 0xff,
        0x07,
        0x04, 0xff, 0xff, 0x00, 0x1d, 0x01, 0x04,
        0x00, 0x00, 0x00, 0x00,
        // output count
        0x01,
        // output
        0x00, 0xf2, 0x05, 0x2a, 0x01, 0x00, 0x00, 0x00, // value (50 BTC in satoshis)
        0x19, // script length
        0x76, 0xa9, 0x14, // OP_DUP OP_HASH160 OP_PUSHBYTES_20
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // placeholder for script hash
        0x88, 0xac, // OP_EQUALVERIFY OP_CHECKSIG
        0x00, 0x00, 0x00, 0x00, // lock time
    ];

    tx
}

pub fn construct_block(coinbase_tx: &[u8], valid_transactions: &[(Vec<u8>, Vec<Vec<u8>>)]) -> Vec<u8> {
    let mut block_data = vec![];

    // Block header (placeholder)
    block_data.extend_from_slice(&[0u8; 80]);

    // Serialize the coinbase transaction
    let coinbase_tx_bytes = coinbase_tx.to_vec();
    let coinbase_tx_len = coinbase_tx_bytes.len() as u64;
    block_data.extend(coinbase_tx_len.to_le_bytes().iter().cloned());
    block_data.extend(coinbase_tx_bytes);

    // Serialize the valid transactions
    for (tx, witness) in valid_transactions {
        let mut tx_bytes = tx.to_vec();
        for input_witness in witness {
            let witness_len = input_witness.len() as u64;
            tx_bytes.extend(witness_len.to_le_bytes().iter().cloned());
            tx_bytes.extend(input_witness);
        }
        let tx_len = tx_bytes.len() as u64;
        block_data.extend(tx_len.to_le_bytes().iter().cloned());
        block_data.extend(tx_bytes);
    }

    block_data
}