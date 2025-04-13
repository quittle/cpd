use serde::Serialize;

use crate::{CardAction, DeclareWrappedType, Target, battle_file};

DeclareWrappedType!(EffectId, id, battle_file::EffectId);

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub enum Trigger {
    Death,
    TurnStart,
}

#[derive(Debug, Clone, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Effect {
    pub id: EffectId,
    pub name: String,
    pub description: String,
    pub image: Option<String>,
    pub actions: Vec<CardAction>,
    pub triggers: Vec<Trigger>,
}

impl Effect {
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

    pub fn has_trigger(&self, trigger: Trigger) -> bool {
        self.triggers.contains(&trigger)
    }
}
