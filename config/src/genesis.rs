use std::error::Error;
use std::fs;
use std::path::Path;

use chain::IndexedBlock;
use keys::Address;
use proto::chain::{block_header::Raw as BlockHeaderRaw, transaction::Raw as TransactionRaw, BlockHeader, Transaction};
use proto::contract::TransferContract;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Witness {
    pub address: String,
    pub url: String,
    pub votes: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Alloc {
    pub address: String,
    pub name: String,
    pub balance: i64,
}

impl Alloc {
    fn to_transaction(&self, sender: &[u8]) -> Result<Transaction, Box<dyn Error>> {
        let transfer_contract = TransferContract {
            owner_address: sender.to_owned(),
            to_address: self.address.parse::<Address>()?.as_bytes().to_owned(),
            amount: self.balance,
        };
        let raw = TransactionRaw {
            contract: Some(transfer_contract.into()),
            ..Default::default()
        };
        let transaction = Transaction {
            raw_data: Some(raw).into(),
            ..Default::default()
        };
        Ok(transaction)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GenesisConfig {
    pub timestamp: i64,
    #[serde(rename = "parentHash")]
    parent_hash: String,
    mantra: String,
    creator: String,
    pub witnesses: Vec<Witness>,
    pub allocs: Vec<Alloc>,
}

impl GenesisConfig {
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn Error>> {
        let content = fs::read_to_string(path)?;
        Ok(serde_json::from_str(&content)?)
    }

    pub fn load_from_str(content: &str) -> Result<Self, Box<dyn Error>> {
        Ok(serde_json::from_str(&content)?)
    }

    fn to_block_header(&self) -> BlockHeader {
        let raw_header = BlockHeaderRaw {
            number: 0,
            timestamp: self.timestamp,
            witness_address: self.mantra.as_bytes().to_owned(),
            parent_hash: parse_hex(&self.parent_hash),
            // merkle_root_hash will be filled by indexed block header
            ..Default::default()
        };
        BlockHeader {
            raw_data: Some(raw_header).into(),
            ..Default::default()
        }
    }

    pub fn to_indexed_block(&self) -> Result<IndexedBlock, Box<dyn Error>> {
        let sender = keys::b58decode_check(&self.creator)?;
        let transactions = self
            .allocs
            .iter()
            .map(|alloc| alloc.to_transaction(&sender))
            .collect::<Result<Vec<Transaction>, Box<dyn Error>>>()?;

        Ok(IndexedBlock::from_raw_header_and_txns(self.to_block_header(), transactions).unwrap())
    }
}

fn parse_hex(encoded: &str) -> Vec<u8> {
    if encoded.starts_with("0x") {
        hex::decode(&encoded[2..]).unwrap()
    } else {
        hex::decode(encoded).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_genesis_json() {
        let content = include_str!("../../etc/genesis.json");
        let conf: GenesisConfig = serde_json::from_str(&content).unwrap();

        let block = conf.to_indexed_block().unwrap();
        println!("block => {:?}", block);
        // mainnet: "8ef446bf3f395af929c218014f6101ec86576c5f61b2ae3236bf3a2ab5e2fecd"
        // nile:    "6556a96828248d6b89cfd0487d4cef82b134b5544dc428c8a218beb2db85ab24"
        // shasta:  "ea97ca7ac977cf2765093fa0e4732e561dc4ff8871c17e35fd2bcabb8b5f821d"

        println!("block_id => {:?}", hex::encode(block.merkle_root_hash()));
    }
}
