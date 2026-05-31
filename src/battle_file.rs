use std::fmt::Display;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::map_serde_error;

pub type LifeNumber = u64;
pub type CardId = usize;
pub type EffectId = usize;
pub type HandSize = usize;
pub type ObjectId = usize;

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Battle {
    pub title: String,
    pub description: String,
    pub board: Board,
    pub introduction: Option<StoryCard>,
    pub default_hand_size: HandSize,
    pub default_movement: Option<u64>,
    #[serde(default)]
    pub effects: Vec<Effect>,
    #[serde(default)]
    pub cards: Vec<Card>,
    #[serde(default)]
    pub objects: Vec<Object>,
    #[serde(default)]
    pub end_conditions: Vec<EndCondition>,
    pub teams: Vec<Team>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct EndCondition {
    pub title: String,
    pub description: String,
    #[serde(rename = "type")]
    pub condition_type: EndConditionType,
    pub condition: EndConditionCriterion,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, JsonSchema, Copy)]
#[serde(rename_all = "lowercase")]
pub enum EndConditionType {
    Win,
    Loss,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum EndConditionCriterion {
    TeamMemberDeath {
        ids: Vec<usize>,
    },
    ObjectOwned {
        character_id: usize,
        object_id: ObjectId,
    },
}

impl Battle {
    pub fn parse_from_str(data: &str) -> Result<Self, String> {
        let battle: Battle =
            serde_json::from_str::<Battle>(data).map_err(|err| map_serde_error(data, err))?;

        for (index, card) in battle.cards.iter().enumerate() {
            if card.id != index {
                return Err(format!("Card with id {} should be {}", card.id, index));
            }

            for action in &card.actions {
                let target = match action {
                    CardAction::Damage { target, .. } => target,
                    CardAction::Heal { target, .. } => target,
                    CardAction::GainAction { target, .. } => target,
                    CardAction::Move { target, .. } => target,
                    CardAction::Effect { target, .. } => target,
                    CardAction::RemoveEffect { target, .. } => target,
                    CardAction::ReduceEffect { target, .. } => target,
                    CardAction::DestroySelf { .. } => &Target::Me,
                };
                if target != &Target::Me && card.range.is_none() {
                    return Err(format!(
                        "Card {} has an action that can target others but without a range specified",
                        card
                    ));
                }
            }
        }

        let mut player_found = false;
        for team in &battle.teams {
            for team_member in &team.members {
                if team_member.is_player {
                    if player_found {
                        return Err("Multiple playable team members found.")?;
                    }
                    player_found = true;
                }
            }
        }

        Ok(battle)
    }
}

pub type StoryCard = Vec<StoryCardEntry>;

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub enum StoryCardEntry {
    H1(String),
    P(String),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged, deny_unknown_fields)]
pub enum LocationRange {
    Point(usize, usize),
    Range((usize, usize), (usize, usize)),
}

impl LocationRange {
    pub fn iter(&self) -> LocationRangeIter {
        match self {
            LocationRange::Point(x, y) => LocationRangeIter {
                start: (*x, *y),
                end: (*x, *y),
                current: None,
            },
            LocationRange::Range((start_x, start_y), (end_x, end_y)) => LocationRangeIter {
                start: (*start_x, *start_y),
                end: (*end_x, *end_y),
                current: None,
            },
        }
    }
}

pub struct LocationRangeIter {
    start: (usize, usize),
    end: (usize, usize),
    current: Option<(usize, usize)>,
}

