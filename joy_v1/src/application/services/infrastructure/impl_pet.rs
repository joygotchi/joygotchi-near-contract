use near_sdk::{
    env::{self},
    json_types::U128,
    near_bindgen, AccountId, Gas, Promise,
};

use crate::{
    application::repository::{random_in_range, sqrt, DAY, HOUR, MINUTE},
    models::{
        contract::{BattleMetadata, JoychiV1, JoychiV1Ext, Status},
        ft_request::external::cross_ft,
        nft_request::external::{cross_nft, PetAttribute, TokenMetadata},
        pet::{PetEnum, PetEvolution, PetFeature, PetMetadata, PetSpecies},
        ItemId, PetId,
    },
};

pub const GAS_FOR_CROSS_CALL: Gas = Gas(3_000_000_000_000);
pub const ATTACHED_DEPOSIT_NFT: u128 = 100_000_000_000_000_000_000_000;
pub const ATTACHED_BURN_FT: u128 = 1_000_000_000_000;
pub const PRECISION: u128 = 1e24 as u128;
pub const BURN_AMOUNT: U128 = U128(10000000000);

#[near_bindgen]
impl PetFeature for JoychiV1 {
    fn set_manager(&mut self, manager_addr: AccountId) {
        assert!(
            self.owner_id == env::signer_account_id(),
            "You're not permission"
        );
        self.manager_address = manager_addr;
    }

    fn token_uri(&mut self, pet_id: PetId) -> PetAttribute {
        let pet = self.pet_metadata_by_id.get(&pet_id).unwrap();
        let evol_phase_pet_now: usize = pet.pet_evolution_phase as usize;

        let pet_evolution_by_id = self.pet_evolution_metadata_by_id.get(&pet_id).unwrap();
        let pet_img: String = pet_evolution_by_id[evol_phase_pet_now - 1].image.clone();

        //assert!(self.check_role_update_pet(pet_id, env::signer_account_id()), "You're not permission");

        let pet_attribute = PetAttribute {
            pet_name: pet.name.clone(),
            image: pet_img.clone(),
            score: pet.score,
            level: pet.level,
            status: pet.status,
            star: pet.star,
        };

        cross_nft::ext(self.nft_address.to_owned())
            .with_static_gas(GAS_FOR_CROSS_CALL)
            .update_medatada_pet(pet_id.to_string(), pet_attribute.clone());

        return pet_attribute;
    }

    fn delegate_update_attribute(&mut self, pet_id: PetId, pet_attribute: PetAttribute) {
        assert!(
            self.check_role_update_pet(pet_id, env::signer_account_id()),
            "You're not permission"
        );
        cross_nft::ext(self.nft_address.to_owned())
            .with_static_gas(GAS_FOR_CROSS_CALL)
            .update_medatada_pet(pet_id.to_string(), pet_attribute);
    }

    fn delegate_update_metadata(&mut self, pet_id: PetId, token_metadata: TokenMetadata) {
        assert!(
            self.check_role_update_pet(pet_id, env::signer_account_id()),
            "You're not permission"
        );
        cross_nft::ext(self.nft_address.to_owned())
            .with_static_gas(GAS_FOR_CROSS_CALL)
            .update_token_metadata(pet_id.to_string(), token_metadata);
    }

