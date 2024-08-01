use near_sdk::{ext_contract, json_types::U128, AccountId};

#[ext_contract(cross_ft)]
pub trait CrossCall {
    fn ft_burn(&mut self, account_id: AccountId, amount: U128);
    fn ft_transfer(
        &mut self,
        receiver_id: AccountId,
        amount: U128,
        memo: Option<String>,
    );
}
