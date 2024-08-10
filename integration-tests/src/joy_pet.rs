use near_gas::NearGas;
use near_token::NearToken;
use near_units::parse_near;
use serde_json::json;
mod helpers;
use near_sdk::json_types::U128;
use near_workspaces::{Account, Contract};
use near_sdk::env;

use helpers::{
    get_item_prototype_metadata_by_id, get_level_pet_by_id, get_pet_metadata_by_id, get_score_pet_by_id, storage_deposit, ItemRarity, Status, TokenMetadata
};

use crate::helpers::{get_item_immidiate_metadata_by_id, JsonToken, PetAttribute, PetEvolution};

const NFT_PET_WASM_FILEPATH: &str = "../res/nft_pet.wasm";

const NFT_ITEM_WASM_FILEPATH: &str = "../res/nft_item.wasm";
const JOYCHI_WASM_FILEPATH: &str = "../res/joy_v1.wasm";
const FT_WASM_FILEPATH: &str = "../res/ft_token.wasm";

const INITIAL_NEAR: NearToken = NearToken::from_near(30);

const DEFAULT_DEPOSIT: NearToken = NearToken::from_yoctonear(1);
const DEFAULT_GAS: NearGas = NearGas::from_tgas(200);

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // initiate environemnt
    let worker = near_workspaces::sandbox().await?;

    // deploy contracts
    let ft_wasm = std::fs::read(FT_WASM_FILEPATH)?;
    let ft_contract = worker.dev_deploy(&ft_wasm).await?;

    let nft_pet_wasm = std::fs::read(NFT_PET_WASM_FILEPATH)?;
    let nft_pet_contract = worker.dev_deploy(&nft_pet_wasm).await?;

    let nft_item_wasm = std::fs::read(NFT_ITEM_WASM_FILEPATH)?;
    let nft_item_contract = worker.dev_deploy(&nft_item_wasm).await?;

    let joychi_wasm = std::fs::read(JOYCHI_WASM_FILEPATH)?;
    let joychi_contract = worker.dev_deploy(&joychi_wasm).await?;

    let owner = worker.root_account().unwrap();

    let owner_ft = owner
        .create_subaccount("ft")
        .initial_balance(INITIAL_NEAR)
        .transact()
        .await?
        .into_result()?;

    let owner_nft_pet = owner
        .create_subaccount("nft_pet")
        .initial_balance(INITIAL_NEAR)
        .transact()
        .await?
        .into_result()?;

    let owner_nft_item = owner
        .create_subaccount("nft_item")
        .initial_balance(INITIAL_NEAR)
        .transact()
        .await?
        .into_result()?;

    let owner_joychi = owner
        .create_subaccount("joychi")
        .initial_balance(INITIAL_NEAR)
        .transact()
        .await?
        .into_result()?;

    // Create Alice Account
    let alice = owner
        .create_subaccount("alice")
        .initial_balance(INITIAL_NEAR)
        .transact()
        .await?
        .into_result()?;
    // Create Bob Account
    let bob = owner
        .create_subaccount("bob")
        .initial_balance(INITIAL_NEAR)
        .transact()
        .await?
        .into_result()?;

    let delegate_user = owner
        .create_subaccount("delegate")
        .initial_balance(INITIAL_NEAR)
        .transact()
        .await?
        .into_result()?;

    // Call new construct for NFT
    ft_contract
        .call("new_default_meta")
        .args_json(json!({
            "owner_id": owner_ft.id(),
            "total_supply": U128::from(parse_near!("1,000,000,000 N")),
            //"ref_addr":"ref-finance-101.testnet"
        }))
        .transact()
        .await?
        .into_result()?;

    // Call new construct for NFT Pet
    nft_pet_contract
        .call("new_default_meta")
        .args_json(json!({
            "owner_id": owner_nft_pet.id()
        }))
        .transact()
        .await?
        .into_result()?;

    // Call new construct for NFT Item
    nft_item_contract
        .call("new_default_meta")
        .args_json(json!({
            "owner_id": owner_nft_item.id()
        }))
        .transact()
        .await?
        .into_result()?;

    // Call init constructor for joychi contract
    owner_joychi
        .call(joychi_contract.id(), "init")
        .args_json(json!({
            "nft_addr": nft_pet_contract.id(),
            "nft_item_addr": nft_item_contract.id(),
            "ft_addr": ft_contract.id()
        }))
        .transact()
        .await?
        .into_result()?;

    // Create species

    test_create_species(
        &owner_ft,
        &owner_joychi,
        &alice,
        &joychi_contract,
        &ft_contract,
    )
    .await?;
    // Create pet
    test_create_pet(&alice, &joychi_contract, &ft_contract, &nft_pet_contract).await?;
    // Change pet name
    test_change_name_pet(&alice, &joychi_contract).await?;
    // Create item then can buy item
    // Only owner joychi
    test_create_item(&owner_joychi, &joychi_contract).await?;

    // Test create item factory 

    test_create_item_factory(&owner_joychi, &joychi_contract).await?;

    test_edit_item_factory(&owner_joychi, &joychi_contract).await?;
    // Buy item and check score and check level
    test_buy_item(&alice, &joychi_contract, &ft_contract).await?;
    // Create pet 2
    test_create_pet_2(&bob, &joychi_contract).await?;


    // Attack
    test_attack(&bob, &joychi_contract).await?;

    // Create staking pool
    test_create_pool(&owner_joychi, &joychi_contract).await?;

    // stake pool
    test_stake(&alice, &joychi_contract).await?;

    // stake pool
    test_unstake(&alice, &joychi_contract, &owner_ft, &ft_contract).await?;
    // Test kill pet

    test_kill_pet(&bob, &joychi_contract).await?;

    // Test redeem

    test_redeem(&alice, &joychi_contract).await?;

    // Test update metadata attribute for level 2
    // test_update_metadata_attribute(
    //     &alice,
    //     &delegate_user,
    //     &owner_joychi,
    //     &joychi_contract,
    //     &nft_pet_contract,
    // )
    // .await?;

    // test_update_metadata_token(&alice, &delegate_user, &joychi_contract, &nft_pet_contract).await?;

    Ok(())
}

