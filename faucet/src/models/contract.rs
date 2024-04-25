use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::U128;
use near_sdk::{near_bindgen, AccountId, PanicOnDefault};

#[near_bindgen]
#[derive(PanicOnDefault, BorshDeserialize, BorshSerialize)]
pub struct Faucet {
    /// Account ID of the owner of the contract.  
    pub owner_id: AccountId,

    pub ft_address: AccountId,

    pub is_active: bool,
    
    pub amount: U128
}
