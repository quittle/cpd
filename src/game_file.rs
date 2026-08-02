use serde::{Deserialize, Serialize};

use crate::map_serde_error;

pub type BattleId = usize;

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Game {
    pub name: String,
    pub description: String,
    pub battles: Vec<Battle>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Battle {
    pub id: BattleId,
    pub data: String,
    pub win: Option<BattleId>,
    pub loss: Option<BattleId>,
}

impl Game {
    pub fn parse_from_str(data: &str) -> Result<Self, String> {
        let game: Game =
            serde_json::from_str::<Game>(data).map_err(|err| map_serde_error(data, err))?;

        for (index, battle) in game.battles.iter().enumerate() {
            if battle.id != index {
                return Err(format!("Battle with id {} should be {}", battle.id, index));
            }

            if let Some(id) = battle.win {
                if id >= game.battles.len() {
                    return Err(format!(
                        "Battle with id {} has win id {} which is out of bounds",
                        battle.id, id
                    ));
                }
            }
            if let Some(id) = battle.loss {
                if id >= game.battles.len() {
                    return Err(format!(
                        "Battle with id {} has loss id {} which is out of bounds",
                        battle.id, id
                    ));
                }
            }
        }

        Ok(game)
    }
}
