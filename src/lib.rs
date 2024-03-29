pub mod abi;
pub mod cli;
pub mod rpc;

use abi::TransactionReceiver;
use cli::Account;
use ironfish_rust::{
    assets::asset_identifier::NATIVE_ASSET,
    errors::IronfishError,
    keys::Language,
    sapling_bls12::Scalar,
    witness::{Witness, WitnessNode},
    IncomingViewKey, MerkleNote, Note, OutgoingViewKey, ProposedTransaction, PublicAddress,
    SaplingKey,
};
use rpc::RpcHandler;
use std::{collections::HashMap, ops::Mul};

const IRON_TO_ORE: u64 = 10_000_0000;

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
    handler: &RpcHandler,
    hash: String,
    incoming_viewkey: &str,
    outgoing_viewkey: &str,
) -> anyhow::Result<HashMap<String, Vec<TransactionReceiver>>> {
    let transaction = handler.get_transaction(&hash)?;
    let mut note_index = transaction.noteSize - transaction.notesCount;
    let mut result: HashMap<String, Vec<TransactionReceiver>> = HashMap::new();
    for item in transaction.notesEncrypted {
        if let Ok(note) = decrypt_encrypted_note(item, incoming_viewkey, outgoing_viewkey) {
            let key = note.sender().hex_public_address();
            let receiver = TransactionReceiver {
                note: note.clone(),
                index: note_index,
                address: note.owner().hex_public_address(),
                value: note.value(),
                assetId: hex::encode(note.asset_id().as_bytes()),
                memo: note.memo().to_string(),
            };
            result.entry(key).or_insert(Vec::new()).push(receiver);
        }
        note_index += 1;
    }
    Ok(result)
}

pub fn decrypt_tx(
    hash: String,
    incoming_viewkey: String,
    outgoing_viewkey: String,
    endpoint: String,
) -> Result<String, IronfishError> {
    let handler = RpcHandler::new(endpoint);
    let decrypted_note =
        decrypt_tx_internal(&handler, hash.clone(), &incoming_viewkey, &outgoing_viewkey);
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
                        receiver.address,
                        receiver.value as f64 / IRON_TO_ORE as f64,
                        receiver.assetId,
                        receiver.memo
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
                        "You have received {} $IRON in this transaction",
                        sendable_value as f64 / IRON_TO_ORE as f64
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
    spending_key: String,
    receiver: String,
    amount: f64,
    fee: u64,
    expiration: Option<u32>,
    memo: String,
    endpoint: String,
) -> Result<String, IronfishError> {
    let handler = RpcHandler::new(endpoint);
    match decrypt_tx_internal(&handler, hash.clone(), &incoming_viewkey, &outgoing_viewkey) {
        Ok(data) => {
            // Handle send amount, f64 u64
            let amount_in_ore = amount.mul(IRON_TO_ORE as f64).trunc();
            if amount_in_ore >= u64::MAX as f64 {
                panic!("Too large send amount, this should never happen");
            }
            let amount_in_ore = amount_in_ore as u64;

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
            if sendable_value <= 0u64 {
                println!("You are not receiver of this transaction");
                return Err(IronfishError::InvalidBalance);
            }

            println!(
                "You have received {} $IRON in this transaction.",
                sendable_value as f64 / IRON_TO_ORE as f64
            );

            if sendable_value <= amount_in_ore {
                println!("Amount not enough to send");
                return Err(IronfishError::InvalidBalance);
            }

            println!(
                "You are about to send: {} $IRON to {}, gas: {} $ore, memo: {}",
                amount, receiver, fee, memo
            );

            // Transaction creation starts here
            let mut builder = ProposedTransaction::new(SaplingKey::from_hex(&spending_key)?);

            // Transaction spends
            for spend in spends.iter() {
                let witness_rpc = handler.get_witness(spend.index).unwrap();
                let witness = Witness {
                    tree_size: witness_rpc.treeSize as usize,
                    root_hash: Scalar::from_bytes(
                        hex::decode(witness_rpc.rootHash.clone()).unwrap()[..]
                            .try_into()
                            .unwrap(),
                    )
                    .unwrap(),
                    auth_path: witness_rpc
                        .authPath
                        .iter()
                        .map(|item| {
                            let data = hex::decode(item.hashOfSibling.clone()).unwrap();
                            let sc = Scalar::from_bytes(&data.try_into().unwrap()).unwrap();
                            if item.side.as_str() == "Left" {
                                WitnessNode::Left(sc)
                            } else {
                                WitnessNode::Right(sc)
                            }
                        })
                        .collect(),
                };
                builder.add_spend(spend.note.clone(), &witness)?;
            }

            // Transaction outputs
            let output = create_output(&receiver, &addr, amount_in_ore, memo)?;
            builder.add_output(output)?;

            // Transaction expiration
            match expiration {
                Some(num) => builder.set_expiration(num),
                None => {
                    let height = handler.get_chain_header().unwrap();
                    builder.set_expiration(height + 30);
                }
            }

            let transaction = builder.post(None, fee)?;
            transaction.verify()?;
            let mut vec: Vec<u8> = vec![];
            transaction.write(&mut vec)?;
            let signed_transaction = hex::encode(vec);
            let send_result = handler.broadcast_transaction(signed_transaction);

            match send_result {
                Ok(res) => Ok(format!("Transaction sent successfully, hash: {}", res.hash)),
                Err(e) => Ok(format!("Transaction was rejected, reason: {}", e)),
            }
        }
        Err(e) => Ok(e.to_string()),
    }
}

fn create_output(
    owner_address: &str,
    sender_address: &str,
    value: u64,
    memo: String,
) -> Result<Note, IronfishError> {
    let owner = PublicAddress::from_hex(owner_address)?;
    let sender = PublicAddress::from_hex(sender_address)?;
    Ok(Note::new(owner, value, memo, NATIVE_ASSET, sender))
}