impl Iterator for LocationRangeIter {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(cur @ (cur_x, cur_y)) = self.current {
            if cur == self.end {
                return None;
            }
            if cur_x < self.end.0 {
                self.current = Some((cur_x + 1, cur_y));
            } else {
                self.current = Some((self.start.0, cur_y + 1));
            }
        } else {
            self.current = Some(self.start);
        }
        self.current
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged, rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
pub enum Cell {
    Card {
        card: CardId,
        location: LocationRange,
    },
    Inert {
        inert: bool,
        location: LocationRange,
    },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct BoardBackground {
    pub image: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Board {
    pub width: usize,
    pub height: usize,
    pub background: Option<BoardBackground>,
    pub cells: Option<Vec<Cell>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Team {
    pub name: String,
    pub members: Vec<TeamMember>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub enum Content {
    Card(CardId),
    Object(ObjectId),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct TeamMember {
    pub name: String,
    pub race: Race,
    pub base_health: LifeNumber,
    pub max_health: Option<LifeNumber>,
    pub cards: Vec<CardId>,
    #[serde(default)]
    pub effects: Vec<EffectId>,
    #[serde(default)]
    pub contains: Vec<Content>,
    pub hand_size: Option<HandSize>,
    #[serde(default)]
    pub is_player: bool,
    pub image: Option<String>,
    pub location: (usize, usize),
    pub movement: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub enum Race {
    Human,
    Machine,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub enum Target {
    #[serde(alias = "self")]
    Me,
    #[serde(alias = "other")]
    Others,
    #[serde(alias = "any")]
    Any,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(untagged, deny_unknown_fields)]
pub enum MaybeU64Range {
    Range(u64, u64),
    Absolute(u64),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(deny_unknown_fields)]
pub struct Effect {
    pub id: EffectId,
    pub name: String,
    pub description: String,
    pub image: Option<String>,
    pub triggers: Option<Vec<Trigger>>,
    pub actions: Vec<CardAction>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub enum Trigger {
    Death,
    TurnStart,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum CardAction {
    Damage {
        target: Target,
        amount: MaybeU64Range,
        area: Option<MaybeU64Range>,
    },
    Heal {
        target: Target,
        amount: MaybeU64Range,
        area: Option<MaybeU64Range>,
    },
    GainAction {
        target: Target,
        amount: MaybeU64Range,
    },
    Move {
        target: Target,
        amount: MaybeU64Range,
    },
    Effect {
        target: Target,
        effect: EffectId,
        chance: Option<f64>,
    },
    RemoveEffect {
        target: Target,
        effect: EffectId,
        chance: Option<f64>,
    },
    ReduceEffect {
        target: Target,
        effect: EffectId,
        amount: Option<u64>,
        chance: Option<f64>,
    },
    DestroySelf {
        chance: Option<f64>,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Card {
    pub id: CardId,
    pub name: String,
    pub description: String,
    pub flavor: Option<String>,
    pub actions: Vec<CardAction>,
    pub range: Option<u64>,
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Card \"{}\" ({})", self.name, self.id)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Object {
    pub id: ObjectId,
    pub name: String,
    pub description: String,
    pub flavor: Option<String>,
    pub image: Option<String>,
}

#[cfg(test)]
mod tests {
    use crate::battle_file::*;

    use super::Battle;

    #[test]
    fn test_deserialize() -> Result<(), String> {
        let data = r#"{
            "title": "Example Game",
            "description": "Example Description",
            "default_hand_size": 5,
            "introduction": [
                { "h1": "Heading" },
                { "p": "Paragraph" }
            ],
            "board": {
                "width": 1,
                "height": 1,
                "cells": [
                    {
                        "card": 0,
                        "location": [0, 0]
                    },
                    {
                        "inert": true,
                        "location": [0, 1]
                    }
                ]
            },
            "effects": [
                {
                    "id": 0,
                    "name": "effect name",
                    "description": "effec description",
                    "triggers": ["death"],
                    "actions": [
                        {
                            "type": "damage",
                            "target": "self",
                            "amount": 1,
                            "area": 1
                        }
                    ]
                }
            ],
            "cards": [
                {
                    "id": 0,
                    "name": "Kick",
                    "description": "description text",
                    "flavor": "flavor text",
                    "range": 1,
                    "actions": [
                        {
                            "type": "damage",
                            "target": "others",
                            "amount": 123,
                            "area": 2
                        }
                    ]
                }
            ],
            "teams": [
                {
                    "name": "Team A",
                    "members": [
                        {
                            "name": "Member 1",
                            "race": "Human",
                            "base_health": 10,
                            "cards": [0],
                            "effects": [0],
                            "location": [0, 0],
                            "is_player": true
                        }
                    ]
                }
            ]
        }"#;

        let battle: Battle = Battle::parse_from_str(data)?;
        assert_eq!(
            battle.cards[battle.teams[0].members[0].cards[0]].actions[0],
            CardAction::Damage {
                target: Target::Others,
                amount: MaybeU64Range::Absolute(123),
                area: Some(MaybeU64Range::Absolute(2)),
            }
        );

        Ok(())
    }

    #[test]
    fn test_multi_player_error() {
        let data = r#"{
            "title": "Example Game",
            "description": "Example Description",
            "default_hand_size": 5,
            "cards": [],
            "board": { "width": 1, "height": 1 },
            "teams": [
                {
                    "name": "Team A",
                    "members": [
                        {
                            "name": "Member 1",
                            "is_player": true,
                            "race": "Human",
                            "base_health": 10,
                            "cards": [],
                            "location": [0, 0]
                        },
                        {
                            "name": "Member 2",
                            "is_player": true,
                            "race": "Human",
                            "base_health": 10,
                            "cards": [],
                            "location": [0, 0]
                        }
                    ]
                }
            ]
        }"#;

        let maybe_battle = Battle::parse_from_str(data);

        assert!(maybe_battle.is_err());
        assert_eq!(
            maybe_battle.unwrap_err(),
            "Multiple playable team members found."
        );
    }

    #[test]
    fn test_end_conditions_object_owned() -> Result<(), String> {
        let data = r#"{
            "title": "Test Battle",
            "description": "Test",
            "default_hand_size": 1,
            "board": { "width": 1, "height": 1 },
            "cards": [],
            "objects": [
                {
                    "id": 0,
                    "name": "Key",
                    "description": "A key"
                }
            ],
            "end_conditions": [
                {
                    "title": "Find the Key",
                    "description": "Locate the key",
                    "type": "win",
                    "condition": {
                        "type": "object_owned",
                        "object_id": 0,
                        "character_id": 0
                    }
                }
            ],
            "teams": [
                {
                    "name": "Team A",
                    "members": [
                        {
                            "name": "Player",
                            "is_player": true,
                            "race": "Human",
                            "base_health": 10,
                            "cards": [],
                            "location": [0, 0]
                        }
                    ]
                }
            ]
        }"#;

        let battle: Battle = Battle::parse_from_str(data)?;

        assert_eq!(battle.end_conditions.len(), 1);
        assert_eq!(battle.end_conditions[0].title, "Find the Key");
        assert_eq!(
            battle.end_conditions[0].condition_type,
            EndConditionType::Win
        );
        match &battle.end_conditions[0].condition {
            EndConditionCriterion::ObjectOwned { object_id, .. } => assert_eq!(*object_id, 0),
            _ => panic!("Expected ObjectOwned condition"),
        }

        Ok(())
    }

    #[test]
    fn test_end_conditions_team_member_death() -> Result<(), String> {
        let data = r#"{
            "title": "Test Battle",
            "description": "Test",
            "default_hand_size": 1,
            "board": { "width": 1, "height": 1 },
            "cards": [],
            "end_conditions": [
                {
                    "title": "Enemy Defeated",
                    "description": "All enemies must be defeated",
                    "type": "win",
                    "condition": {
                        "type": "team_member_death",
                        "ids": [1, 2]
                    }
                }
            ],
            "teams": [
                {
                    "name": "Team A",
                    "members": [
                        {
                            "name": "Player",
                            "is_player": true,
                            "race": "Human",
                            "base_health": 10,
                            "cards": [],
                            "location": [0, 0]
                        }
                    ]
                },
                {
                    "name": "Team B",
                    "members": [
                        {
                            "name": "Enemy 1",
                            "race": "Human",
                            "base_health": 10,
                            "cards": [],
                            "location": [0, 0]
                        },
                        {
                            "name": "Enemy 2",
                            "race": "Human",
                            "base_health": 10,
                            "cards": [],
                            "location": [0, 0]
                        }
                    ]
                }
            ]
        }"#;

        let battle: Battle = Battle::parse_from_str(data)?;

        assert_eq!(battle.end_conditions.len(), 1);
        assert_eq!(battle.end_conditions[0].title, "Enemy Defeated");
        assert_eq!(
            battle.end_conditions[0].condition_type,
            EndConditionType::Win
        );
        match &battle.end_conditions[0].condition {
            EndConditionCriterion::TeamMemberDeath { ids } => {
                assert_eq!(ids.len(), 2);
                assert_eq!(ids[0], 1);
                assert_eq!(ids[1], 2);
            }
            _ => panic!("Expected TeamMemberDeath condition"),
        }

        Ok(())
    }

    #[test]
    fn test_end_conditions_loss_condition() -> Result<(), String> {
        let data = r#"{
            "title": "Test Battle",
            "description": "Test",
            "default_hand_size": 1,
            "board": { "width": 1, "height": 1 },
            "cards": [],
            "end_conditions": [
                {
                    "title": "Game Over",
                    "description": "You are defeated",
                    "type": "loss",
                    "condition": {
                        "type": "team_member_death",
                        "ids": [0]
                    }
                }
            ],
            "teams": [
                {
                    "name": "Team A",
                    "members": [
                        {
                            "name": "Player",
                            "is_player": true,
                            "race": "Human",
                            "base_health": 10,
                            "cards": [],
                            "location": [0, 0]
                        }
                    ]
                }
            ]
        }"#;

        let battle: Battle = Battle::parse_from_str(data)?;

        assert_eq!(battle.end_conditions.len(), 1);
        assert_eq!(
            battle.end_conditions[0].condition_type,
            EndConditionType::Loss
        );

        Ok(())
    }

    #[test]
    fn test_multiple_end_conditions() -> Result<(), String> {
        let data = r#"{
            "title": "Test Battle",
            "description": "Test",
            "default_hand_size": 1,
            "board": { "width": 1, "height": 1 },
            "cards": [],
            "objects": [
                {
                    "id": 0,
                    "name": "Treasure",
                    "description": "Gold"
                }
            ],
            "end_conditions": [
                {
                    "title": "Find Treasure",
                    "description": "Get the treasure",
                    "type": "win",
                    "condition": {
                        "type": "object_owned",
                        "object_id": 0,
                        "character_id": 0
                    }
                },
                {
                    "title": "Find Artifact",
                    "description": "Alternative win condition",
                    "type": "win",
                    "condition": {
                        "type": "team_member_death",
                        "ids": [1, 2]
                    }
                }
            ],
            "teams": [
                {
                    "name": "Team A",
                    "members": [
                        {
                            "name": "Player",
                            "is_player": true,
                            "race": "Human",
                            "base_health": 10,
                            "cards": [],
                            "location": [0, 0]
                        }
                    ]
                }
            ]
        }"#;

        let battle: Battle = Battle::parse_from_str(data)?;

        assert_eq!(battle.end_conditions.len(), 2);
        assert_eq!(battle.end_conditions[0].title, "Find Treasure");
        assert_eq!(battle.end_conditions[1].title, "Find Artifact");

        Ok(())
    }

    #[test]
    fn test_empty_end_conditions() -> Result<(), String> {
        let data = r#"{
            "title": "Test Battle",
            "description": "Test",
            "default_hand_size": 1,
            "board": { "width": 1, "height": 1 },
            "cards": [],
            "teams": [
                {
                    "name": "Team A",
                    "members": [
                        {
                            "name": "Player",
                            "is_player": true,
                            "race": "Human",
                            "base_health": 10,
                            "cards": [],
                            "location": [0, 0]
                        }
                    ]
                }
            ]
        }"#;

        let battle: Battle = Battle::parse_from_str(data)?;

        assert_eq!(battle.end_conditions.len(), 0);

        Ok(())
    }
}