pub async fn test_create_species(
    owner_ft: &Account,
    owner_joychi: &Account,
    user: &Account,
    joychi_contract: &Contract,
    ft_contract: &Contract,
) -> anyhow::Result<()> {
    // user have JOY token
    storage_deposit(owner_ft, ft_contract, user).await?;
    owner_ft
        .call(ft_contract.id(), "ft_transfer")
        .args_json(json!({
            "receiver_id": user.id(),
            "amount": U128(parse_near!("10 N"))
        }))
        .deposit(DEFAULT_DEPOSIT)
        .transact()
        .await?
        .into_result()?;

    // check user should have initital JOY token

    let user_balance: U128 = user
        .call(ft_contract.id(), "ft_balance_of")
        .args_json(json!({
            "account_id": user.id()
        }))
        .transact()
        .await?
        .json()?;

    assert_eq!(user_balance, U128(parse_near!("10 N")));
    // define pet evolution
    let pet_evolution_1 = PetEvolution {
        image: "evolution_1_image.com".to_string(),
        name: "Gold".to_string(),
        attack_win_rate: 5,
        next_evolution_level: 2,
    };

    let pet_evolution_2 = PetEvolution {
        image: "evolution_2_image.com".to_string(),
        name: "Platium".to_string(),
        attack_win_rate: 5,
        next_evolution_level: 3,
    };

    let pet_evolution_3 = PetEvolution {
        image: "evolution_3_image.com".to_string(),
        name: "Titan".to_string(),
        attack_win_rate: 5,
        next_evolution_level: 4,
    };

    let pet_evolutions = vec![pet_evolution_1, pet_evolution_2, pet_evolution_3];

    //Create species
    owner_joychi
        .call(joychi_contract.id(), "create_species")
        .args_json(json!({"need_evol_item": true, "evol_item_id": 1, "name_spec": "JOY1", "pet_evolution":pet_evolutions}))
        .transact()
        .await?
        .into_result()?;
    println!("      Passed ✅ test_create_species");
    Ok(())
}

