use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use std::collections::HashSet;
use std::fs;
use glob::glob;
use secp256k1::{Secp256k1, Message, PublicKey, Signature};
use secp256k1::Error as SecpError;

#[derive(Serialize, Deserialize, Debug)]
struct Transaction {
    version: i32,
    locktime: i32,
    vin: Vec<Vin>,
    txid: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Prevout {
    scriptpubkey: String,
    scriptpubkey_asm: String,
    scriptpubkey_type: String,
    scriptpubkey_address: String,
    value: i64,
}

// Update Vin to include Prevout
#[derive(Serialize, Deserialize, Debug)]
struct Vin {
    txid: String,
    vout: i32,
    prevout: Option<Prevout>,
    scriptsig: String,
    scriptsig_asm: String,
    witness: Vec<String>,
    is_coinbase: bool,
    sequence: u32,
}


#[derive(Debug)]
struct Block {
    header: String,
    transactions: Vec<Transaction>,
}
fn read_transactions_from_mempool() -> Vec<Transaction> {
    let mut transactions = Vec::new();
    for entry in glob("./mempool/*.json").expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                let data = fs::read_to_string(path).expect("Unable to read file");
                let transaction: Transaction = serde_json::from_str(&data).expect("Unable to parse JSON");
                transactions.push(transaction);
            },
            Err(e) => println!("{:?}", e),
        }
    }
    transactions
}

fn extract_public_key(witness: &[String]) -> Option<String> {
   witness.get(1).cloned()
}

fn verify_signature(pub_key: &str, sig: &str, msg: &str) -> Result<bool, SecpError> {
    let secp = Secp256k1::new();
    let msg = match hex::decode(msg) {
        Ok(m) => Message::from_slice(&m)?,
        Err(_) => return Err(secp256k1::Error::InvalidMessage),
    };
    
    let pub_key = match hex::decode(pub_key) {
        Ok(pk) => PublicKey::from_slice(&pk)?,
        Err(_) => return Err(secp256k1::Error::InvalidPublicKey),
    };
    let sig_bytes = match hex::decode(sig) {
        Ok(bytes) => bytes,
        Err(_) => return Err(secp256k1::Error::InvalidSignature),
    };
    let sig = Signature::from_der(&sig_bytes)?;
    Ok(secp.verify(&msg, &sig, &pub_key).is_ok())
}

fn validate_transaction(tx: &Transaction, spent_outputs: &mut HashSet<String>) -> bool {
    let message = "message constructed from the transaction";
    let public_key = "public key of the sender";
    let signature = "signature of the transaction";

    match verify_signature("public_key", "transaction_hash", "signature") {
        Ok(valid) => if !valid {
            println!("Signature verification failed");
            return false;
        },
        Err(_) => {
            println!("Error verifying signature");
            return false;
        }
    }
    for input in &tx.vin {
        if !input.is_coinbase {
            if let Some(prevout) = &input.prevout {
                if prevout.scriptpubkey.is_empty() || prevout.scriptpubkey_address.is_empty() {
                    println!("Invalid or missing scriptpubkey details");
                    return false;
                }

                // Simulate checking for double spending
                let mut spent_outputs = std::collections::HashSet::new();
                let outpoint = format!("{}:{}", input.txid, input.vout);
                if !spent_outputs.insert(outpoint) {
                    println!("Double spending detected");
                    return false;
                }

                  match verify_signature("public_key", "transaction_hash", "signature") {
                    Ok(valid) => if !valid {
                        println!("Signature verification failed");
                        return false;
                    },
                    Err(_) => {
                        println!("Error verifying signature");
                        return false;
                    }
                } 
            } else {
                println!("Missing prevout for non-coinbase transaction");
                return false;
            }

            if input.scriptsig.is_empty() && input.witness.is_empty() {
                println!("Missing scriptsig and witness data for non-coinbase transaction");
                return false;
            }
        }
    }
    true
}
fn mine_block(transactions: Vec<Transaction>) -> Block {
    let mut nonce = 0;
    let mut header = String::new();
    loop {
        header = format!("...{}{}", transactions_hash(&transactions), nonce);
        let hash = Sha256::digest(header.as_bytes());
        let hash_hex = format!("{:x}", hash);
        if hash_hex < "0000ffff00000000000000000000000000000000000000000000000000000000".to_string() {
            break;
        }
        nonce += 1;
    }
    Block { header, transactions }
}

fn create_coinbase_transaction() -> Transaction {
    Transaction {
        version: 1, // Version 1 for simplicity
        locktime: 0, // No locktime
        vin: vec![Vin {
            txid: String::new(), // No input transaction
            vout: 0, // No output index
            prevout: None, // No previous output
            scriptsig: String::from("Coinbase"), // Identifier for coinbase tx
            scriptsig_asm: String::new(), // No asm for coinbase
            witness: vec![String::new()], // No witness data
            is_coinbase: true, // This is a coinbase transaction
            sequence: 0, // Sequence number
        }],
        txid: String::from("coinbase_txid"), // A placeholder txid
    }
}

fn transactions_hash(transactions: &[Transaction]) -> String {
    let txids = transactions.iter().map(|tx| tx.txid.clone()).collect::<Vec<String>>().join("");
    let hash = Sha256::digest(txids.as_bytes());
    format!("{:x}", hash)
}

fn write_output(block: &Block) {
    let mut output = block.header.clone() + "\n";
    let coinbase_tx_serialized = serde_json::to_string(&block.transactions[0]).expect("Unable to serialize coinbase transaction");
    output += &coinbase_tx_serialized;
    output += "\n";
    for tx in &block.transactions[1..] {
        output += &tx.txid;
        output += "\n";
    }
    fs::write("output.txt", output).expect("Unable to write file");
}

fn main() {
    let transactions = read_transactions_from_mempool();
    let mut spent_outputs: HashSet<String> = HashSet::new();
    let coinbase_transaction = create_coinbase_transaction();
    let mut all_transactions = vec![create_coinbase_transaction()];
    let valid_transactions: Vec<Transaction> = transactions.into_iter().filter(|tx| validate_transaction(tx, &mut spent_outputs)).collect();
    let block = mine_block(all_transactions);
    write_output(&block);

}