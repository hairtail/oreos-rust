use crate::{
    abi::{NoteWitness, OreoOverview, OreoTransaction, PostTransactionResponse, RpcTransaction},
    rpc::RpcResponse,
};
use anyhow::Result;
use std::time::Duration;
use ureq::Agent;

pub struct OreoscanRequest {
    pub base_route: String,
    pub agent: Agent,
}

impl OreoscanRequest {
    pub fn new() -> Self {
        Self {
            base_route: "http://www.oreoscan.info/v0/api".into(),
            agent: ureq::AgentBuilder::new()
                .timeout_read(Duration::from_secs(10))
                .timeout_write(Duration::from_secs(10))
                .build(),
        }
    }

    pub fn get_chain_header(&self) -> Result<u32> {
        let path = format!("{}/overview", self.base_route);
        let response: OreoOverview = ureq::get(&path).call()?.into_json()?;
        Ok(response.height)
    }

    pub fn get_transaction(&self, hash: &str) -> Result<OreoTransaction> {
        let oreos_tx_path = format!("{}/transaction/{}", self.base_route, hash);
        let response: OreoTransaction = ureq::get(&oreos_tx_path).call()?.into_json()?;
        Ok(response)
    }

    pub fn get_rpc_transaction(
        &self,
        block_hash: &str,
        transaction_hash: &str,
    ) -> Result<RpcTransaction> {
        let route = format!("{}/rpc/tx", &self.base_route);
        let response: RpcResponse<RpcTransaction> = self
            .agent
            .clone()
            .post(&route)
            .send_json(ureq::json!({
                "blockHash": block_hash,
                "transactionHash": transaction_hash,
            }))?
            .into_json()?;
        Ok(response.data)
    }

    pub fn get_rpc_note_witness(&self, index: u64) -> Result<NoteWitness> {
        let route = format!("{}/rpc/note", &self.base_route);
        let response: RpcResponse<NoteWitness> = self
            .agent
            .clone()
            .post(&route)
            .send_json(ureq::json!({
                "index": index,
            }))?
            .into_json()?;
        Ok(response.data)
    }

    pub fn post_rpc_transaction(
        &self,
        signed_transaction: String,
    ) -> Result<PostTransactionResponse> {
        let route = format!("{}/rpc/addTx", &self.base_route);
        let response: RpcResponse<PostTransactionResponse> = self
            .agent
            .clone()
            .post(&route)
            .send_json(ureq::json!({
                "transaction": signed_transaction,
            }))?
            .into_json()?;
        Ok(response.data)
    }
}