pub async fn test_create_pet(
    user: &Account,
    joychi_contract: &Contract,
    ft_contract: &Contract,
    nft_contract: &Contract,
) -> anyhow::Result<()> {
    // create pet

    user.call(joychi_contract.id(), "create_pet")
        .args_json(json!({ "name": "Pet1"}))
        .gas(DEFAULT_GAS)
        .transact()
        .await?
        .into_result()?;

    // check pet alive
    let pet_is_alive: bool = user
        .call(joychi_contract.id(), "is_pet_alive")
        .args_json(json!({
            "pet_id": 1
        }))
        .transact()
        .await?
        .json()?;
    assert_eq!(pet_is_alive, true);
    // Check burn token after create pet (mint pet)

    let user_balance_after_creation: U128 = user
        .call(ft_contract.id(), "ft_balance_of")
        .args_json(json!({
            "account_id": user.id()
        }))
        .transact()
        .await?
        .json()?;

    assert_eq!(user_balance_after_creation, U128(9999999999999990000000000));

    // Check nft is minted
    // View metdata
    let expected_metadata = json!({
        "base_uri": serde_json::Value::Null,
        "icon": serde_json::Value::Null,
        "name": "Joygotchi",
        "reference": serde_json::Value::Null,
        "reference_hash": serde_json::Value::Null,
        "spec": "Joygotchi",
        "symbol": "Joychi",
    });
    let result_metadata: serde_json::Value = user
        .call(nft_contract.id(), "nft_metadata")
        .args_json(json!({  "account_id": user.id()  }))
        .transact()
        .await?
        .json()?;
    assert_eq!(expected_metadata, result_metadata);

    // Check pet attribute in level 1
    let pet_attribute: PetAttribute = user
        .call(joychi_contract.id(), "token_uri")
        .args_json(json!({"pet_id": 1}))
        .transact()
        .await?
        .json()?;

    assert_eq!(pet_attribute.pet_name, "Pet1".to_string());
    assert_eq!(pet_attribute.image, "evolution_1_image.com".to_string());
    assert_eq!(pet_attribute.score, 0);
    assert_eq!(pet_attribute.level, 1);
    assert_eq!(pet_attribute.status, Status::HAPPY);
    assert_eq!(pet_attribute.star, 0);
    println!("      Passed ✅ test_create_pet");

    Ok(())
}

pub async fn test_change_name_pet(
    user: &Account,
    joychi_contract: &Contract,
) -> anyhow::Result<()> {
    // Check change pet name

    // Before change name

    let pet_before_changing_name = get_pet_metadata_by_id(&user, 1, &joychi_contract).await?;

    // we create `Pet1` name
    assert_eq!("Pet1".to_string(), pet_before_changing_name.name);

    // change to Pet1_New
    user.call(joychi_contract.id(), "change_name_pet")
        .args_json(json!({ "pet_id": 1,"name": "Pet1_New"}))
        .transact()
        .await?
        .into_result()?;

    let pet_after_changing_name = get_pet_metadata_by_id(&user, 1, &joychi_contract).await?;

    assert_eq!("Pet1_New".to_string(), pet_after_changing_name.name);

    println!("      Passed ✅ test_change_name_pet");
    Ok(())
}

pub async fn test_create_item(
    owner_joychi: &Account,
    joychi_contract: &Contract,
) -> anyhow::Result<()> {
    let name = "hat".to_string();
    let price = 100000;
    let points = 100000000000000u128;
    let time_extension = 100021310000u128;
    let price_delta = 10;
    let stock = 5;
    let shield = 10;

    owner_joychi
        .call(joychi_contract.id(), "create_item_immidiate")
        .args_json(json!({"name":name,"price":price, "points":points, "time_extension": time_extension, "price_delta": price_delta, "stock":stock, "shield": shield, "is_revival": true}))
        .transact()
        .await?
        .into_result()?;

    println!("      Passed ✅ test_create_item");
    Ok(())
}

