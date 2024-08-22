use near_sdk::{collections::LookupMap, env, json_types::U128, near_bindgen, AccountId};

use crate::models::{
    contract::{JoychiV1, JoychiV1Ext, JoychiV1StorageKey}, ft_request::external::cross_ft, item_factory::{ItemFeature, ItemType}, pet::PetFeature, staking_and_mining::{MiningData, NFTInfo, PoolInfo, PoolMetadata, StakingAndMining}, PetId, PoolId
};
pub const ATTACHED_TRANSFER_FT: u128 = 1;

use super::impl_pet::GAS_FOR_CROSS_CALL;

#[near_bindgen]
impl StakingAndMining for JoychiV1 {

    fn create_new_staking_pool(&mut self, name: String, reward_nft_ids: Vec<u128>, staking_start_time: u128, staking_end_time: u128, max_slot_in_pool: u128, token_reward_per_slot: u128, max_slot_per_wallet: u128) -> PoolMetadata {
        let new_pool = PoolInfo {
            name: name,
            reward_nft_ids: reward_nft_ids,
            staking_start_time: staking_start_time,
            staking_end_time: staking_end_time,
            max_slot_in_pool: max_slot_in_pool,
            token_reward_per_slot: token_reward_per_slot,
            max_slot_per_wallet: max_slot_per_wallet,
            total_staked_slot: 0
        };

        let num_pool = self.all_pool_id.len();

        let new_pool_metadata = PoolMetadata {
            pool_id: num_pool + 1,
            price_per_slot: 37,
            pool_info: new_pool,
            staked_pets: Vec::new(),
        };

        self.pool_metadata_by_id.insert(&(&num_pool + 1), &new_pool_metadata);
        self.all_pool_id.insert(&(&num_pool + 1));

        new_pool_metadata

    }

    fn stake(&mut self, pet_id: PetId, pool_id: PoolId) -> PoolMetadata {
        let mut pet = self.pet_metadata_by_id.get(&pet_id).unwrap();
        let mut pool = self.pool_metadata_by_id.get(&pool_id).unwrap();
        let account_id = env::signer_account_id();

        assert!(pool.pool_id == pool_id, "Invalid pool id");
        assert!(pool.pool_info.staking_start_time < env::block_timestamp() as u128, "Staking pool has started");
        assert!(
           account_id == pet.owner_id,
            "You're not owner this pet"
        );

        assert!(self.is_pet_alive(pet_id), "Your pet is dead, you cannot stake it");
        assert!(pet.is_lock == false, "Your pet is locked, you cannot stake it");
        assert!(pool.pool_info.total_staked_slot < pool.pool_info.max_slot_in_pool , "Staking pool is full");

        pet.is_lock = true;

        let mut inner_map = self.user_staked_pet_count.get(&account_id).unwrap_or_else(|| {
            LookupMap::new(JoychiV1StorageKey::UserStakedPetCountInner { account_id: account_id.clone() })
        });

        let current_count = inner_map.get(&pool_id).unwrap_or(0);
        inner_map.insert(&pool_id, &(current_count + 1));
        self.user_staked_pet_count.insert(&account_id, &inner_map);

        pool.pool_info.total_staked_slot += 1;

        let nft_info = NFTInfo {
            nft_id: pet_id as u128,
            owner: account_id
        };

        pool.staked_pets.push(nft_info);
        
        self.pool_metadata_by_id.insert(&pool_id, &pool);
        self.pet_metadata_by_id.insert(&pet_id, &pet);
        
        pool

    }
    
