use models::contract::{JoychiV1, JoychiV1Ext, JoychiV1StorageKey};
use near_sdk::borsh::BorshSerialize;
use near_sdk::{
    collections::{LookupMap, UnorderedSet},
    env, near_bindgen, AccountId,
};

pub mod application;
pub mod models;

#[near_bindgen]
impl JoychiV1 {
    #[init]
    pub fn init(nft_addr: AccountId, nft_item_addr: AccountId, ft_addr: AccountId) -> Self {
        let owner_id = env::signer_account_id();

        Self::new(owner_id, nft_addr, nft_item_addr, ft_addr)
    }

    #[init]
    pub fn new(
        owner_id: AccountId,
        nft_addr: AccountId,
        nft_item_addr: AccountId,
        ft_addr: AccountId,
    ) -> Self {
        Self {
            owner_id,
            nft_address: nft_addr,
            nft_item_address: nft_item_addr,
            manager_address: env::signer_account_id(),
            total_score: 0,
            ft_address: ft_addr,
            all_item_id: UnorderedSet::new(JoychiV1StorageKey::AllItemId.try_to_vec().unwrap()),
            item_metadata_by_id: LookupMap::new(
                JoychiV1StorageKey::ItemMetadataById.try_to_vec().unwrap(),
            ),
            all_pet_id: UnorderedSet::new(JoychiV1StorageKey::AllPetId.try_to_vec().unwrap()),
            pet_metadata_by_id: LookupMap::new(
                JoychiV1StorageKey::PetMetadataById.try_to_vec().unwrap(),
            ),
            all_battle_id: UnorderedSet::new(JoychiV1StorageKey::AllBattleId.try_to_vec().unwrap()),
            battle_metadata_by_id: LookupMap::new(
                JoychiV1StorageKey::BattleMetadataById.try_to_vec().unwrap(),
            ),
            all_pet_species_id: UnorderedSet::new(
                JoychiV1StorageKey::AllPetSpeciesId.try_to_vec().unwrap(),
            ),
            pet_species_metadata_by_id: LookupMap::new(
                JoychiV1StorageKey::PetSpeciesMetadataById
                    .try_to_vec()
                    .unwrap(),
            ),
            pet_evolution_metadata_by_id: LookupMap::new(
                JoychiV1StorageKey::PetEvolutionMetadataById
                    .try_to_vec()
                    .unwrap(),
            ),
        }
    }
}
