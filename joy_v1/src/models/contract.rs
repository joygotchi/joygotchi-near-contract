use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::BorshStorageKey;
use near_sdk::{
    collections::{LookupMap, UnorderedSet},
    json_types::Base64VecU8,
    near_bindgen,
    serde::{Deserialize, Serialize},
    AccountId, PanicOnDefault,
};

use super::item_factory::ItemMetadata;
use super::pet::{PetEvolution, PetSpecies};
use super::staking_and_mining::PoolMetadata;
use super::{PetSpeciesId, PoolId};
use super::{item_immidiate::ItemImmidiateMetadata, pet::PetMetadata, BattleId, ItemId, PetId};

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct JoychiV1Metadata {
    /// Specification associated with the joygotchi contract.
    pub spec: String,

    /// Name of the joygotchi contract.
    pub name: String,

    /// Symbol associated with the joygotchi contract.
    pub symbol: String,

    /// Optional icon for the joygotchi contract.
    pub icon: Option<String>,

    /// Optional base URI for the joygotchi contract.
    pub base_uri: Option<String>,

    /// Optional reference string for the joygotchi contract.
    pub reference: Option<String>,

    /// Optional hash of the reference, encoded in base64.
    pub reference_hash: Option<Base64VecU8>,
}

#[near_bindgen]
#[derive(PanicOnDefault, BorshDeserialize, BorshSerialize)]
pub struct JoychiV1 {
    /// Account ID of the owner of the contract.  
    pub owner_id: AccountId,

    pub nft_address: AccountId,

    pub nft_item_address: AccountId,

    pub manager_address: AccountId,

    pub total_score: u128,

    pub ft_address: AccountId,

    pub all_item_immidiate_id: UnorderedSet<ItemId>,

    pub item_immidiate_metadata_by_id: LookupMap<ItemId, ItemImmidiateMetadata>,

    pub all_item_id: UnorderedSet<ItemId>,

    pub item_metadata_by_id: LookupMap<ItemId, ItemMetadata>,

    pub all_pet_id: UnorderedSet<PetId>,

    pub pet_metadata_by_id: LookupMap<PetId, PetMetadata>,

    pub all_battle_id: UnorderedSet<BattleId>,

    pub battle_metadata_by_id: LookupMap<BattleId, BattleMetadata>,

    pub all_pet_species_id: UnorderedSet<PetSpeciesId>,

    pub pet_species_metadata_by_id: LookupMap<PetSpeciesId, PetSpecies>,

    pub pet_evolution_metadata_by_id: LookupMap<PetId, Vec<PetEvolution>>,

    pub pool_metadata_by_id: LookupMap<PoolId, PoolMetadata>,

    pub all_pool_id: UnorderedSet<PoolId>,

    pub user_staked_pet_count: LookupMap<AccountId, LookupMap<PoolId, u64>>,

}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct BattleMetadata {
    pub battle_id: BattleId,

    pub winner: PetId,

    pub attacker: PetId,

    pub loser: PetId,

    pub time: u64,
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub enum Status {
    HAPPY,
    HUNGRY,
    STARVING,
    DYING,
}

#[derive(BorshSerialize, BorshStorageKey)]
pub enum JoychiV1StorageKey {
    AllItemImmidiateId,
    ItemImmidiateMetadataById,
    AllItemId,
    ItemMetadataById,
    AllPetId,
    PetMetadataById,
    AllBattleId,
    BattleMetadataById,
    AllPetSpeciesId,
    PetSpeciesMetadataById,
    PetEvolutionMetadataById,
    PoolMetadataById,
    AllPoolId,
    UserStakedPetCountOuter,
    UserStakedPetCountInner { account_id: AccountId },
}
