use std::collections::HashMap;

use ethers::prelude::*;
use ethers::types::{ TransactionRequest, U256, Bytes };
use ethers::types::transaction::eip2718::TypedTransaction;

#[derive(Debug)]
pub struct SignatureStruct {
    pub to: Address,
    pub value: U256,
    pub gas: U256,
    pub gas_price: U256,
    pub nonce: U256,
    pub data: Option<Bytes>,
    pub wallet: LocalWallet,
}

pub trait SignTransferTrait {
    async fn sign_transfer(&self) -> Result<String, String>;
}

impl SignatureStruct {
    pub fn from(map: &HashMap<String, String>) -> Result<Self, String> {
        let chain_id : u64 = if let Some(chain) = map.get("chain") {
            chain
                .replace("_", "")
                .parse::<u64>()
                .map_err(|_| "invalid chain id".to_string())?
        } else {
            return Err("chain id MUST be set.".to_string());
        };

        let wallet : LocalWallet = if let Some(pk) = map.get("pk") {
            let w = pk
                        .parse::<LocalWallet>()
                        .map_err(|_| "invalid private key".to_string())?;
            w.with_chain_id(chain_id)
        } else {
            return Err("sender's private key MUST be set.".to_string());
        };

        let to : Address = if let Some(to_str) = map.get("to") {
            to_str
                .parse::<Address>()
                .map_err(|_| "invalid receiver's address.".to_string())?
        } else {
            return Err("Received's address MUST be set.".to_string());
        };

        let value : U256 = if let Some(value_str) = map.get("value") {
            value_str
                .replace("_", "")
                .parse::<U256>()
                .map_err(|_| "invalid value".to_string())?
        } else {
            return Err("ammount MUST be set.".to_string())
        };

        let gas : U256  = if let Some(gas_str) = map.get("gas") {
            gas_str
                .replace("_", "")
                .parse::<U256>()
                .map_err(|_| "invalid gas value")?
        } else {
            return Err("GAS MUST be set.".to_string());
        };

        let gas_price : U256 = if let Some(gas_price_str) = map.get("gas_price") {
            gas_price_str
                .replace("_", "")
                .parse::<U256>()
                .map_err(|_| "invalid gas price.".to_string())?
        } else {
            return Err("GAS Price MUST be set.".to_string());
        };

        let nonce: U256 = if let Some(nonce_str) = map.get("nonce") {
            nonce_str
                .replace("_", "")
                .parse::<U256>()
                .map_err(|_| "invalid nonce.".to_string())?
        } else {
            return Err("NONCE MUST be set.".to_string());
        };

        let data : Option<Bytes> = if let Some(data_str) = map.get("data") {
            let bytes = match hex::decode(data_str) {
                Ok(vec) => Bytes::from(vec),
                Err(_) => Bytes::from(data_str.as_bytes().to_vec())
            };
            Some(bytes)
        } else {
            None
        };
        
        Ok(SignatureStruct{ to, value, gas, gas_price, nonce, data, wallet })
    }
}

impl SignTransferTrait for SignatureStruct {
    async fn sign_transfer(&self) -> Result<String, String> {
        let mut tx = TransactionRequest::new()
                .to(self.to)
                .value(self.value)
                .gas(self.gas)
                .gas_price(self.gas_price)
                .nonce(self.nonce);

        if let Some(bytes) = &self.data {
            tx = tx.data(bytes.clone());
        }

        let typed_tx : TypedTransaction = tx.clone().into();

        let sign = self.wallet
                        .sign_transaction(&typed_tx)
                        .await
                        .map_err(|_| "failed to sigh the transfer transaction".to_string())?;

        let rlp = tx.rlp_signed(&sign);

        Ok(format!("0x{}", hex::encode(rlp)))
    }
}