pub async fn test_create_item_factory(
    owner_joychi: &Account,
    joychi_contract: &Contract,
) -> anyhow::Result<()> {

    let prototype_item_image = "Prototype_Image_1";
    let prototype_item_type = "Hat";
    let prototype_item_cooldown_breed_time = 100000u128;
    let prototype_item_reduce_breed_fee = 1000000u128; 
    let prototype_item_points = 100000000u128;
    let prototype_item_rarity = ItemRarity::Common;
    let prototype_itemmining_power = 10u128;
    let prototype_itemmining_charge_time = 1000u128;

    owner_joychi
        .call(joychi_contract.id(), "create_item")
        .args_json(json!({"prototype_item_image":prototype_item_image,"prototype_item_type":prototype_item_type, "prototype_item_cooldown_breed_time":prototype_item_cooldown_breed_time, "prototype_item_reduce_breed_fee": prototype_item_reduce_breed_fee, "prototype_item_points": prototype_item_points, "prototype_item_rarity":prototype_item_rarity, "prototype_itemmining_power": prototype_itemmining_power, "prototype_itemmining_charge_time": prototype_itemmining_charge_time}))
        .transact()
        .await?
        .into_result()?;

    let item_create = get_item_prototype_metadata_by_id(&owner_joychi, 1, joychi_contract).await?;
    assert_eq!("Prototype_Image_1", item_create.prototype_item_image);
    assert_eq!(ItemRarity::Common, item_create.prototype_item_rarity);
    println!("      Passed ✅ test_create_item_factory");
    Ok(())
}

async fn test_edit_item_factory(
    owner_joychi: &Account,
    joychi_contract: &Contract,
) -> anyhow::Result<()> {

    let prototype_item_image = "Prototype_Image_1";
    let prototype_item_type = "Hat";
    let prototype_item_cooldown_breed_time = 100000u128;
    let prototype_item_reduce_breed_fee = 1000000u128; 
    let prototype_item_points = 100000000u128;
    let prototype_item_rarity = ItemRarity::Epic;
    let prototype_itemmining_power = 10u128;
    let prototype_itemmining_charge_time = 1000u128;

    owner_joychi
        .call(joychi_contract.id(), "edit_item")
        .args_json(json!({"item_id":1, "prototype_item_image":prototype_item_image,"prototype_item_type":prototype_item_type, "prototype_item_cooldown_breed_time":prototype_item_cooldown_breed_time, "prototype_item_reduce_breed_fee": prototype_item_reduce_breed_fee, "prototype_item_points": prototype_item_points, "prototype_item_rarity":prototype_item_rarity, "prototype_itemmining_power": prototype_itemmining_power, "prototype_itemmining_charge_time": prototype_itemmining_charge_time}))
        .transact()
        .await?
        .into_result()?;

    let item_edit = get_item_prototype_metadata_by_id(&owner_joychi, 1, joychi_contract).await?;
    assert_eq!(ItemRarity::Epic, item_edit.prototype_item_rarity);
    println!("      Passed ✅ test_edit_item_factory");
    Ok(())
}


pub async fn test_buy_item(
    user: &Account,
    joychi_contract: &Contract,
    ft_contract: &Contract,
) -> anyhow::Result<()> {
    // get score and level at the beginning
    let score_before_buying_item = get_score_pet_by_id(user, 1, joychi_contract).await?;
    assert_eq!(score_before_buying_item, 0);

    let level_before_buying_item = get_level_pet_by_id(user, 1, joychi_contract).await?;
    assert_eq!(level_before_buying_item, 1);

    let stock_before_buying_item = get_item_immidiate_metadata_by_id(user, 1, joychi_contract)
        .await?
        .stock;
    assert_eq!(stock_before_buying_item, 5);

    // buy item

    user.call(joychi_contract.id(), "buy_item_immidiate")
        .args_json(json!({ "pet_id": 1,"item_id": 1}))
        .transact()
        .await?
        .into_result()?;

    let score_after_buying_item = get_score_pet_by_id(user, 1, joychi_contract).await?;

    assert_eq!(score_after_buying_item, 100000000000000u128);
    

    // Update level
    let new_level: u128 = user
        .call(joychi_contract.id(), "level_pet")
        .args_json(json!({ "pet_id": 1}))
        .transact()
        .await?
        .json()?;
    assert_eq!(new_level, 2);

    let level_after_buying_item = get_level_pet_by_id(user, 1, joychi_contract).await?;
    assert_eq!(level_after_buying_item, 2);

    let stock_after_buying_item = get_item_immidiate_metadata_by_id(user, 1, joychi_contract)
        .await?
        .stock;
    assert_eq!(stock_after_buying_item, 4);

    // check burn token
    let user_balance_after_buying: U128 = user
        .call(ft_contract.id(), "ft_balance_of")
        .args_json(json!({
            "account_id": user.id()
        }))
        .transact()
        .await?
        .json()?;

    assert_eq!(user_balance_after_buying, U128(9999999999999989999899990));

    println!("      Passed ✅ test_buy_item");

    Ok(())
}

