use near_contract_standards::non_fungible_token::TokenId;
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

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct MiningData {
    pub account_id: Option<AccountId>,
    pub mining_points: u128,
    pub total_mining_power: u128,
    pub total_mining_charge_time: u128,
    pub last_mining_time: u128,
    pub mining_tool_used: Vec<u128>,
}

pub trait StakingAndMining {
    fn create_new_staking_pool(&mut self, name: String, reward_nft_ids: Vec<u128>, staking_start_time: u128, staking_end_time: u128, max_slot_in_pool: u128, token_reward_per_slot: u128, max_slot_per_wallet: u128) -> PoolMetadata;
    fn stake(&mut self, nft_id: PetId, pool_id: PoolId) -> PoolMetadata;
    fn un_stake(&mut self, nft_id: PetId, pool_id: PoolId);
    fn add_mining_tool(&mut self, tool_id: u64);
    fn remove_mining_tool(&mut self, tool_id: u64);
    fn mining(&mut self);
    fn redemn_mining_points(&mut self);
    fn owner_withdraw_redundant_token(&mut self, pool_id: PoolId);
    fn configure_mining_pool(&mut self, name: String, mining_power_multiplier: u128, charge_of_time_multiplier: u128);
    fn set_mining_points_used_per_redemn(&mut self, points: u128);
    fn set_token_earned_per_redemn(&mut self, token: u128);
    fn set_price_per_slot(&mut self, price_per_slot: u128);
    fn caculate_charge_of_time(&mut self, number_of_tools: u128);
    fn remove_item_from_list_tool(&mut self, value: u128);
}

pub trait StakingAndMiningEnum {


    fn get_mining_data_by_account_id(&self, account_id: AccountId) -> MiningData;
}