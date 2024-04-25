use models::{
    contract::{Faucet, FaucetExt},
    ft_request::external::cross_joychi,
};
use near_sdk::{env, json_types::U128, near_bindgen, AccountId, Gas};
pub mod models;

pub const GAS_FOR_CROSS_CALL: Gas = Gas(3_000_000_000_000);
pub const ATTACHED_TRANSFER_FT: u128 = 1;
pub const ATTACHED_STORAGE_DEPOSIT: u128 = 1_250_000_000_000_000_000_000;

#[near_bindgen]
impl Faucet {
    #[init]
    pub fn init(ft_address: AccountId) -> Self {
        let owner_id = env::signer_account_id();
        let is_ative = true;

        Self {
            owner_id,
            ft_address,
            is_active: is_ative,
            amount: U128(10)
        }
    }

    pub fn get_joychi(&mut self, addr_to: AccountId) {
        assert!(self.is_active, "faucet's not ative");
        assert!(self.amount.0 > 0, "Owner must set faucet amount");
        cross_joychi::ext(self.ft_address.to_owned())
            .with_static_gas(GAS_FOR_CROSS_CALL)
            .with_attached_deposit(ATTACHED_STORAGE_DEPOSIT)
            .storage_deposit(addr_to.clone())
            .then(
                cross_joychi::ext(self.ft_address.to_owned())
                    .with_static_gas(GAS_FOR_CROSS_CALL)
                    .with_attached_deposit(ATTACHED_TRANSFER_FT)
                    .ft_transfer(addr_to.clone(), self.amount),
            );
    }

    pub fn set_faucet_amount(&mut self, amount: U128){
        assert!(self.owner_id  == env::signer_account_id(), " Not Owner");
        self.amount = amount;
        

    }

}
