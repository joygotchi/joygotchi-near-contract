use near_sdk::{
    AccountId,
    borsh::{self, BorshDeserialize, BorshSerialize},
    serde::{Deserialize, Serialize},
};

use super::{PetId, PoolId};

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct PetCountUser {
    pub user: AccountId,
    pub pet_count: u8,
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct NFTInfo {
    pub nft_id: u128,
    pub owner: AccountId
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct PoolMetadata {
    pub pool_id: PoolId,
    pub price_per_slot: u128,
    pub pool_info: PoolInfo,
    pub staked_pets: Vec<NFTInfo>,
}


#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct PoolInfo {
    pub name: String,
    pub reward_nft_ids: Vec<u128>,
    pub staking_start_time: u128,
    pub staking_end_time: u128,
    pub max_slot_in_pool: u128,
    pub token_reward_per_slot: u128,
    pub max_slot_per_wallet: u128,
    pub total_staked_slot: u128,
}

pub trait StakingAndMining {
    fn create_new_staking_pool(&mut self, name: String, reward_nft_ids: Vec<u128>, staking_start_time: u128, staking_end_time: u128, max_slot_in_pool: u128, token_reward_per_slot: u128, max_slot_per_wallet: u128) -> PoolMetadata;
    fn stake(&mut self, nft_id: PetId, pool_id: PoolId) -> PoolMetadata;
    fn un_stake(&mut self, nft_id: PetId, pool_id: PoolId);
    fn add_mining_tool(&mut self, tool_id: u64);
}