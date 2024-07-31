use near_sdk::{env, near_bindgen, AccountId};

use crate::models::{
    contract::{JoychiV1, JoychiV1Ext}, pet::PetFeature, staking_and_mining::{PetCountUser, PoolInfo, PoolMetadata, StakingAndMining}
};

use super::impl_pet::{ATTACHED_DEPOSIT_NFT, GAS_FOR_CROSS_CALL};

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
            user_staked_pet_count: Vec::new(),
            staked_pets: Vec::new(),
        };

        self.pool_metadata_by_id.insert(&(&num_pool + 1), &new_pool_metadata);
        self.all_pool_id.insert(&(&num_pool + 1));

        new_pool_metadata

    }

    fn stake(&mut self, pet_id: PetId, pool_id: PoolId) {
        let mut pet = self.pet_metadata_by_id.get(&pet_id).unwrap();
        let mut pool = self.pool_metadata_by_id.get(&pool_id).unwrap();

        assert!(pool.pool_info.staking_start_time < env::block_timestamp(), "Staking pool has started");
        assert!(
            env::signer_account_id() == pet.owner_id,
            "You're not owner this pet"
        );

        assert!(self.is_pet_alive(pet_id), "Your pet is dead, you cannot stake it");
        assert!(pet.is_lock == false, "Your pet is locked, you cannot stake it");
        assert!(pool.total_staked_slot < pool.max_slot_in_pool , "Staking pool is full");

        pet.is_lock = true;

        let peet_count = 0;

        for user_count in 

        let user_count_info = PetCountUser {
            user: pet.owner_id,
            // pet_count: pool.user_staked_pet_count.
        };
        pool.user_staked_pet_count;


    }
    fn un_stake(&mut self, pet_id: PetId, pool_id: PoolId) {

    }
}