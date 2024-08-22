use near_sdk::{near_bindgen, AccountId};

use crate::models::{
    contract::{JoychiV1, JoychiV1Ext},
    staking_and_mining::{MiningData, StakingAndMiningEnum},
};

#[near_bindgen]
impl StakingAndMiningEnum for JoychiV1 {
    fn get_mining_data_by_account_id(&self, account_id: AccountId) -> MiningData {
        let mining_data = self.mining_data_by_account_id.get(&account_id).unwrap();

        mining_data
    }
}