pub async fn test_create_pet_2(user: &Account, joychi_contract: &Contract) -> anyhow::Result<()> {
    // create pet

    user.call(joychi_contract.id(), "create_pet")
        .args_json(json!({ "name": "Pet2","metadata": TokenMetadata::default()}))
        .gas(DEFAULT_GAS)
        .transact()
        .await?
        .into_result()?;

    println!("      Passed ✅ test_create_pet_2");

    Ok(())
}

pub async fn test_attack(user: &Account, joychi_contract: &Contract) -> anyhow::Result<()> {
    let pet_is_alive: bool = user
        .call(joychi_contract.id(), "is_pet_alive")
        .args_json(json!({
            "pet_id": 2
        }))
        .transact()
        .await?
        .json()?;

    assert_eq!(pet_is_alive, true);

    user.call(joychi_contract.id(), "attack")
        .args_json(json!({ "from_id": 2,"to_id":1 }))
        .gas(DEFAULT_GAS)
        .transact()
        .await?
        .into_result()?;

    println!("      Passed ✅ test_attack");

    Ok(())
}


async fn test_create_pool(
    owner_joychi: &Account,
    joychi_contract: &Contract,
) -> anyhow::Result<()> {

    
    let start_time = env::block_timestamp_ms();
    let end_time = start_time + 100000000000;

    owner_joychi
        .call(joychi_contract.id(), "create_new_staking_pool")
        .args_json(json!({"name": "Pool1", "reward_nft_ids": vec![1], "staking_start_time": start_time, "staking_end_time": end_time, "max_slot_in_pool":10, "token_reward_per_slot": 1, "max_slot_per_wallet": 10   }))
        .transact()
        .await?
        .into_result()?;

    println!("      Passed ✅ test_create_pool");
    Ok(())
}


pub async fn test_stake(user: &Account, joychi_contract: &Contract) -> anyhow::Result<()> {


    user.call(joychi_contract.id(), "stake")
        .args_json(json!({ "pet_id": 1,"pool_id": 1}))
        .transact()
        .await?
        .into_result()?;


    println!("      Passed ✅ test_stake");

    Ok(())
}

pub async fn test_unstake(user: &Account, joychi_contract: &Contract, owner_ft: &Account, ft_contract: &Contract) -> anyhow::Result<()> {


    storage_deposit(owner_ft, ft_contract, joychi_contract.as_account()).await?;
    
    owner_ft
        .call(ft_contract.id(), "ft_transfer")
        .args_json(serde_json::json!({
            "receiver_id": joychi_contract.id(),
            "amount": U128(parse_near!("10,000 N"))
        }))
        .deposit(DEFAULT_DEPOSIT)
        .transact()
        .await?
        .into_result()?;

    user.call(joychi_contract.id(), "un_stake")
        .args_json(json!({ "pet_id": 1,"pool_id": 1}))
        .gas(DEFAULT_GAS)
        .deposit(NearToken::from_yoctonear(1))
        .transact()
        .await?
        .into_result()?;

    println!("      Passed ✅ test_unstake");

    Ok(())
}

pub async fn test_redeem(user: &Account, joychi_contract: &Contract) -> anyhow::Result<()> {
    joychi_contract
        .as_account()
        .call(joychi_contract.id(), "redeem")
        .args_json(json!({ "pet_id": 1, "to_addr": user.id()}))
        .transact()
        .await?
        .into_result()?;
    println!("      Passed ✅ test_redeem");
    Ok(())
}

