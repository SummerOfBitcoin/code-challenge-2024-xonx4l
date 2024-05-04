use std::fs;

pub fn read_transactions(mempool_dir: &str) -> Result<Vec<Vec<u8>>, Box<dyn std::error::Error>> {
    let mut transactions = Vec::new();

    for entry in fs::read_dir(mempool_dir)? {
        let path = entry?.path();
        if path.is_file() {
            let tx_bytes = fs::read(path)?;
            transactions.push(tx_bytes);
        }
    }

    Ok(transactions)
}