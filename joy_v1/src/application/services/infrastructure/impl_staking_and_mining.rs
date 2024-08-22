use near_sdk::{collections::LookupMap, env, json_types::U128, near_bindgen, AccountId};

use crate::models::{
    contract::{JoychiV1, JoychiV1Ext, JoychiV1StorageKey}, ft_request::external::cross_ft, item_factory::{ItemFeature, ItemType}, pet::PetFeature, staking_and_mining::{NFTInfo, PoolInfo, PoolMetadata, StakingAndMining}, PetId, PoolId
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
        let item = self.item_metadata_by_id.get(&tool_id).unwrap();
        let item_type = item.prototype_item_type;
        let account_id = env::signer_account_id();
        let mut mining_tool_used = self.mining_tool_used.get(&env::signer_account_id()).unwrap_or_default();
        assert!(item_type == ItemType::MineTool, "This item is not a mining tool");        
        
        assert!(self.mining_tool_owner.get(&tool_id).unwrap() != account_id, "Mining tool is already used");
        assert!(self.is_lock_item.get(&tool_id).unwrap() == false, "This tool is locked");

        self.is_lock_item.insert(&tool_id, &true);
        let mining_power = self.item_metadata_by_id.get(&tool_id).unwrap().prototype_itemmining_power;
        
        if (mining_tool_used.is_empty()) {
            mining_tool_used.push(tool_id as u128);
            self.mining_tool_used.insert(&account_id, &mining_tool_used);
            self.last_mining_time.insert(&account_id, &(0 as u128));
            self.total_mining_power.insert(&account_id, &(0 as u128));
        } else {
            assert!(mining_tool_used.len() < 3, "You have reached the maximum mining tool");
            mining_tool_used.push(tool_id as u128);
            self.mining_tool_used.insert(&account_id, &mining_tool_used);
        }

        // total Mining charge time

        self.total_mining_power.insert(&account_id, &(self.total_mining_power.get(&account_id).unwrap() + mining_power));

        self.mining_tool_owner.insert(&tool_id, &account_id); 

        let mut last_time_mining = self.last_mining_time.get(&account_id).unwrap_or_default();

        if last_time_mining == 0 || self.mining_tool_used.get(&account_id).unwrap().len() == 1 {
            last_time_mining = env::block_timestamp() as u128;
            self.last_mining_time.insert(&account_id, &last_time_mining);

        }


    }

    fn remove_mining_tool(&mut self, tool_id: u64) {
        let account_id = env::signer_account_id();
        let mut mining_tool_owner = self.mining_tool_owner.get(&tool_id).unwrap();
        let mut item = self.item_metadata_by_id.get(&tool_id).unwrap();
        assert!(mining_tool_owner == account_id, "You are not the owner of this tool");

        let mut total_mining_power = self.total_mining_power.get(&account_id).unwrap();
        total_mining_power -= item.prototype_itemmining_power;
        self.total_mining_power.insert(&account_id, &total_mining_power);

        let mut null_account = "null_account_joy_v2.testnet".to_string();
        mining_tool_owner = AccountId::try_from(null_account.clone()).expect("Invalid Account ID");
        self.mining_tool_owner.insert(&tool_id, &mining_tool_owner);

        // removeItemFromListTool TODO
        // calculate charge time TODO

        self.is_lock_item.insert(&tool_id, &false);

    }

    fn mining(&mut self) {
        let account_id = env::signer_account_id();
        assert!(self.total_mining_power.get(&account_id).unwrap() > 0, "You do not have any mining tool");

        let total_mining_charge_of_time = (1000 * self.total_mining_charge_time.get(&account_id).unwrap()) / 1000 as u128; // TODO

        assert!(self.last_mining_time.get(&account_id).unwrap() + total_mining_charge_of_time > env::block_timestamp() as u128, "You need to wait for the mining tool to be charged");
        self.last_mining_time.insert(&account_id, &(env::block_timestamp() as u128));

        let total_points_mined = (1000 * self.total_mining_power.get(&account_id).unwrap()) / 1000 as u128;

        self.mining_points.insert(&account_id, &(self.mining_points.get(&account_id).unwrap() + total_points_mined));
    }

    fn redemn_mining_points(&mut self) {
        let account_id = env::signer_account_id();
        let mut mining_points = self.mining_points.get(&account_id).unwrap();

        assert!(mining_points > self.points_used_per_redemn, "You do not have enough mining point");
        self.mining_points.insert(&account_id, &(mining_points - &self.points_used_per_redemn));

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
        let mut user_list_mining_tool = self.mining_tool_used.get(&account_id).unwrap();
        let last_tool = user_list_mining_tool.last().unwrap();

        for i in 0..user_list_mining_tool.len() {
            if user_list_mining_tool[i] == *last_tool {
                    user_list_mining_tool.pop();
                    self.mining_tool_used.insert(&account_id, &user_list_mining_tool);
                    break;
                }
        }
    }
}