use near_sdk::{collections::LookupMap, env, json_types::U128, near_bindgen};

use crate::models::{
    contract::{JoychiV1, JoychiV1Ext, JoychiV1StorageKey}, ft_request::external::cross_ft, item_factory::{ItemFeature, ItemType}, pet::PetFeature, staking_and_mining::{NFTInfo, PoolInfo, PoolMetadata, StakingAndMining}, PetId, PoolId
};

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
        
        pool

    }
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
            .ft_transfer(pet.owner_id.clone(), U128::from(pool.price_per_slot), None);

        self.pet_metadata_by_id.insert(&pet_id, &pet);

    }

    fn add_mining_tool(&mut self, tool_id: u64) {
        let item = self.item_metadata_by_id.get(&tool_id).unwrap();
        let itemType = item.prototype_item_type;
        let account_id = env::signer_account_id();
        let mut mining_tool_used = self.mining_tool_used.get(&env::signer_account_id()).unwrap();
        assert!(itemType == ItemType::MineTool, "This item is not a mining tool");
        assert!(mining_tool_used.len() < 3, "You have reached the maximum mining tool");
        assert!(self.mining_tool_owner.get(&tool_id).unwrap() != account_id, "Mining tool is already used");
        assert!(self.is_lock_item.get(&tool_id).unwrap() == false, "This tool is locked");

        self.is_lock_item.insert(&tool_id, &true);
        let mining_power = self.item_metadata_by_id.get(&tool_id).unwrap().prototype_itemmining_power;
        
        mining_tool_used.push(tool_id as u128);
        self.mining_tool_used.insert(&account_id, &mining_tool_used);

        // total Mining charge time

        let mut total_minging_power = self.total_mining_power.get(&account_id).unwrap();
        total_minging_power += mining_power;
        self.total_mining_power.insert(&account_id, &total_minging_power);

        let mut mining_tool_owner = self.mining_tool_owner.get(&tool_id).unwrap();
        self.mining_tool_owner.insert(&tool_id, &account_id); 

        let mut last_time_mining = self.last_mining_time.get(&account_id).unwrap();

        if last_time_mining == 0 || mining_tool_used.len() == 1 {
            last_time_mining = env::block_timestamp() as u128;
        }

    }


}