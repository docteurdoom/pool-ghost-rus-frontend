use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use serde::Deserialize;
use serde_json::Value;
use crate::JSON_PATH;

#[derive(Default, Debug, Clone, Deserialize)]
pub struct PoolData {
    #[serde(rename(deserialize = "mode"))]
    pub core_version: String,
    #[serde(rename(deserialize = "pool_adress"))]
    pub pubkey: String,
    #[serde(rename(deserialize = "pool_fee"))]
    pub fee: String,
    pub stake_bonus: String,
    pub blocks_between_payment_runs: String,
    pub minimum_output_value: String,
    pub synced_height: String,
    pub blocks_found: String,
    pub total_disbursed: String,
    pub last_payment_run: String,
    pub total_pool_rewards: String,
    pub total_pool_fees: String,
    pub total_pooled_coin: String,
    pub currently_staking: String,
    pub recent_blocks: RecentBlocks,
    pub last_payments: LastPayments,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct RecentBlocks {
    pub height: [String; 5],
    pub block_hash: [String; 5],
    pub block_reward: [String; 5],
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct LastPayments {
    pub height: [String; 5],
    pub txid: [String; 5],
    pub disbursed: [String; 5],
}

pub fn raw_data() -> Result<Value, Box<dyn Error>> {
    let mut file = File::open(PathBuf::from(JSON_PATH))?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    let json = serde_json::from_str(&buffer)?;
    Ok(json)
}

pub fn deserialized_data() -> Result<PoolData, Box<dyn Error>> {
    let mut file = File::open(PathBuf::from(JSON_PATH))?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    let pooldata: PoolData = serde_json::from_str(&buffer)?;
    Ok(pooldata)
}