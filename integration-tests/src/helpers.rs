
use near_sdk::json_types::Base64VecU8;
use near_sdk::AccountId;
use std::collections::HashMap;
use near_token::NearToken;
use near_workspaces::{Account, Contract};
use serde::{Deserialize, Serialize};
use serde_json::json;

pub type ItemId = u64;
pub type PetId = u64;

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct TokenMetadata {
    pub title: Option<String>, // ex. "Arch Nemesis: Mail Carrier" or "Parcel #5055"
    pub description: Option<String>, // free-form description
    pub media: Option<String>, // URL to associated media, preferably to decentralized, content-addressed storage
    pub media_hash: Option<Base64VecU8>, // Base64-encoded sha256 hash of content referenced by the `media` field. Required if `media` is included.
    pub copies: Option<u64>, // number of copies of this set of metadata in existence when token was minted.
    pub issued_at: Option<u64>, // When token was issued or minted, Unix epoch in milliseconds
    pub expires_at: Option<u64>, // When token expires, Unix epoch in milliseconds
    pub starts_at: Option<u64>, // When token starts being valid, Unix epoch in milliseconds
    pub updated_at: Option<u64>, // When token was last updated, Unix epoch in milliseconds
    pub extra: Option<String>, // anything extra the NFT wants to store on-chain. Can be stringified JSON.
    pub reference: Option<String>, // URL to an off-chain JSON file with more info.
    pub reference_hash: Option<Base64VecU8>, // Base64-encoded sha256 hash of JSON from reference field. Required if `reference` is included.
}


#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct ItemImmidiateMetadata {
    pub item_id: ItemId,
    pub name: String,
    pub points: u128,
    pub price: u128,
    pub price_delta: u128,
    pub stock: u128,
    pub shield: u128,
    pub time_extension: u128,
    pub is_revival: bool,
}


#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct PetEvolution {
    pub image: String,
    pub name: String,
    pub attack_win_rate: u128,
    pub next_evolution_level: u128,
}


#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub enum Status {
    HAPPY,
    HUNGRY,
    STARVING,
    DYING,
}


#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct PetMetadata {
    pub pet_id: PetId,
    pub name: String,
    pub owner_id: AccountId,
    pub time_pet_born: u128,
    pub time_until_starving: u128,
    pub items: Vec<ItemImmidiateMetadata>,
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
    pub is_lock: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct PetAttribute {
    pub pet_name: String,
    pub image: String,
    pub score: u128,
    pub level: u128,
    pub status: Status,
    pub star: u64,
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct ItemMetadata {
    pub item_id: ItemId,
    pub item_rarity_amount: u128,
    pub list_prototype_items_of_rarity: Vec<u128>,
    pub prototype_item_image: String,
    pub prototype_item_type: ItemType,
    pub prototype_item_cooldown_breed_time: u128,
    pub prototype_item_reduce_breed_fee: u128,
    pub prototype_item_points: u128,
    pub prototype_item_rarity: ItemRarity,
    pub prototype_itemmining_power: u128,
    pub prototype_itemmining_charge_time: u128,
    pub owner: AccountId,
    pub is_lock: bool,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub enum ItemRarity {
    Common,
    Rare,
    Legendary,
    Epic,
    MineTool,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub enum ItemType {
    Normal,
    MineTool,
}


#[derive(Deserialize, Serialize, Clone, Default, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct MiningData {
    pub account_id: Option<AccountId>,
    pub mining_points: u128,
    pub total_mining_power: u128,
    pub total_mining_charge_time: u128,
    pub last_mining_time: u128,
    pub mining_tool_used: Vec<u128>,
}


pub async fn storage_deposit(
    owner: &Account,
    ft_contract: &Contract,
    user: &Account,
) -> anyhow::Result<()> {
    //Register owner storage deposit ft_contract
    let default_deposit = NearToken::from_millinear(8);
    owner
        .call(ft_contract.id(), "storage_deposit")
        .args_json(serde_json::json!({
            "account_id": user.id()
        }))
        .deposit(default_deposit)
        .transact()
        .await?
        .into_result()?;
    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct JsonToken {
    //token ID
    pub token_id: String,
    //owner of the token
    pub owner_id: AccountId,
    //token metadata
    pub metadata: TokenMetadata,
    //list of approved account IDs that have access to transfer the token. This maps an account ID to an approval ID
    pub approved_account_ids: HashMap<AccountId, u64>,
}

pub async fn get_pet_metadata_by_id(
    user: &Account,
    pet_id: PetId,
    joychi_contract: &Contract,
) -> anyhow::Result<PetMetadata> {
    let pet: PetMetadata = user
        .call(joychi_contract.id(), "get_pet_by_pet_id")
        .args_json(json!({
            "pet_id": pet_id
        }))
        .transact()
        .await?
        .json()?;

    Ok(pet)
}

pub async fn get_item_immidiate_metadata_by_id(
    user: &Account,
    item_id: ItemId,
    joychi_contract: &Contract,
) -> anyhow::Result<ItemImmidiateMetadata> {
    let item: ItemImmidiateMetadata = user
        .call(joychi_contract.id(), "get_item_immidiate_by_item_id")
        .args_json(json!({
            "item_id": item_id
        }))
        .transact()
        .await?
        .json()?;

    Ok(item)
}

pub async fn get_item_prototype_metadata_by_id(
    user: &Account,
    item_id: ItemId,
    joychi_contract: &Contract,
) -> anyhow::Result<ItemMetadata> {
    let item: ItemMetadata = user
        .call(joychi_contract.id(), "get_item_by_item_id")
        .args_json(json!({
            "item_id": item_id
        }))
        .transact()
        .await?
        .json()?;

    Ok(item)
}

pub async fn get_score_pet_by_id(
    user: &Account,
    pet_id: PetId,
    joychi_contract: &Contract,
) -> anyhow::Result<u128> {
    let pet = get_pet_metadata_by_id(user, pet_id, joychi_contract).await?;

    Ok(pet.score)
}

pub async fn get_level_pet_by_id(
    user: &Account,
    pet_id: PetId,
    joychi_contract: &Contract,
) -> anyhow::Result<u128> {
    let pet = get_pet_metadata_by_id(user, pet_id, joychi_contract).await?;

    Ok(pet.level)
}


pub async fn get_mining_data_by_id(
    user: &Account,
    joychi_contract: &Contract,
) -> anyhow::Result<MiningData> {
    let pet: MiningData = user
        .call(joychi_contract.id(), "get_mining_data_by_account_id")
        .args_json(json!({
            "account_id": user.id()
        }))
        .transact()
        .await?
        .json()?;

    Ok(pet)
}