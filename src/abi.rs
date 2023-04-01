use ironfish_rust::Note;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct OreoOverview {
    pub height: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OreoTransaction {
    pub hash: String,
    pub blockHash: String,
    pub index: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Asset {
    pub assetId: String,
    pub value: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RpcNote {
    pub noteIndex: u64,
    pub noteData: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RpcTransaction {
    pub fee: String,
    pub expiration: u64,
    pub index: u64,
    pub notesCount: u64,
    pub spendsCount: u64,
    pub signature: String,
    pub notesEncrypted: Vec<RpcNote>,
    pub mints: Vec<Asset>,
    pub burns: Vec<Asset>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthPath {
    pub side: String,
    pub hashOfSibling: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NoteWitness {
    pub treeSize: u64,
    pub rootHash: String,
    pub authPath: Vec<AuthPath>,
}

pub struct TransactionReceiver {
    pub note: Note,
    pub index: u64,
    pub address: String,
    pub value: u64,
    pub assetId: String,
    pub memo: String,
}