    fn create_pet(&mut self, name: String) -> PetMetadata {
        let owner_id = env::signer_account_id();

        let num_pet_id: u64 = self.all_pet_id.len();

        let pet_id = num_pet_id + 1;

        assert!(
            self.all_pet_species_id.len() > 0,
            "You need create pet species before"
        );

        let pet_species_id = random_in_range(1, self.all_pet_species_id.len() as i64);

        let mut pet_species = self
            .pet_species_metadata_by_id
            .get(&pet_species_id)
            .unwrap();

        let pet_metadata = PetMetadata {
            pet_id: pet_id,
            name: name.clone(),
            owner_id: owner_id.clone(),
            time_pet_born: env::block_timestamp() as u128,
            time_until_starving: env::block_timestamp() as u128 + (1 * DAY),
            items: Vec::new(),
            score: 0,
            level: 1,
            status: Status::HAPPY,
            star: 0,
            reward_debt: 0,
            pet_species: pet_species.species_id as u128,
            pet_shield: 0,
            last_attack_used: 0,
            last_attacked: 0,
            pet_evolution_item_id: pet_species.evolution_item_id,
            pet_need_evolution_item: pet_species.need_evolution_item,
            pet_has_evolution_item: true,
            pet_evolution_phase: 1,
            extra_permission: Vec::new(),
            category: pet_species.species_name,
        };

        let token_metadata = TokenMetadata {
            title: Some(name.clone()),
            description: Some(format!(
                "name:{}, image:{}, level:{}, score:{}, status:{:?}, star:{}",
                pet_metadata.name, pet_species.pet_evolution[0].image, 1, 0, pet_metadata.status, 0
            )),
            media: Some(pet_species.pet_evolution[0].image.clone()),
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

        self.pet_evolution_metadata_by_id
            .insert(&pet_id, &pet_species.pet_evolution);

        self.pet_metadata_by_id.insert(&pet_id, &pet_metadata);
        self.all_pet_id.insert(&pet_id);

        cross_nft::ext(self.nft_address.to_owned())
            .with_static_gas(GAS_FOR_CROSS_CALL)
            .with_attached_deposit(ATTACHED_DEPOSIT_NFT)
            .nft_mint(
                (num_pet_id + 1).to_string(),
                token_metadata,
                owner_id.clone(),
            );
        cross_ft::ext(self.ft_address.to_owned())
            .with_static_gas(GAS_FOR_CROSS_CALL)
            .ft_burn(owner_id.clone(), BURN_AMOUNT);

        pet_metadata
    }

    fn change_name_pet(&mut self, pet_id: PetId, name: String) {
        let mut pet = self.pet_metadata_by_id.get(&pet_id).unwrap();

        assert!(
            env::signer_account_id() == pet.owner_id,
            "You're not owner this pet"
        );

        pet.name = name;

        self.pet_metadata_by_id.insert(&pet_id, &pet);
    }

    fn buy_item(&mut self, pet_id: PetId, item_id: ItemId) {
        let mut pet = self.pet_metadata_by_id.get(&pet_id).unwrap();

        let mut item = self.item_metadata_by_id.get(&item_id).unwrap();

        assert!(
            pet.owner_id == env::signer_account_id(),
            "You're not permission"
        );
        assert!(
            self.is_pet_alive(pet_id) || (!self.is_pet_alive(pet_id) && item.is_revival),
            "Pet's not alive"
        );
        assert!(item.name.len() > 0, "This item doesn't exist");

        if pet.pet_need_evolution_item && pet.pet_evolution_item_id == item_id as u128 {
            pet.pet_has_evolution_item = true;
        }

        pet.items.push(item.clone());

        self.total_score += item.points;

        pet.score += item.points;
        pet.pet_shield += item.shield;

        let time_extension = env::block_timestamp() as u128 + item.time_extension;
        pet.time_until_starving = time_extension;

        pet.status = update_status_pet(time_extension);
        
        // calc petRewardDebt

        item.price += item.price_delta;
        item.stock -= 1;

        self.item_metadata_by_id.insert(&item_id, &item);
        self.pet_metadata_by_id.insert(&pet_id, &pet);

        cross_ft::ext(self.ft_address.to_owned())
            .with_static_gas(GAS_FOR_CROSS_CALL)
            .ft_burn(pet.owner_id.clone(), U128(item.price));
    }

    fn attack(&mut self, from_id: PetId, to_id: PetId) -> BattleMetadata {
        assert!(from_id != to_id, "Can't hurt yourself");
        assert!(self.is_pet_alive(from_id), "Pet's not alive");

        let ods = random_in_range(from_id as i64, to_id as i64);

        let mut pet_from = self.pet_metadata_by_id.get(&from_id).unwrap();
        let mut pet_to = self.pet_metadata_by_id.get(&to_id).unwrap();

        assert!(
            pet_from.owner_id == env::signer_account_id(),
            "You're not permission"
        );

        // calc condition manager v2

        assert!(
            env::block_timestamp() as u128 >= pet_from.last_attack_used + 15 * MINUTE
                || pet_from.last_attack_used == 0,
            "You have one attack every 15 mins"
        );
        assert!(
            env::block_timestamp() as u128 > pet_to.last_attacked + 1 * HOUR,
            "can be attacked once every hour"
        );
        assert!(
            pet_from.level < pet_to.level,
            "Only attack pets above your level"
        );

        // calculate score & time attack for pet when attack

        let winner;
        let loser;
        if ods > (from_id + to_id) / 2 {
            winner = from_id;
            pet_from.score += 1000;
            if pet_to.score < 1000 {
                pet_to.score = 0;
                pet_to.status = Status::DYING;
            } else {
                pet_to.score -= 1000;
            }

            pet_from.last_attack_used = env::block_timestamp() as u128;
            pet_to.last_attacked = env::block_timestamp() as u128;

            loser = to_id;
        } else {
            winner = to_id;
            pet_to.score += 1000;
            if pet_from.score < 1000 {
                pet_from.score = 0;
                pet_from.status = Status::DYING;
            } else {
                pet_from.score -= 1000;
            }

            pet_to.last_attack_used = env::block_timestamp() as u128;
            pet_from.last_attacked = env::block_timestamp() as u128;

            loser = from_id;
        }

        let num_battle = self.all_battle_id.len() + 1;

        // save log battle

        let battle_metadata: BattleMetadata = BattleMetadata {
            battle_id: num_battle,
            winner,
            loser,
            attacker: from_id.clone(),
            time: env::block_timestamp(),
        };

        self.all_battle_id.insert(&num_battle);
        self.battle_metadata_by_id
            .insert(&num_battle, &battle_metadata);

        self.pet_metadata_by_id.insert(&from_id, &pet_from);
        self.pet_metadata_by_id.insert(&to_id, &pet_to);

        battle_metadata
    }

    fn kill_pet(&mut self, pet_kill: PetId, pet_receive: PetId) {
        assert!(self.is_pet_alive(pet_kill), "Pet's not alive");
        assert!(self.is_pet_alive(pet_receive), "Pet receive's not alive");
        assert!(
            self.pet_metadata_by_id.get(&pet_kill).unwrap().owner_id == env::signer_account_id(),
            "You're not permission"
        );

        // redeem pet kill
        Self::redeem(
            self,
            pet_kill,
            self.pet_metadata_by_id.get(&pet_kill).unwrap().owner_id,
        );

        // Remove the pet_id from the Vec
        self.all_pet_id.remove(&pet_kill);

        // Remove the PetMetadata from the LookupMap
        self.pet_metadata_by_id.remove(&pet_kill);

        let mut pet_received = self.pet_metadata_by_id.get(&pet_receive).unwrap();

        pet_received.star += 1;

        self.pet_metadata_by_id.insert(&pet_receive, &pet_received);
    }

    fn level_pet(&mut self, pet_id: PetId) -> u128 {
        let mut pet = self.pet_metadata_by_id.get(&pet_id).unwrap();

        let mut score: u128 = pet.score / 1e12 as u128;

        score /= 100 as u128;

        let mut pet_level = 1;

        if score != 0 {
            let level = sqrt(score * 2);
            pet_level = level * 2
        }

        // update level pet on metadata
        pet.level = pet_level;

        self.pet_metadata_by_id.insert(&pet_id, &pet);

        pet_level
    }

    fn is_pet_alive(&self, pet_id: PetId) -> bool {
        let pet = self.pet_metadata_by_id.get(&pet_id).unwrap();
        if pet.time_until_starving >= env::block_timestamp() as u128 {
            return true;
        } else {
            return false;
        }
    }

    fn add_access_update_pet(&mut self, pet_id: PetId, user_id: AccountId) -> PetMetadata {
        assert!(
            env::signer_account_id() == self.owner_id,
            "YOu are not owner"
        );
        let mut pet = self.pet_metadata_by_id.get(&pet_id).unwrap();

        pet.extra_permission.push(user_id);

        self.pet_metadata_by_id.insert(&pet_id, &pet);

        pet
    }

    fn check_role_update_pet(&self, pet_id: PetId, user_id: AccountId) -> bool {
        if let Some(pet) = self.pet_metadata_by_id.get(&pet_id) {
            for extra_permission_user_id in &pet.extra_permission {
                if extra_permission_user_id == &user_id {
                    return true;
                }
            }
        }

        let pet = self.pet_metadata_by_id.get(&pet_id).unwrap();

        if pet.owner_id == env::signer_account_id() {
            return true;
        }

        false
    }

    fn create_species(
        &mut self,
        need_evol_item: bool,
        evol_item_id: u128,
        name_spec: String,
        pet_evolution: Vec<PetEvolution>,
    ) {
        assert!(self.owner_id == env::signer_account_id(), "Only owner");

        let num_pet_spec = self.all_pet_species_id.len() + 1;

        let species = PetSpecies {
            species_id: num_pet_spec,
            species_name: name_spec,
            need_evolution_item: need_evol_item.clone(),
            evolution_item_id: evol_item_id,
            pet_evolution: pet_evolution.clone(),
        };

        self.all_pet_species_id.insert(&num_pet_spec);
        self.pet_species_metadata_by_id
            .insert(&num_pet_spec, &species);
    }

    fn check_evol_pet_if_needed(&mut self, pet_id: PetId) {
        let mut pet = self.pet_metadata_by_id.get(&pet_id).unwrap();
        let current_phase = pet.pet_evolution_phase;

        let next_evol_phase = self.get_pet_evolution_phase(pet_id, current_phase);

        if (current_phase < next_evol_phase) {
            pet.pet_evolution_phase = next_evol_phase;
        }

        self.pet_metadata_by_id.insert(&pet_id, &pet);
    }

    #[payable]
    fn redeem(&mut self, pet_id: PetId, to_addr: AccountId) {
        let mut pet = self.pet_metadata_by_id.get(&pet_id).unwrap();

        self.total_score -= pet.score;
        pet.score = 0;
        pet.reward_debt = 0;

        Promise::new(to_addr.clone()).transfer(1 / 10_000);
    }
}



// Helper function
fn update_status_pet(time_until_starving: u128) -> Status {

    if time_until_starving > (env::block_timestamp() as u128 + 16 * HOUR) {
        return Status::HAPPY;
    } else if time_until_starving > (env::block_timestamp() as u128 + 12 * HOUR)
        && time_until_starving < (env::block_timestamp() as u128 + 16 * HOUR)
    {
        return Status::HUNGRY;
    } else if time_until_starving > (env::block_timestamp() as u128 + 8 * HOUR)
        && time_until_starving < (env::block_timestamp() as u128 + 12 * HOUR)
    {
        return Status::STARVING;
    } else {
        return Status::DYING;
    }
}