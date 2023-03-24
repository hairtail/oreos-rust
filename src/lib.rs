pub mod abi;
pub mod cli;
pub mod oreoscan;
pub mod rpc;

use abi::TransactionReceiver;
use cli::Account;
use ironfish_rust::{
    errors::IronfishError, keys::Language, IncomingViewKey, MerkleNote, Note, OutgoingViewKey,
    PublicAddress, SaplingKey,
};
use oreoscan::OreoscanRequest;
use rpc::RpcHandler;
use std::collections::HashMap;

pub fn create_account(account: Account) -> Result<String, IronfishError> {
    match account {
        Account::New {
            mnemonic,
            language,
            key,
        } => {
            if mnemonic.is_some() {
                Ok(SaplingKey::from_words(
                    mnemonic.unwrap(),
                    Language::from_language_code(&language).unwrap_or(Language::English),
                )?
                .to_string())
            } else if key.is_some() {
                Ok(SaplingKey::from_hex(&key.unwrap())?.to_string())
            } else {
                Ok(SaplingKey::generate_key().to_string())
            }
        }
    }
}

fn decrypt_encrypted_note(
    enc_note: String,
    incoming_viewkey: &str,
    outgoing_viewkey: &str,
) -> Result<Note, IronfishError> {
    let data = hex::decode(enc_note).unwrap();
    let note_enc = MerkleNote::read(&data[..])?;

    let incoing_view_key = IncomingViewKey::from_hex(incoming_viewkey)?;
    let note_as_receiver = note_enc.decrypt_note_for_owner(&incoing_view_key);
    if let Ok(note) = note_as_receiver {
        return Ok(note);
    }

    let outgoing_view_key = OutgoingViewKey::from_hex(outgoing_viewkey)?;
    let note_as_spender = note_enc.decrypt_note_for_spender(&outgoing_view_key);
    if let Ok(note) = note_as_spender {
        return Ok(note);
    };

    Err(IronfishError::InvalidViewingKey)
}

fn decrypt_tx_internal(
    hash: String,
    incoming_viewkey: &str,
    outgoing_viewkey: &str,
    endpoint: String,
) -> anyhow::Result<HashMap<String, Vec<TransactionReceiver>>> {
    let transaction_info = OreoscanRequest::get_transaction(&hash)?;
    let rpc_handler = RpcHandler::new(endpoint);
    let resp = rpc_handler.get_transaction(&transaction_info.blockHash, &transaction_info.hash)?;
    let mut result: HashMap<String, Vec<TransactionReceiver>> = HashMap::new();
    for item in resp.notesEncrypted {
        if let Ok(note) = decrypt_encrypted_note(item, incoming_viewkey, outgoing_viewkey) {
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

pub fn decrypt_tx(
    hash: String,
    incoming_viewkey: String,
    outgoing_viewkey: String,
    endpoint: String,
) -> Result<String, IronfishError> {
    let decrypted_note = decrypt_tx_internal(
        hash.clone(),
        &incoming_viewkey,
        &outgoing_viewkey,
        endpoint.clone(),
    );
    match decrypted_note {
        Ok(notes) => {
            let mut result = String::from("");
            let view_key = IncomingViewKey::from_hex(&incoming_viewkey).unwrap();
            let addr = PublicAddress::from_view_key(&view_key).hex_public_address();
            let mut sendable_value = 0u64;
            for (sender, receivers) in notes.iter() {
                let sender = format!("Sender: {}\n", sender);
                result.push_str(&sender);
                for receiver in receivers {
                    let line = format!(
                        "Receiver: {}, {}, {}, {}\n",
                        receiver.address, receiver.value, receiver.assetId, receiver.memo
                    );
                    result.push_str(&line);
                    if receiver.address == addr {
                        sendable_value += receiver.value;
                    }
                }
            }
            if sendable_value > 0u64 {
                result.push_str(
                    format!(
                        "You have received {} $ore in this transaction",
                        sendable_value
                    )
                    .as_str(),
                );
            };
            return Ok(result);
        }
        Err(e) => return Ok(e.to_string()),
    };
}

pub fn causal_send(
    hash: String,
    incoming_viewkey: String,
    outgoing_viewkey: String,
    endpoint: String,
    receiver: String,
    amount: u64,
    fee: u64,
    memo: String,
) -> Result<String, IronfishError> {
    match decrypt_tx_internal(
        hash.clone(),
        &incoming_viewkey,
        &outgoing_viewkey,
        endpoint.clone(),
    ) {
        Ok(data) => {
            let view_key = IncomingViewKey::from_hex(&incoming_viewkey)?;
            let addr = PublicAddress::from_view_key(&view_key).hex_public_address();
            let mut sendable_value = 0u64;
            let mut spends = Vec::new();
            for (_sender, receivers) in data.iter() {
                for receiver in receivers {
                    if receiver.address == addr {
                        sendable_value += receiver.value;
                        spends.push(receiver);
                    }
                }
            }
            println!("You have received {} $ore in this transaction.\n You can send them to another address now", sendable_value);
            println!(
                "You are about to send: {} $ore to {}, gas: {} $ore, memo: {}",
                amount, receiver, fee, memo
            );
            Ok("hello".into())
        }
        Err(e) => Ok(e.to_string()),
    }
}
