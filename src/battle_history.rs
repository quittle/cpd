use std::fmt::Display;

use schemars::JsonSchema;
use serde::Serialize;

use crate::TemplateEntry;

#[derive(Clone, Serialize, JsonSchema)]
pub enum BattleTextEntry {
    Id,
    Attack,
    Damage,
}

impl BattleTextEntry {
    pub fn id(text: &dyn Display) -> TemplateEntry<Self> {
        TemplateEntry::Typed(Self::Id, text.to_string())
    }

    pub fn attack(text: &dyn Display) -> TemplateEntry<Self> {
        TemplateEntry::Typed(Self::Attack, text.to_string())
    }

    pub fn damage(text: &dyn Display) -> TemplateEntry<Self> {
        TemplateEntry::Typed(Self::Damage, text.to_string())
    }
}

#[macro_export]
macro_rules! battle_markup {
    ( $($tokens:tt)*  ) => {
        {
            use $crate::BattleTextEntry;
            use $crate::markup;
            markup!(BattleTextEntry: [$($tokens)*])
        }
    }
}

pub type BattleText = Vec<TemplateEntry<BattleTextEntry>>;
