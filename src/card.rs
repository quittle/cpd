use crate::{DeclareWrappedType, EffectId, RandomProvider, battle_file};
use schemars::JsonSchema;
use serde::Serialize;

DeclareWrappedType!(CardId, id, battle_file::CardId);

pub type LifeNumber = battle_file::LifeNumber;

DeclareWrappedType!(Chance, chance, u32);

impl Chance {
    pub fn resolve(&self, random_provider: &dyn RandomProvider) -> bool {
        random_provider.pick_linear_u32(0, u32::MAX) <= self.chance
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct U64Range(pub u64, pub u64);

impl U64Range {
    pub fn resolve(&self, random_provider: &dyn RandomProvider) -> LifeNumber {
        random_provider.pick_linear_u64(self.0, self.1)
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub enum Target {
    Me,
    Others,
    Any,
}

impl Target {
    /// Checks if `other` is compatible with `self`
    pub fn is_super_set(&self, other: &Self) -> bool {
        self == other || *self == Self::Any
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub enum CardAction {
    Damage {
        target: Target,
        amount: U64Range,
        area: U64Range,
    },
    Heal {
        target: Target,
        amount: U64Range,
        area: U64Range,
    },
    GainAction {
        target: Target,
        amount: U64Range,
    },
    Move {
        target: Target,
        amount: U64Range,
    },
    Effect {
        target: Target,
        effect: EffectId,
        chance: Chance,
    },
    RemoveEffect {
        target: Target,
        effect: EffectId,
        chance: Chance,
    },
}

impl CardAction {
    pub fn target(&self) -> &Target {
        match self {
            Self::Damage { target, .. } => target,
            Self::Heal { target, .. } => target,
            Self::GainAction { target, .. } => target,
            Self::Move { target, .. } => target,
            Self::Effect { target, .. } => target,
            Self::RemoveEffect { target, .. } => target,
        }
    }
}

#[derive(Debug, Clone, Serialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Card {
    pub id: CardId,
    pub name: String,
    pub description: String,
    pub flavor: Option<String>,
    pub actions: Vec<CardAction>,
    pub range: u64,
}

impl Card {
    /// If any action requires others, the target is Others
    /// If any action supports any and no target is others, the target is Any
    /// If neither are present, the target is Me
    pub fn target(&self) -> Target {
        let mut target = Target::Me;
        for action in &self.actions {
            match action.target() {
                Target::Others => return Target::Others,
                Target::Any => target = Target::Any,
                Target::Me => (),
            }
        }
        target
    }
}