    #[payable]
    fn un_stake(&mut self, pet_id: PetId, pool_id: PoolId) {
        let mut pet = self.pet_metadata_by_id.get(&pet_id).unwrap();
        let pool = self.pool_metadata_by_id.get(&pool_id).unwrap();
        let account_id = env::signer_account_id();
        let mut is_staked_pet_owner = false;
        for owner in pool.staked_pets {
            if owner.owner == account_id && owner.nft_id == pet_id as u128 {
                is_staked_pet_owner = true;
            }
        }

        assert!(is_staked_pet_owner, "Pet not staked");

        assert!(pool.pool_id == pool_id, "Invalid pool id");
        assert!(pool.pool_info.staking_end_time < env::block_timestamp() as u128, "Staking pool has not ended yet");
        assert!(
           account_id == pet.owner_id,
            "You're not owner this pet"
        );

        assert!(self.is_pet_alive(pet_id), "Your pet is dead, you cannot stake it");
        assert!(pet.is_lock == true, "Your pet is not locked, you cannot unstake it");

        pet.is_lock = false;

        for reward in pool.pool_info.reward_nft_ids {
            self.mint_item_for_user(account_id.clone(), reward as u64);
        }

        cross_ft::ext(self.ft_address.to_owned())
            .with_static_gas(GAS_FOR_CROSS_CALL)
            .with_attached_deposit(ATTACHED_TRANSFER_FT)
            .ft_transfer(pet.owner_id.clone(), U128::from(pool.price_per_slot), None);

        self.pet_metadata_by_id.insert(&pet_id, &pet);

    }

    fn add_mining_tool(&mut self, tool_id: u64) {
        let account_id = env::signer_account_id();
        let mut item = self.item_metadata_by_id.get(&tool_id).unwrap();

        let item_type = item.prototype_item_type.clone();
        assert!(item_type == ItemType::MineTool, "This item is not a mining tool");        

        let mining_data_option = self.mining_data_by_account_id.get(&account_id);

        let mut mining_data = mining_data_option.clone().unwrap_or_default();
        if (mining_data_option.is_none()) {
            let mut mining_used = Vec::new() as Vec<u128>;
            mining_used.push(tool_id as u128);
            mining_data = MiningData {
                account_id: Some(account_id.clone()),
                mining_points: 0,
                total_mining_power: 0,
                total_mining_charge_time: 0,
                last_mining_time: 0,
                mining_tool_used: mining_used,
            };
        } else {

            assert!(mining_data.account_id.clone().unwrap() != account_id, "You're not owned");
            assert!(self.item_metadata_by_id.get(&tool_id).unwrap().is_lock == false, "This tool is locked");
            assert!(mining_data.mining_tool_used.len() < 3, "You have reached the maximum mining tool");
            
            item.is_lock = true;
            self.item_metadata_by_id.insert(&tool_id, &item.clone());

            let mining_power = self.item_metadata_by_id.get(&tool_id).unwrap().prototype_itemmining_power;
            mining_data.mining_tool_used.push(tool_id as u128);
            

            if mining_data.last_mining_time == 0 || mining_data.mining_tool_used.len() == 1 {
                let last_time_mining = env::block_timestamp() as u128;
                mining_data.last_mining_time += last_time_mining;
            }
            mining_data.total_mining_power += mining_power;
        }

        self.mining_data_by_account_id.insert(&account_id, &mining_data);
        
    }

    fn remove_mining_tool(&mut self, tool_id: u64) {
        let account_id = env::signer_account_id();
        let mut item = self.item_metadata_by_id.get(&tool_id).unwrap();
        assert!(item.owner == account_id, "You are not the owner of this tool");

        let mut mining_data = self.mining_data_by_account_id.get(&account_id).unwrap();

        if mining_data.total_mining_power > item.prototype_itemmining_power {
            mining_data.total_mining_power -= item.prototype_itemmining_power;
            mining_data.mining_tool_used.pop();
            if let Some(pos) = mining_data.mining_tool_used.iter().position(|&x| x == tool_id as u128) {
                mining_data.mining_tool_used.remove(pos);
            }
            item.is_lock = true;
        }
        self.mining_data_by_account_id.insert(&account_id, &mining_data);
        self.item_metadata_by_id.insert(&tool_id, &item);

    }

