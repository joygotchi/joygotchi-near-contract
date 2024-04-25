use near_sdk::{ext_contract, json_types::U128, AccountId};

#[ext_contract(cross_joychi)]
pub trait CrossCall {
    fn ft_transfer(&mut self, receiver_id: AccountId, amount: U128);
    fn storage_deposit(&mut self, account_id: AccountId);
}
