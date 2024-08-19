use near_sdk::near_bindgen;

use crate::models::{
    contract::{JoychiV1, JoychiV1Ext},
    item_factory::{ItemEnum, ItemMetadata},
    ItemId,
};

#[near_bindgen]
impl ItemEnum for JoychiV1 {
    fn get_all_item_metadata(&self, start: Option<u32>, limit: Option<u32>) -> Vec<ItemMetadata> {
        self.all_item_id
            .iter()
            .skip(start.unwrap_or(0) as usize)
            .take(limit.unwrap_or(20) as usize)
            .map(|x| self.item_metadata_by_id.get(&x).unwrap())
            .collect()
    }

    fn get_item_by_item_id(&self, item_id: ItemId) -> ItemMetadata {
        let item = self.item_metadata_by_id.get(&item_id).unwrap();

        item
    }
}
