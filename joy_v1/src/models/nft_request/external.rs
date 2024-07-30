use near_sdk::json_types::Base64VecU8;
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    serde::{Deserialize, Serialize},
};
use near_sdk::{ext_contract, AccountId};

use crate::models::contract::Status;
use crate::models::item_factory::ItemRarity;

#[ext_contract(cross_pet_nft)]
pub trait PetCrossCall {
    fn nft_mint(
        &mut self,
        token_id: String,
        metadata: TokenMetadata,
        receiver_id: AccountId,
        //we add an optional parameter for perpetual royalties
    );

    fn update_medatada_pet(&mut self, token_id: String, pet_attribute: PetAttribute);
    fn update_token_metadata(&mut self, token_id: String, token_metadata: TokenMetadata);
}

#[ext_contract(cross_item_nft)]
pub trait ItemCrossCall {
    fn nft_mint(
        &mut self,
        token_id: String,
        metadata: TokenMetadata,
        receiver_id: AccountId,
        //we add an optional parameter for perpetual royalties
    );

    fn update_medatada_item(&mut self, token_id: String, item_attribute: ItemAttribute);
    fn update_token_metadata(&mut self, token_id: String, token_metadata: TokenMetadata);
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct PetAttribute {
    pub pet_name: String,
    pub image: String,
    pub score: u128,
    pub level: u128,
    pub status: Status,
    pub star: u64,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct ItemAttribute {
    pub item_image: String,
    pub item_type: String,
    pub cooldown_breed_time: u128,
    pub reduce_breed_fee: u128,
    pub item_points: u128,
    pub item_rarity: ItemRarity,
    pub mining_power: u128,
    pub mining_charge_time: u128,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
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