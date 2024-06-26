use serde::Serialize;

use crate::{battle_file, DeclareWrappedType};

DeclareWrappedType!(CardId, id, battle_file::CardId);

pub type LifeNumber = battle_file::LifeNumber;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum Target {
    Me,
    Others,
}

#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum CardAction {
    Damage { target: Target, amount: LifeNumber },
    Heal { target: Target, amount: LifeNumber },
}

impl CardAction {
    pub fn target(&self) -> &Target {
        match self {
            Self::Damage { target, .. } => target,
            Self::Heal { target, .. } => target,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Card {
    pub id: CardId,
    pub name: String,
    pub description: String,
    pub flavor: Option<String>,
    pub actions: Vec<CardAction>,
}

impl Card {
    pub fn target(&self) -> Target {
        if self
            .actions
            .iter()
            .any(|action| action.target() == &Target::Others)
        {
            Target::Others
        } else {
            Target::Me
        }
    }
}
