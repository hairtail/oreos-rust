use crate::abi::OreoTransaction;
use anyhow::Result;
pub struct OreoscanRequest {}

impl OreoscanRequest {
    pub fn get_transaction(hash: &str) -> Result<OreoTransaction> {
        let oreos_tx_path = format!("http://www.oreoscan.info/v0/api/transaction/{}", hash);
        let response: OreoTransaction = ureq::get(&oreos_tx_path).call()?.into_json()?;
        Ok(response)
    }
}
