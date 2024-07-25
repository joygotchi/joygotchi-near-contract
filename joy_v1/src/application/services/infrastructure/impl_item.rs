use near_sdk::{env, near_bindgen};

use crate::models::{
    contract::{JoychiV1, JoychiV1Ext},
    item::{ItemFeature, ItemMetadata, ItemRarity},
    ItemId,
};

#[near_bindgen]
impl ItemFeature for JoychiV1 {
    fn create_item(
        &mut self,
        prototype_item_image: String,
        prototype_item_type: String,
        prototype_item_cooldown_breed_time: u128,
        prototype_item_reduce_breed_fee: u128,
        prototype_item_points: u128,
        prototype_item_rarity: ItemRarity,
        prototype_itemmining_power: u128,
        prototype_itemmining_charge_time: u128,
    ) -> ItemMetadata {
        assert!(
            self.owner_id == env::signer_account_id(),
            "You're not permission"
        );
        let num_item_id = self.all_item_id.len();

        let item_metadata = ItemMetadata {
            item_id: num_item_id + 1,
            item_rarity_amount: 1,                      // TODO
            list_prototype_items_of_rarity: Vec::new(), // TODO
            prototype_item_image,
            prototype_item_type,
            prototype_item_cooldown_breed_time,
            prototype_item_reduce_breed_fee,
            prototype_item_points,
            prototype_item_rarity,
            prototype_itemmining_power,
            prototype_itemmining_charge_time,
        };

        self.item_metadata_by_id
            .insert(&(&num_item_id + 1), &item_metadata);
        self.all_item_id.insert(&(&num_item_id + 1));

        item_metadata
    }

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
    ) {
        let mut item = self.item_metadata_by_id.get(&item_id).unwrap();

        assert!(
            self.owner_id == env::signer_account_id(),
            "You're not permission"
        );

        item.prototype_item_image = prototype_item_image;
        item.prototype_item_cooldown_breed_time = prototype_item_cooldown_breed_time;
        item.prototype_item_reduce_breed_fee = prototype_item_reduce_breed_fee;
        item.prototype_item_points = prototype_item_points;
        item.prototype_item_rarity = prototype_item_rarity;
        item.prototype_itemmining_power = prototype_itemmining_power;
        item.prototype_itemmining_charge_time = prototype_itemmining_charge_time;

        self.item_metadata_by_id.insert(&item_id, &item);
    }
}
