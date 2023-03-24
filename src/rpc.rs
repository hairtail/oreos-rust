use crate::abi::{NoteWitness, RpcTransaction};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use ureq::Agent;

#[derive(Debug, Deserialize, Serialize)]
pub struct RpcResponse {
    pub status: u16,
    pub data: RpcTransaction,
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
    pub fn get_transaction(
        &self,
        block_hash: &str,
        transaction_hash: &str,
    ) -> Result<RpcTransaction> {
        let path = format!("http://{}/chain/getTransaction", self.endpoint);
        let response: RpcResponse = self
            .agent
            .clone()
            .post(&path)
            .send_json(ureq::json!({
                "blockHash": block_hash,
                "transactionHash": transaction_hash,
            }))?
            .into_json()?;
        Ok(response.data)
    }

    pub fn get_witness(&self, _index: u64) -> Result<NoteWitness> {
        unimplemented!()
    }

    pub fn post_transaction(&self, _raw_transaction: String) -> Result<()> {
        unimplemented!()
    }
}