pub async fn test_kill_pet(user: &Account, joychi_contract: &Contract) -> anyhow::Result<()> {
    user.call(joychi_contract.id(), "kill_pet")
        .args_json(json!({"pet_kill": 2, "pet_receive": 1}))
        .transact()
        .await?
        .into_result()?;
    println!("      Passed ✅ test_kill_pet");
    Ok(())
}

pub async fn test_update_metadata_attribute(
    user: &Account,
    delegate_user: &Account,
    owner_joychi: &Account,
    joychi_contract: &Contract,
    nft_contract: &Contract,
) -> anyhow::Result<()> {

    println!("GO to here:");
    let pet_attribute: PetAttribute = user
        .call(joychi_contract.id(), "token_uri")
        .args_json(json!({"pet_id": 1}))
        .transact()
        .await?
        .json()?;
    println!("GO to here 2:");
    assert_eq!(pet_attribute.pet_name, "Pet1_New".to_string());
    assert_eq!(pet_attribute.image, "evolution_2_image.com".to_string());
    assert_eq!(pet_attribute.score, 99999999999000);
    assert_eq!(pet_attribute.level, 2);
    assert_eq!(pet_attribute.status, Status::HAPPY);
    assert_eq!(pet_attribute.star, 1);
    let nft_metadata: JsonToken = user
        .call(nft_contract.id(), "nft_token")
        .args_json(json!({"token_id": "1"}))
        .transact()
        .await?
        .json()?;
    assert_eq!(nft_metadata.token_id, "1");
    assert_eq!(nft_metadata.owner_id.as_str(), user.id().as_str());
    assert_eq!(nft_metadata.metadata.media.unwrap(), "evolution_2_image.com".to_string());

    let current_level = get_level_pet_by_id(user, 1, joychi_contract).await?;
    assert_eq!(current_level, 2);

    // Assign to delegate user
    owner_joychi
        .call(joychi_contract.id(), "add_access_update_pet")
        .args_json(json!({ "pet_id": 1, "user_id": delegate_user.id()}))
        .transact()
        .await?
        .into_result()?;

    let new_attribute = PetAttribute {
        pet_name: "Dustin".to_string(),
        image: "xyz.com".to_string(),
        score: 10000,
        level: 1,
        status: Status::HAPPY,
        star: 0,
    };
    delegate_user
        .call(joychi_contract.id(), "delegate_update_attribute")
        .args_json(json!({ "pet_id": 1, "pet_attribute":new_attribute}))
        .transact()
        .await?
        .into_result()?;

    let nft_metadata: JsonToken = user
        .call(nft_contract.id(), "nft_token")
        .args_json(json!({"token_id": "1"}))
        .transact()
        .await?
        .json()?;
    assert_eq!(nft_metadata.metadata.media.unwrap(), "xyz.com".to_string());
    println!("      Passed ✅ test_update_metadata_attribute");
    Ok(())
}

pub async fn test_update_metadata_token(
    user: &Account,
    delegate_user: &Account,
    joychi_contract: &Contract,
    nft_contract: &Contract,
) -> anyhow::Result<()> {
    let nft_metadata_before: JsonToken = user
        .call(nft_contract.id(), "nft_token")
        .args_json(json!({"token_id": "1"}))
        .transact()
        .await?
        .json()?;

    assert_eq!(nft_metadata_before.metadata.title.unwrap(), "Pet1".to_string());

    let mut new_metadata = TokenMetadata::default();
    new_metadata.title = Some("This is new metadata description".to_string());

    delegate_user
        .call(joychi_contract.id(), "delegate_update_metadata")
        .args_json(json!({ "pet_id": 1, "token_metadata":new_metadata}))
        .transact()
        .await?
        .into_result()?;

    let nft_metadata_after: JsonToken = user
        .call(nft_contract.id(), "nft_token")
        .args_json(json!({"token_id": "1"}))
        .transact()
        .await?
        .json()?;
    assert_eq!(nft_metadata_after.metadata.title.unwrap(), "This is new metadata description".to_string());

    println!("      Passed ✅ test_update_metadata_token");

    Ok(())
}
