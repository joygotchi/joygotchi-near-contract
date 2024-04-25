use near_gas::NearGas;
use near_token::NearToken;
use near_units::parse_near;
use serde_json::json;
mod helpers;
use near_sdk::json_types::U128;
use near_workspaces::{Account, Contract};

use helpers::storage_deposit;

const JOY_TOKEN_WASM_FILEPATH: &str = "../res/ft_token.wasm";
const JOY_FAUCET_WASM_FILEPATH: &str = "../res/faucet.wasm";

const ALICE_NEAR: NearToken = NearToken::from_near(30);
const BOB_NEAR: NearToken = NearToken::from_near(30);

const DEFAULT_DEPOSIT: NearToken = NearToken::from_yoctonear(1);
const DEFAULT_GAS: NearGas = NearGas::from_tgas(200);

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // initiate environemnt
    let worker = near_workspaces::sandbox().await?;

    // deploy contracts
    let ft_wasm = std::fs::read(JOY_TOKEN_WASM_FILEPATH)?;
    let ft_contract = worker.dev_deploy(&ft_wasm).await?;
    let faucet_wasm = std::fs::read(JOY_FAUCET_WASM_FILEPATH)?;
    let faucet_contract = worker.dev_deploy(&faucet_wasm).await?;

    let owner = worker.root_account().unwrap();

    // Create Alice Account
    let alice = owner
        .create_subaccount("alice")
        .initial_balance(ALICE_NEAR)
        .transact()
        .await?
        .into_result()?;
    // Create Bob Account
    let bob = owner
        .create_subaccount("bob")
        .initial_balance(BOB_NEAR)
        .transact()
        .await?
        .into_result()?;

    // Call new construct for fungible token
    ft_contract
        .call("new_default_meta")
        .args_json(json!({
            "owner_id": owner.id(),
            "total_supply": U128::from(parse_near!("1,000,000,000 N"))
        }))
        .transact()
        .await?
        .into_result()?;

    // Call init constructor for faucet
    faucet_contract
        .call("init")
        .args_json(json!({
            "ft_address": ft_contract.id()
        }))
        .transact()
        .await?
        .into_result()?;

    test_total_supply(&owner, &ft_contract).await?;
    test_faucet_token(&owner, &alice, &faucet_contract, &ft_contract).await?;

    // Check faucet token JOY

    Ok(())
}

async fn test_total_supply(owner: &Account, contract: &Contract) -> anyhow::Result<()> {
    let initial_balance = U128::from(parse_near!("1,000,000,000 N"));
    let res: U128 = owner
        .call(contract.id(), "ft_total_supply")
        .args_json(json!({}))
        .transact()
        .await?
        .json()?;

    assert_eq!(res, initial_balance);

    let root_balance: U128 = owner
        .call(contract.id(), "ft_balance_of")
        .args_json(serde_json::json!({
            "account_id": owner.id()
        }))
        .transact()
        .await?
        .json()?;

    assert_eq!(root_balance, initial_balance);

    println!("      Passed ✅ test_total_supply");
    Ok(())
}

async fn test_faucet_token(
    owner: &Account,
    user: &Account,
    faucet_contract: &Contract,
    ft_contract: &Contract,
) -> anyhow::Result<()> {
    // storage_deposit
    storage_deposit(owner, ft_contract, faucet_contract.as_account()).await?;
    storage_deposit(owner, ft_contract, ft_contract.as_account()).await?;

    owner
        .call(ft_contract.id(), "ft_transfer")
        .args_json(serde_json::json!({
            "receiver_id": faucet_contract.id(),
            "amount": U128(parse_near!("10,000 N"))
        }))
        .deposit(DEFAULT_DEPOSIT)
        .transact()
        .await?
        .into_result()?;

    // set faucet amount
    faucet_contract
        .as_account()
        .call(faucet_contract.id(), "set_faucet_amount")
        .args_json(json!({"amount": U128::from(2000000000000000000000000)}))
        .gas(DEFAULT_GAS)
        .transact()
        .await?
        .into_result()?;

    // Faucet

    owner
        .call(faucet_contract.id(), "get_joychi")
        .args_json(json!({"addr_to": user.id()}))
        .gas(DEFAULT_GAS)
        .transact()
        .await?
        .into_result()?;
    // check token balance
    let user_balance: U128 = user
        .call(ft_contract.id(), "ft_balance_of")
        .args_json(serde_json::json!({
            "account_id": user.id()
        }))
        .transact()
        .await?
        .json()?;

    //set default faucet amount = 2
    let faucet_amount = U128::from(parse_near!("2 N"));
    assert_eq!(user_balance, faucet_amount);

    println!("      Passed ✅ test_faucet");
    Ok(())
}
