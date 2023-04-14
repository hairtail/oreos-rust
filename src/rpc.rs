use crate::abi::{BroadcastTransactionResponse, ChainInfo, NoteWitness, RpcTransaction};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use ureq::Agent;

#[derive(Debug, Deserialize, Serialize)]
pub struct RpcResponse<T> {
    pub status: u16,
    pub data: T,
}

pub struct RpcHandler {
    pub endpoint: String,
    pub agent: Agent,
}

impl RpcHandler {
    pub fn new(endpoint: String) -> Self {
        Self {
            endpoint,
            agent: ureq::AgentBuilder::new()
                .timeout_read(Duration::from_secs(10))
                .timeout_write(Duration::from_secs(10))
                .build(),
        }
    }

    pub fn get_chain_header(&self) -> Result<u32> {
        let path = format!("http://{}/chain/getChainInfo", self.endpoint);
        let response: RpcResponse<ChainInfo> =
            self.agent.clone().post(&path).call()?.into_json()?;
        Ok(response.data.currentBlockIdentifier.index.parse::<u32>()?)
    }

    pub fn get_transaction(&self, transaction_hash: &str) -> Result<RpcTransaction> {
        let path = format!("http://{}/chain/getTransaction", self.endpoint);
        let response: RpcResponse<RpcTransaction> = self
            .agent
            .clone()
            .post(&path)
            .send_json(ureq::json!({
                "transactionHash": transaction_hash,
            }))?
            .into_json()?;
        Ok(response.data)
    }

    pub fn get_witness(&self, index: u64) -> Result<NoteWitness> {
        let path = format!("http://{}/chain/getNoteWitness", self.endpoint);
        let response: RpcResponse<NoteWitness> = self
            .agent
            .clone()
            .post(&path)
            .send_json(ureq::json!({
                "index": index,
            }))?
            .into_json()?;
        Ok(response.data)
    }

    pub fn broadcast_transaction(
        &self,
        signed_transaction: String,
    ) -> Result<BroadcastTransactionResponse> {
        println!("tx: {}", signed_transaction);
        let path = format!("http://{}/chain/broadcastTransaction", self.endpoint);
        let response: RpcResponse<BroadcastTransactionResponse> = self
            .agent
            .clone()
            .post(&path)
            .send_json(ureq::json!({
                "transaction": signed_transaction,
            }))?
            .into_json()?;
        Ok(response.data)
    }
}
