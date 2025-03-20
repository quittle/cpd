use std::{
    cmp::min,
    ops::{Sub, SubAssign},
};

use serde::Serialize;

use crate::*;

type HandSize = usize;

DeclareWrappedType!(CharacterId, id, usize);

#[derive(Serialize)]
pub enum CharacterRace {
    Human,
    Machine,
}

DeclareWrappedType!(Attack, damage, u64);

DeclareWrappedType!(Health, health, u64);

impl Sub<Attack> for Health {
    type Output = Self;

    fn sub(self, attack: Attack) -> Self {
        Health::new(self.health.saturating_sub(attack.damage))
    }
}

impl SubAssign<Attack> for Health {
    fn sub_assign(&mut self, attack: Attack) {
        self.health = (*self - attack).health;
    }
}

#[derive(Serialize)]
pub struct Character {
    pub id: CharacterId,
    pub name: String,
    pub effects: Vec<EffectId>,
    pub race: CharacterRace,
    pub hand: Vec<CardId>,
    pub deck: Vec<CardId>,
    pub discard: Vec<CardId>,
    pub health: Health,
    pub max_health: Health,
    pub remaining_actions: u64,
    pub hand_size: HandSize,
    pub image: Option<String>,
    pub movement: u64,
    pub default_movement: u64,
}

impl Character {
    pub fn is_dead(&self) -> bool {
        self.health.health == 0
    }

    pub fn refresh_hand(&mut self, random_provider: &dyn RandomProvider) {
        let cards_to_draw = if self.hand_size >= self.hand.len() {
            self.hand_size - self.hand.len()
        } else {
            0
        };

        if self.deck.len() < cards_to_draw {
            self.deck.extend(self.discard.shuffle(random_provider));
            self.discard.clear();
        }

        self.hand
            .extend(self.deck.drain(..min(self.deck.len(), cards_to_draw)));
    }

    pub fn get_default_turn_actions(&self) -> Option<u64> {
        None
    }

    pub fn heal(&mut self, healing: Health) {
        self.health = min(self.health + healing, self.max_health);
    }
}

#[derive(Clone)]
pub enum CharacterAction {
    Attack { name: String, base_damage: i64 },
}