    fn mining(&mut self) {
        let account_id = env::signer_account_id();
        let mut mining_data = self.mining_data_by_account_id.get(&account_id).unwrap();
        assert!(mining_data.total_mining_power > 0, "You do not have any mining tool");

        let total_mining_charge_of_time = (1000 * mining_data.total_mining_charge_time) / 1000 as u128; // TODO

        assert!(mining_data.last_mining_time + total_mining_charge_of_time > env::block_timestamp() as u128, "You need to wait for the mining tool to be charged");
        mining_data.last_mining_time = env::block_timestamp() as u128;

        let total_points_mined = (10000 * mining_data.total_mining_power) / 1000 as u128;

        mining_data.mining_points += total_points_mined;

        self.mining_data_by_account_id.insert(&account_id, &mining_data);
    }

    fn redemn_mining_points(&mut self) {
        let account_id = env::signer_account_id();
        let mut mining_data = self.mining_data_by_account_id.get(&account_id).unwrap();
 
        let mut mining_points = mining_data.mining_points;

        assert!(mining_points > self.points_used_per_redemn, "You do not have enough mining point");
        mining_points -= &self.points_used_per_redemn;

        // Mint New Item TODO

        cross_ft::ext(self.ft_address.to_owned())
            .with_static_gas(GAS_FOR_CROSS_CALL)
            .with_attached_deposit(ATTACHED_TRANSFER_FT)
            .ft_transfer(account_id.clone(), U128::from(self.token_earned_per_redemn), None);

    }

    fn owner_withdraw_redundant_token(&mut self, pool_id: PoolId) {
        let account_id = env::signer_account_id();
        let mut pool = self.pool_metadata_by_id.get(&pool_id).unwrap();
        assert!(pool.pool_info.staking_end_time < (env::block_timestamp() as u128),  "Staking pool has not ended yet");

        let total_tokens_for_reward = pool.pool_info.token_reward_per_slot * pool.pool_info.max_slot_in_pool;
        let token_distributed = pool.pool_info.token_reward_per_slot * pool.pool_info.total_staked_slot;

        let redundant_token = total_tokens_for_reward - token_distributed;

        cross_ft::ext(self.ft_address.to_owned())
            .with_static_gas(GAS_FOR_CROSS_CALL)
            .with_attached_deposit(ATTACHED_TRANSFER_FT)
            .ft_transfer(account_id.clone(), U128::from(redundant_token), None);
    }

    fn configure_mining_pool(&mut self, name: String, mining_power_multiplier: u128, charge_of_time_multiplier: u128) {
        self.mining_pool_name = name;
        self.mining_power_multiplier = mining_power_multiplier;
        self.charge_of_time_multiplier = charge_of_time_multiplier;
    }

    fn set_mining_points_used_per_redemn(&mut self, points: u128) {
        self.points_used_per_redemn = points;
    }

    fn set_token_earned_per_redemn(&mut self, token: u128) {
        self.token_earned_per_redemn = token;
    }

    fn set_price_per_slot(&mut self, price_per_slot: u128) {
        self.price_per_slot = price_per_slot;
    }

    fn caculate_charge_of_time(&mut self, number_of_tools: u128) {
        if number_of_tools == 1 {
            // let charge_of_time = self.mining
        }
    }

    fn remove_item_from_list_tool(&mut self, value: u128) {
        let account_id = env::signer_account_id();
        let mut mining_data = self.mining_data_by_account_id.get(&account_id).unwrap();

        let mut user_list_mining_tool = mining_data.mining_tool_used.clone();
        let last_tool = user_list_mining_tool.last().unwrap();

        for i in 0..mining_data.mining_tool_used.len() {
            if mining_data.mining_tool_used[i] == *last_tool {
                    mining_data.mining_tool_used.pop();
                    self.mining_data_by_account_id.insert(&account_id, &mining_data);
                    break;
                }
        }
    }
}