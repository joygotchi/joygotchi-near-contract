use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    serde::{Deserialize, Serialize},
};

use super::ItemId;

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Clone)]
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

pub trait ItemImmidiateFeature {
    fn create_item_immidiate(
        &mut self,
        name: String,
        price: u128,
        points: u128,
        time_extension: u128,
        price_delta: u128,
        stock: u128,
        shield: u128,
        is_revival: bool,
    ) -> ItemImmidiateMetadata;

    fn edit_item_immidiate(
        &mut self,
        item_id: ItemId,
        name: String,
        price: u128,
        points: u128,
        time_extension: u128,
        price_delta: u128,
        stock: u128,
        shield: u128,
        is_revival: bool,
    );
}

pub trait ItemImmidiateEnum {
    fn get_all_item_immidiate_metadata(
        &self,
        start: Option<u32>,
        limit: Option<u32>,
    ) -> Vec<ItemImmidiateMetadata>;

    fn get_item_immidiate_by_item_id(&self, item_id: ItemId) -> ItemImmidiateMetadata;
}
