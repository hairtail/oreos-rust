use clap::Parser;
use ironfish_rust::{
    errors::IronfishError, keys::Language, IncomingViewKey, MerkleNote, Note, OutgoingViewKey,
    SaplingKey,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Parser, Clone)]
#[clap(name = "oreos", author = "hairtail")]
#[clap(author, version, about, long_about = None)]
pub enum Cli {
    /// Create a new wallet
    Create,
    /// Recover wallet from spendingKey | mnemonic
    Recover(Recover),
    /// Decrypt an encrypted note
    Decrypt(Decrypt),
    /// Decrypt a transaction
    Watch(Transaction),
}

#[derive(Debug, Parser, Clone)]
pub struct Recover {
    /// Mnemonic or spendingKey used to recover wallet from
    #[clap(short, long)]
    pub data: String,
    /// Language if mnemonic is used
    #[clap(short, long, default_value_t=String::from("en"))]
    pub language: String,
}

#[derive(Debug, Parser, Clone)]
pub struct Decrypt {
    /// Hex encoded data of encrypted note
    #[clap(short, long)]
    pub data: String,
    /// Hex encoded account incoming view key
    #[clap(short, long)]
    pub incoming_viewkey: String,
    /// Hex encoded account outgoing view key
    #[clap(short, long)]
    pub outgoing_viewkey: String,
}

#[derive(Debug, Parser, Clone)]
pub struct Transaction {
    /// Transaction hash
    #[clap(long)]
    pub hash: String,
    /// Hex encoded account incoming view key
    #[clap(short, long)]
    pub incoming_viewkey: String,
    /// Hex encoded account outgoing view key
    #[clap(short, long)]
    pub outgoing_viewkey: String,
    /// Rpc node for getTransaction
    #[clap(long)]
    pub endpoint: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct OreoTransaction {
    hash: String,
    blockHash: String,
    index: u32,
}

#[derive(Debug, Deserialize, Serialize)]
struct RpcResponse {
    status: u16,
    data: RpcTransaction,
}

#[derive(Debug, Deserialize, Serialize)]
struct RpcTransaction {
    notesEncrypted: Vec<String>,
}

pub struct TransactionReceiver {
    address: String,
    value: u64,
    assetId: String,
    memo: String,
}

pub fn recover_key(data: Recover) -> Result<String, IronfishError> {
    let key_from_spending = SaplingKey::from_hex(&data.data);
    match key_from_spending {
        Ok(key) => Ok(key.to_string()),
        Err(_error) => SaplingKey::from_words(
            data.data.clone(),
            Language::from_language_code(&data.language).unwrap_or(Language::English),
        )
        .map(|key| key.to_string()),
    }
}

fn decrypt_encrypted_note(enc: Decrypt) -> Result<Note, IronfishError> {
    let data = hex::decode(enc.data).unwrap();
    let note_enc = MerkleNote::read(&data[..])?;

    let incoing_view_key = IncomingViewKey::from_hex(&enc.incoming_viewkey)?;
    let note_as_receiver = note_enc.decrypt_note_for_owner(&incoing_view_key);
    if let Ok(note) = note_as_receiver {
        return Ok(note);
    }

    let outgoing_view_key = OutgoingViewKey::from_hex(&enc.outgoing_viewkey)?;
    let note_as_spender = note_enc.decrypt_note_for_spender(&outgoing_view_key);
    if let Ok(note) = note_as_spender {
        return Ok(note);
    }

    Err(IronfishError::InvalidDecryptionKey)
}

pub fn decrypt_encrypted_note_print(enc: Decrypt) -> Result<String, IronfishError> {
    decrypt_encrypted_note(enc).map(|note| note.to_string())
}

fn decrypt_tx(tx: Transaction) -> Result<HashMap<String, Vec<TransactionReceiver>>, IronfishError> {
    let oreos_tx_path = format!("http://www.oreoscan.info/v0/api/transaction/{}", tx.hash);
    let transaction_info: OreoTransaction = ureq::get(&oreos_tx_path)
        .call()
        .unwrap()
        .into_json()
        .unwrap();
    let full_path = format!("http://{}/chain/getTransaction", tx.endpoint);
    let resp: RpcResponse = ureq::post(&full_path)
        .send_json(ureq::json!({
            "blockHash": transaction_info.blockHash.clone(),
            "transactionHash": transaction_info.hash.clone(),
        }))
        .unwrap()
        .into_json()
        .unwrap();
    let mut result: HashMap<String, Vec<TransactionReceiver>> = HashMap::new();
    for item in resp.data.notesEncrypted {
        if let Ok(note) = decrypt_encrypted_note(Decrypt {
            data: item,
            incoming_viewkey: tx.incoming_viewkey.clone(),
            outgoing_viewkey: tx.outgoing_viewkey.clone(),
        }) {
            let key = note.sender().hex_public_address();
            let receiver = TransactionReceiver {
                address: note.owner().hex_public_address(),
                value: note.value(),
                assetId: hex::encode(note.asset_id()),
                memo: note.memo().to_string(),
            };
            result.entry(key).or_insert(Vec::new()).push(receiver);
        }
    }
    Ok(result)
}

pub fn decrypt_tx_print(tx: Transaction) -> Result<String, IronfishError> {
    decrypt_tx(tx).map(|tx| {
        let mut result = String::from("");
        for (sender, receivers) in tx.iter() {
            let sender = format!("Sender: {}\n", sender);
            result.push_str(&sender);
            for receiver in receivers {
                let line = format!(
                    "Receiver: {}, {}, {}, {}\n",
                    receiver.address, receiver.value, receiver.assetId, receiver.memo
                );
                result.push_str(&line);
            }
        }
        result
    })
}
