use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    serde::{Deserialize, Serialize},
    AccountId,
};

use super::ItemId;

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Clone)]
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
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub enum ItemRarity {
    Common,
    Rare,
    Legendary,
    Epic,
    MineTool,
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub enum ItemType {
    Normal,
    MineTool,
}

pub trait ItemFeature {
    fn create_item(
        &mut self,
        prototype_item_image: String,
        prototype_item_type: ItemType,
        prototype_item_cooldown_breed_time: u128,
        prototype_item_reduce_breed_fee: u128,
        prototype_item_points: u128,
        prototype_item_rarity: ItemRarity,
        prototype_itemmining_power: u128,
        prototype_itemmining_charge_time: u128,
    ) -> ItemMetadata;

    fn edit_item(
        &mut self,
        item_id: ItemId,
        prototype_item_image: String,
        prototype_item_cooldown_breed_time: u128,
        prototype_item_reduce_breed_fee: u128,
        prototype_item_points: u128,
        prototype_item_rarity: ItemRarity,
        prototype_itemmining_power: u128,
        prototype_itemmining_charge_time: u128,
    );

    fn mint_item_for_user(&mut self, to_addr: AccountId, item_id: ItemId);
}

pub trait ItemEnum {
    fn get_all_item_metadata(&self, start: Option<u32>, limit: Option<u32>) -> Vec<ItemMetadata>;

    fn get_item_by_item_id(&self, item_id: ItemId) -> ItemMetadata;
}