use near_sdk::{env, near_bindgen, AccountId};

use crate::models::{
    contract::{JoychiV1, JoychiV1Ext},
    item_factory::{ItemFeature, ItemMetadata, ItemRarity, ItemType},
    nft_request::external::{cross_item_nft, TokenMetadata},
    ItemId,
};

use super::impl_pet::{ATTACHED_DEPOSIT_NFT, GAS_FOR_CROSS_CALL};

#[near_bindgen]
impl ItemFeature for JoychiV1 {
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
        let mut item: ItemMetadata = self.item_metadata_by_id.get(&item_id).unwrap();

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

        // TODO
    }

    fn mint_item_for_user(&mut self, to_addr: AccountId, item_id: ItemId) {
        let item_metadata = self.item_metadata_by_id.get(&item_id).unwrap();

        let token_metadata = TokenMetadata {
            title: Some(item_metadata.prototype_item_image.clone()),
            description: Some(format!(
                "item_image:{}, item_type:{:?}, cooldown_breed_time:{}, reduce_breed_fee:{}, item_points:{:?}, item_rarity:{:?}, mining_power:{}, mining_charge_time:{}",
                item_metadata.prototype_item_image,
                item_metadata.prototype_item_type,
                item_metadata.prototype_item_cooldown_breed_time,
                item_metadata.prototype_item_reduce_breed_fee,
                item_metadata.prototype_item_points,
                item_metadata.prototype_item_rarity,
                item_metadata.prototype_itemmining_power,
                item_metadata.prototype_itemmining_charge_time
        )),
            media: Some(item_metadata.prototype_item_image.clone()),
            media_hash: None,
            copies: None,
            issued_at: None,
            expires_at: Some(env::block_timestamp()),
            starts_at: Some(env::block_timestamp()),
            updated_at: Some(env::block_timestamp()),
            extra: None,
            reference: None,
            reference_hash: None,
        };

        cross_item_nft::ext(self.nft_item_address.to_owned())
        .with_static_gas(GAS_FOR_CROSS_CALL)
        .with_attached_deposit(ATTACHED_DEPOSIT_NFT)
        .nft_mint(
            (item_id.clone()).to_string(),
            token_metadata,
            to_addr.clone(),
        );
    }

}