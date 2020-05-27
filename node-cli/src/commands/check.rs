use clap::ArgMatches;
use log::info;
use primitives::H256;
use std::path::Path;

use crate::config::Config;
use crate::db::ChainDB;

pub async fn main<P: AsRef<Path>>(config_path: P, matches: &ArgMatches<'_>) -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::load_from_file(config_path)?;
    info!("config file loaded");
    let db = ChainDB::new(&config.storage.data_dir);
    info!("db opened");

    db.await_background_jobs();

    match matches.value_of("WHAT") {
        Some("compact") => {
            println!("compact db ...");
            let ret = db.compact_db();
            println!("compact => {:?}", ret);
        }
        Some("merkle_tree") => {
            let patch = config
                .merkle_tree_patch
                .iter()
                .map(|patch| {
                    (
                        H256::from_slice(&hex::decode(&patch.txn).unwrap()),
                        H256::from_slice(&hex::decode(&patch.tree_node_hash).unwrap()),
                    )
                })
                .collect();
            db.verify_merkle_tree(&patch)?;
        }
        Some("parent_hash") => {
            db.verify_parent_hashes()?;
        }
        _ => (),
    }

    info!("block height = {}", db.get_block_height());

    Ok(())
}