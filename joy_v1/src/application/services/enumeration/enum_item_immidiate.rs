use near_sdk::near_bindgen;

use crate::models::{
    contract::{JoychiV1, JoychiV1Ext},
    item_immidiate::{ItemEnum, ItemImmidiateMetadata},
    ItemId,
};

#[near_bindgen]
impl ItemEnum for JoychiV1 {
    fn get_all_item_immidiate_metadata(
        &self,
        start: Option<u32>,
        limit: Option<u32>,
    ) -> Vec<ItemImmidiateMetadata> {
        self.all_item_immidiate_id
            .iter()
            .skip(start.unwrap_or(0) as usize)
            .take(limit.unwrap_or(20) as usize)
            .map(|x| self.item_immidiate_metadata_by_id.get(&x).unwrap())
            .collect()
    }

    fn get_item_immidiate_by_item_id(&self, item_id: ItemId) -> ItemImmidiateMetadata {
        let item = self.item_immidiate_metadata_by_id.get(&item_id).unwrap();

        item
    }
}
