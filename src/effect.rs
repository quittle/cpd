use serde::Serialize;

use crate::{battle_file, CardAction, DeclareWrappedType, Target};

DeclareWrappedType!(EffectId, id, battle_file::EffectId);

#[derive(Debug, Clone, Serialize)]
pub enum Trigger {
    Death,
}

#[derive(Debug, Clone, Serialize)]
pub struct Effect {
    pub id: EffectId,
    pub name: String,
    pub description: String,
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
}
