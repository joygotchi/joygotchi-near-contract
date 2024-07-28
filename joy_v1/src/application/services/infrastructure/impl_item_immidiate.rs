use near_sdk::{env, near_bindgen};

use crate::models::{
    contract::{JoychiV1, JoychiV1Ext},
    item_immidiate::{ItemImmidiateFeature, ItemImmidiateMetadata},
    ItemId,
};

#[near_bindgen]
impl ItemImmidiateFeature for JoychiV1 {
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
    ) -> ItemImmidiateMetadata {
        assert!(
            self.owner_id == env::signer_account_id(),
            "You're not permission"
        );
        let num_item_id = self.all_item_immidiate_id.len();

        let item_metadata = ItemImmidiateMetadata {
            item_id: num_item_id + 1,
            name,
            price,
            points,
            time_extension,
            price_delta,
            stock,
            shield,
            is_revival,
        };

        self.item_immidiate_metadata_by_id
            .insert(&(&num_item_id + 1), &item_metadata);
        self.all_item_immidiate_id.insert(&(&num_item_id + 1));

        item_metadata
    }

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
    ) {
        let mut item = self.item_immidiate_metadata_by_id.get(&item_id).unwrap();

        assert!(
            self.owner_id == env::signer_account_id(),
            "You're not permission"
        );

        item.name = name;
        item.price = price;
        item.points = points;
        item.time_extension = time_extension;
        item.price_delta = price_delta;
        item.stock = stock;
        item.shield = shield;
        item.is_revival = is_revival;

        self.item_immidiate_metadata_by_id.insert(&item_id, &item);
    }
}
