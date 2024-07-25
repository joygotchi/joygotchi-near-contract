use crate::models::nft_request::external::TokenMetadata;
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    serde::{Deserialize, Serialize},
    AccountId,
};

use super::{
    contract::{BattleMetadata, Status},
    item::ItemMetadata,
    nft_request::external::PetAttribute,
    BattleId, ItemId, PetId, PetSpeciesId,
};

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct PetMetadata {
    pub pet_id: PetId,
    pub name: String,
    pub owner_id: AccountId,
    pub time_pet_born: u128,
    pub time_until_starving: u128,
    pub items: Vec<ItemMetadata>,
    pub score: u128,
    pub level: u128,
    pub status: Status,
    pub star: u64,
    pub reward_debt: u128,
    pub pet_species: u128,
    pub pet_shield: u128,
    pub last_attack_used: u128,
    pub last_attacked: u128,
    pub pet_evolution_item_id: u128,
    pub pet_need_evolution_item: bool,
    pub pet_has_evolution_item: bool,
    pub pet_evolution_phase: u128,
    pub extra_permission: Vec<AccountId>,
    pub category: String,
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct PetSpecies {
    pub species_id: PetSpeciesId,
    pub species_name: String,
    pub need_evolution_item: bool,
    pub evolution_item_id: u128,
    pub pet_evolution: Vec<PetEvolution>,
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct PetEvolution {
    pub image: String,
    pub name: String,
    pub attack_win_rate: u128,
    pub next_evolution_level: u128,
}

pub trait PetFeature {
    fn set_manager(&mut self, manager_addr: AccountId);

    fn create_pet(&mut self, name: String) -> PetMetadata;

    fn change_name_pet(&mut self, pet_id: PetId, name: String);

    // fn buy_item(&mut self, pet_id: PetId, item_id: ItemId);

    fn attack(&mut self, from_id: PetId, to_id: PetId) -> BattleMetadata;

    fn kill_pet(&mut self, pet_kill: PetId, pet_receive: PetId);

    fn level_pet(&mut self, pet_id: PetId) -> u128;

    fn is_pet_alive(&mut self, pet_id: PetId) -> bool;

    fn create_species(
        &mut self,
        need_evol_item: bool,
        evol_item_id: u128,
        name_spec: String,
        pet_evolution: Vec<PetEvolution>,
    );

    fn redeem(&mut self, pet_id: PetId, to_addr: AccountId);

    fn token_uri(&mut self, pet_id: PetId) -> PetAttribute;

    fn check_role_update_pet(&self, pet_id: PetId, user_id: AccountId) -> bool;

    fn add_access_update_pet(&mut self, pet_id: PetId, user_id: AccountId) -> PetMetadata;

    // Update pet attribute by delegated user
    fn delegate_update_attribute(&mut self, pet_id: PetId, pet_attribute: PetAttribute);

    // Update nft token metadata
    fn delegate_update_metadata(&mut self, pet_id: PetId, token_metadata: TokenMetadata);

    fn check_evol_pet_if_needed(&mut self, pet_id: PetId);
}

pub trait PetEnum {
    fn get_all_pet_metadata(&self, start: Option<u32>, limit: Option<u32>) -> Vec<PetMetadata>;

    fn get_pet_by_pet_id(&self, pet_id: PetId) -> PetMetadata;

    fn get_all_battle_metadata(
        &self,
        start: Option<u32>,
        limit: Option<u32>,
    ) -> Vec<BattleMetadata>;

    fn get_battle_by_pet_id(&self, battle_id: BattleId) -> BattleMetadata;

    fn get_status_pet(&self, pet_id: PetId) -> Status;

    fn get_pet_evolution_item(&self, pet_id: PetId) -> PetEvolution;

    fn get_pet_attack_winrate(&self, pet_id: PetId) -> u128;

    fn get_pet_image(&self, pet_id: PetId) -> String;

    fn get_pet_evolution_phase(&self, pet_id: PetId, current_evo_phase: u128) -> u128;
}
