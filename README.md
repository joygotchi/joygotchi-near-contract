# Contracts for testnet

```
FT_CONTRACT = "token.joychi.testnet"
NFT_CONTRACT = "nft.joychi.testnet"
FAUCET_CONTRACT = "faucet.joychi.testnet"
GAME_CONTRACT = "game.joychi.testnet"
```

# Contracts for dev

```bash
export ECO=$(<./neardev/dev-account)
export USER1=kaito021201.testnet
export USER2=kaito021202.testnet
```

## Deploy Fungible Token

```bash
cd ft_token
cargo make clean
cargo make build
cargo make dev-deploy

cargo make call new_default_meta '{"owner_id": "'$USER1'", "total_supply": "100000000"}' --accountId $ECO

cargo make call storage_deposit '{"account_id": "'$USER1'"}' --accountId $ECO --amount 0.0125

cargo make call ft_transfer '{"receiver_id": "'$USER1'", "amount": "100"}' --accountId $ECO --amount 0.000000000000000000000001

cargo make view ft_balance_of '{"account_id": "'$USER1'"}'

```

## Deploy Faucet

```bash
cd faucet
cargo make clean
cargo make build
cargo make dev-deploy
cargo make call init '{"ft_address": "'$FT_ADDRESS'"}'
cargo make call get_joychi '{"'$ADDR_TO'"}' --accountId $ECO
```

## Deploy Non-Fungible Token

```bash
cd nft
cargo make clean
cargo make build
cargo make dev-deploy
export ECO=$(<./neardev/dev-account)
cargo make call new_default_meta '{"owner_id": "'$ECO'"}' --accountId $ECO
# cargo make call nft_mint '{"token_id": "token-1", "metadata": {"score": 100, "level": 2, "status": "alive", "star": 4}, "receiver_id": "'$USER1'"}' --accountId $USER1 --amount 0.1
cargo make view nft_token '{"token_id": "1"}'

```

## Flow Joychi

```bash
# Deploy FT contract -> get $FT_ADDRESS
# Deploy NFT contract -> get $NFT_ADDRESS
```

```bash
cd joy_v1
cargo make clean
cargo make build
cargo make dev-deploy

# init joychi

cargo make call-self init '{"nft_addr": "'$NFT_ADDRESS'", "ft_addr": "'$FT_ADDRESS'"}'

export ECO=$(<./neardev/dev-account)

# create species for pet with owner contract joychi

cargo make call create_species '{"need_evol_item": true, "evol_item_id": 1, "name_spec": "test", "pet_evolution": [{"image": "test", "name": "test1", "attack_win_rate": 1, "next_evolution_level": 2}, {"image": "test", "name": "test2", "attack_win_rate": 1, "next_evolution_level": 3}, {"image": "test", "name": "test3", "attack_win_rate": 1, "next_evolution_level": 4}]}' --accountId $ECO

# create 2 pet for user

cargo make call create_pet '{"name": "Joychi1"}' --accountId $USER1 // create pet

cargo make call create_pet '{"name": "Joychi2"}' --accountId $USER2

# change name pet by owner

cargo make call change_name_pet '{"pet_id": 1, "name": "Joychi 2" }' --accountId $USER1

# create some item by owner contract joychi

cargo make call create_item_immidiate '{"name": "test3", "price": 100, "points": 10, "time_extension": 10002131000000, "price_delta": 0, "stock": 3, "shield": 2, "is_revival": true}' --accountId $ECO

cargo make call create_item_immidiate '{"name": "test2", "price": 100, "points": 10, "time_extension": 10002131100000, "price_delta": 0, "stock": 3, "shield": 2, "is_revival": false }' --accountId $ECO

# buy item for pet

cargo make call buy_item_immidiate '{"pet_id": 1, "item_id": 1 }' --accountId $USER1

cargo make call buy_item_immidiate '{"pet_id": 2, "item_id": 2 }' --accountId $USER2

# attack different pet

cargo make call attack '{"from_id": 1, "to_id": 2}' --accountId $USER1
cargo make call attack '{"from_id": 1, "to_id": 2}' --accountId $USER1

# calc level pet

cargo make call level_pet '{"pet_id" : 1}' --accountId $USER1
cargo make call level_pet '{"pet_id" : 2}' --accountId $USER2

# kill pet

cargo make call kill_pet '{"pet_kill": 1, "pet_receive": 2}' --accountId $USER1

# update metadata pet to token uri nft

cargo make call token_uri '{"pet_id": 1}' --accountId $USER1

# Give permission to Delegated user to update attribute pet

cargo make call add_access_update_pet '{"pet_id": 1, "user_id":"'$USER1'"}' --accountId $ECO

# Delegated user to update attribute pet

cargo make call delegate_update_attribute '{"pet_id": 1, "pet_attribute":{pet_name:"Dustin",image: "xyz.com",score: 10000,level: 1,status: 0,star: 0}}' --accountId $ECO

# Check evol pet if reach phase

cargo make call check_evol_pet_if_needed '{"pet_id": 1}' --accountId $USER1

```

### GET infomation

```bash
# get all pet

cargo make view get_all_pet_metadata

# get pet by pet id

cargo make view get_pet_by_pet_id '{"pet_id": 1}'

# get all battle

cargo make view get_all_battle_metadata '{""}'

# get battle by pet id

cargo make view get_battle_by_pet_id '{"battle_id": 1}'

# get status pet

cargo make view get_status_pet '{"pet_id": 1}'

# get pet evolution item

cargo make view get_pet_evolution_item '{"pet_id": 1}'

# get pet attack winrate

cargo make view get_pet_attack_winrate '{"pet_id": 1}'

# get pet image

cargo make view get_pet_image '{"pet_id": 1}'

# get all item immidiate

cargo make view get_all_item_immidiate_metadata

# get item by item id

cargo make view get_item_immidiate_by_item_id '{"item_id": 1}'


```

## Integration tests

### Build contract

```bash
    ./scripts/build.sh
```

### Run integration tests

1. Go to `integration-tests` folder

```
    cd integration-tests
```

2. Run tests for faucet contract

```
    cargo run --example faucet
```

3. Run tests for joychi contract

```
    cargo run --example pet
```
