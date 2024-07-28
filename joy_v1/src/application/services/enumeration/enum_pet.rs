use near_sdk::{env, near_bindgen};

use crate::{
    application::repository::HOUR,
    models::{
        contract::{BattleMetadata, JoychiV1, JoychiV1Ext, Status},
        pet::{self, PetEnum, PetEvolution, PetMetadata},
        BattleId, PetId,
    },
};

#[near_bindgen]
impl PetEnum for JoychiV1 {
    fn get_all_pet_metadata(&self, start: Option<u32>, limit: Option<u32>) -> Vec<PetMetadata> {
        self.all_pet_id
            .iter()
            .skip(start.unwrap_or(0) as usize)
            .take(limit.unwrap_or(20) as usize)
            .map(|x| self.pet_metadata_by_id.get(&x).unwrap())
            .collect()
    }

    fn get_pet_by_pet_id(&self, pet_id: PetId) -> PetMetadata {
        let pet = self.pet_metadata_by_id.get(&pet_id).unwrap();

        pet
    }

    fn get_all_battle_metadata(
        &self,
        start: Option<u32>,
        limit: Option<u32>,
    ) -> Vec<BattleMetadata> {
        self.all_battle_id
            .iter()
            .skip(start.unwrap_or(0) as usize)
            .take(limit.unwrap_or(20) as usize)
            .map(|x| self.battle_metadata_by_id.get(&x).unwrap())
            .collect()
    }

    fn get_battle_by_pet_id(&self, battle_id: BattleId) -> BattleMetadata {
        let battle = self.battle_metadata_by_id.get(&battle_id).unwrap();

        battle
    }

    fn get_status_pet(&self, pet_id: PetId) -> Status {
        let pet = self.pet_metadata_by_id.get(&pet_id).unwrap();

        if pet.time_until_starving > (env::block_timestamp() as u128 + 16 * HOUR) {
            return Status::HAPPY;
        } else if pet.time_until_starving > (env::block_timestamp() as u128 + 12 * HOUR)
            && pet.time_until_starving < (env::block_timestamp() as u128 + 16 * HOUR)
        {
            return Status::HUNGRY;
        } else if pet.time_until_starving > (env::block_timestamp() as u128 + 8 * HOUR)
            && pet.time_until_starving < (env::block_timestamp() as u128 + 12 * HOUR)
        {
            return Status::STARVING;
        } else {
            return Status::DYING;
        }
    }

    fn get_pet_evolution_item(&self, pet_id: PetId) -> PetEvolution {
        let pet: PetMetadata = self.pet_metadata_by_id.get(&pet_id).unwrap();
        let pet_evolution_by_id = self.pet_evolution_metadata_by_id.get(&pet_id).unwrap();

        let pet_evol = pet_evolution_by_id[pet.pet_evolution_phase as usize - 1].clone();

        pet_evol
    }

    fn get_pet_attack_winrate(&self, pet_id: PetId) -> u128 {
        let pet = self.pet_metadata_by_id.get(&pet_id).unwrap();
        let pet_evolution_by_id = self.pet_evolution_metadata_by_id.get(&pet_id).unwrap();
        let pet_evol = pet_evolution_by_id[pet.pet_evolution_phase as usize - 1]
            .clone()
            .attack_win_rate;

        pet_evol
    }

    fn get_pet_image(&self, pet_id: PetId) -> String {
        let pet = self.pet_metadata_by_id.get(&pet_id).unwrap();
        let pet_evolution_by_id = self.pet_evolution_metadata_by_id.get(&pet_id).unwrap();

        let pet_evol = pet_evolution_by_id[pet.pet_evolution_phase as usize - 1]
            .clone()
            .image;

        pet_evol
    }

    fn get_pet_evolution_phase(&self, pet_id: PetId, current_evo_phase: u128) -> u128 {
        let pet = self.pet_metadata_by_id.get(&pet_id).unwrap();

        let evol_phase_pet_now: usize = pet.pet_evolution_phase as usize;

        let pet_evolution = self.pet_evolution_metadata_by_id.get(&pet_id).unwrap();

        let evol_level = pet_evolution[evol_phase_pet_now - 1].next_evolution_level;
        if (pet.level >= evol_level) {
            return self.get_pet_evolution_phase(pet_id, current_evo_phase + 1);
        }
        return current_evo_phase;
    }
}
