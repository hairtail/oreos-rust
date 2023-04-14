use ironfish_rust::Note;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Asset {
    pub assetId: String,
    pub value: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RpcTransaction {
    pub fee: String,
    pub expiration: u64,
    pub noteSize: u64,
    pub notesCount: u64,
    pub spendsCount: u64,
    pub signature: String,
    pub notesEncrypted: Vec<String>,
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

#[derive(Debug, Deserialize, Serialize)]
pub struct BroadcastTransactionResponse {
    pub hash: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BlockIdentifier {
    pub index: String,
    pub hash: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ChainInfo {
    pub currentBlockIdentifier: BlockIdentifier,
}
