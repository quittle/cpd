use std::path::PathBuf;

use crate::{
    Actor, Battle, Board, BoardItem, CardId, CardInstance, CardInstanceId, Character, CharacterId,
    CharacterRace, DumbActor, EffectId, EndCondition, EndConditionCriterion, Health, NumericExt,
    ObjectId, ObjectInstance, ObjectInstanceId, RandomProvider, Team, TeamId, TerminalActor,
    U64Range, battle_file, web_actor::WebActor,
};
use futures::future::join_all;

pub fn normalize_maybe_u64_range(life_number_range: &battle_file::MaybeU64Range) -> U64Range {
    match *life_number_range {
        battle_file::MaybeU64Range::Absolute(value) => U64Range(value, value),
        battle_file::MaybeU64Range::Range(low, high) => U64Range(low, high),
    }
}

fn validate_ids<T>(entries: &[T], id_extractor: impl Fn(&T) -> usize) -> Result<(), String>
where
    T: std::fmt::Debug,
{
    for (index, entry) in entries.iter().enumerate() {
        if index != id_extractor(entry) {
            return Err(format!(
                "ID mismatch at index {} but found ({:?})",
                index, entry,
            ));
        }
    }
    Ok(())
}

fn get_player_team_id(battle: &battle_file::Battle) -> Result<usize, String> {
    for (team_id, team) in battle.teams.iter().enumerate() {
        if team.members.iter().any(|member| member.is_player) {
            return Ok(team_id);
        }
    }
    Err("No player team found".to_string())
}

fn validate_and_get_additional_end_conditions(
    battle: &battle_file::Battle,
) -> Result<Vec<battle_file::EndCondition>, String> {
    let team_member_ids = get_all_team_character_ids(battle);
    for end_condition in battle.end_conditions.iter() {
        match &end_condition.condition {
            battle_file::EndConditionCriterion::TeamMemberDeath { ids } => {
                for id in ids {
                    if !team_member_ids
                        .iter()
                        .any(|(_team_id, character_id, _team_member)| character_id == id)
                    {
                        return Err(format!(
                            "End condition references non-existent character id {id}"
                        ));
                    }
                }
            }
            battle_file::EndConditionCriterion::ObjectOwned {
                character_id,
                object_id,
            } => {
                if !team_member_ids
                    .iter()
                    .any(|(_team_id, iter_character_id, _team_member)| {
                        iter_character_id == character_id
                    })
                {
                    return Err(format!(
                        "End condition references object id {object_id} that is already owned by a character"
                    ));
                }
                if !battle.objects.iter().any(|object| object.id == *object_id) {
                    return Err(format!(
                        "End condition references non-existent object id {object_id}"
                    ));
                }
            }
        }
    }

    let has_win_condition = battle
        .end_conditions
        .iter()
        .any(|ec| ec.condition_type == battle_file::EndConditionType::Win);
    let has_loss_condition = battle
        .end_conditions
        .iter()
        .any(|ec| ec.condition_type == battle_file::EndConditionType::Loss);

    let mut additional_end_conditions = vec![];

    if !has_win_condition {
        let player_team_id = get_player_team_id(battle)?;
        let enemy_ids: Vec<usize> = get_all_team_character_ids(battle)
            .iter()
            .filter_map(|(team_id, character_id, _team_member)| {
                if *team_id == player_team_id {
                    None
                } else {
                    Some(*character_id)
                }
            })
            .collect();

        additional_end_conditions.push(battle_file::EndCondition {
            title: "Victory".to_string(),
            description: "Game over when all enemies are defeated".to_string(),
            condition_type: battle_file::EndConditionType::Win,
            condition: battle_file::EndConditionCriterion::TeamMemberDeath { ids: enemy_ids },
        });
    }

    if !has_loss_condition {
        additional_end_conditions.push(battle_file::EndCondition {
            title: "Defeated".to_string(),
            description: "Game over when you are defeated".to_string(),
            condition_type: battle_file::EndConditionType::Loss,
            condition: battle_file::EndConditionCriterion::TeamMemberDeath {
                ids: battle
                    .teams
                    .iter()
                    .flat_map(|team| &team.members)
                    .enumerate()
                    .filter_map(|(i, member)| if member.is_player { Some(i) } else { None })
                    .collect(),
            },
        });
    }

    Ok(additional_end_conditions)
}

fn get_all_team_character_ids(
    battle: &battle_file::Battle,
) -> Vec<(usize, usize, &battle_file::TeamMember)> {
    battle
        .teams
        .iter()
        .enumerate()
        .flat_map(|(team_id, team)| team.members.iter().map(move |member| (team_id, member)))
        .enumerate()
        .map(|(character_id, (team_id, team_member))| (team_id, character_id, team_member))
        .collect()
}

impl Battle {
    pub async fn deserialize(
        data: &str,
        asset_directory: Option<PathBuf>,
        random_provider: Box<dyn RandomProvider>,
    ) -> Result<Self, String> {
        let battle = battle_file::Battle::parse_from_str(data)?;

        validate_ids(&battle.cards, |entry| entry.id)?;
        validate_ids(&battle.effects, |entry| entry.id)?;
        validate_ids(&battle.objects, |entry| entry.id)?;

        let mut end_conditions = validate_and_get_additional_end_conditions(&battle)?;
        end_conditions.extend(battle.end_conditions.clone());

        let mut board = Board::new(battle.board.width, battle.board.height);

        let mut current_card_instance_id = 0usize;
        let mut current_object_instance_id = 0usize;

        for cell in battle.board.cells.as_ref().into_iter().flatten() {
            match cell {
                battle_file::Cell::Card { card, location } => {
                    for (x, y) in location.iter() {
                        if !board.grid.is_valid(x, y) {
                            return Err(format!("Invalid card position: {x}, {y}"));
                        }
                        board.grid.set(
                            x,
                            y,
                            BoardItem::Card(CardInstance {
                                card_id: CardId::new(*card),
                                card_instance_id: CardInstanceId::new(
                                    current_card_instance_id.inc(),
                                ),
                            }),
                        );
                    }
                }
                battle_file::Cell::Inert { location, .. } => {
                    for (x, y) in location.iter() {
                        if !board.grid.is_valid(x, y) {
                            return Err(format!("Invalid inert position: {x}, {y}"));
                        }
                        board.grid.set(x, y, BoardItem::Inert);
                    }
                }
            }
        }

        let team_character_ids = get_all_team_character_ids(&battle);

        for (_team_id, character_id, team_member) in &team_character_ids {
            let (x, y) = team_member.location;
            if !board.grid.is_valid(x, y) {
                return Err(format!("Invalid team member position: {x}, {y}"));
            }
            if let Some(_prev_id) =
                board
                    .grid
                    .set(x, y, BoardItem::Character(CharacterId::new(*character_id)))
            {
                return Err(format!("Multiple entries found at {x}, {y}"));
            }

            for item in &team_member.contains {
                match item {
                    battle_file::Content::Card(id) => {
                        if battle.cards.len() <= *id {
                            return Err(format!("Invalid card id {id} for {}", team_member.name));
                        }
                    }
                    battle_file::Content::Object(id) => {
                        if battle.objects.len() <= *id {
                            return Err(format!("Invalid object id {id} for {}", team_member.name));
                        }
                    }
                }
            }
        }

        let canonical_asset_directory =
            asset_directory.map(|path_buf| path_buf.canonicalize().unwrap());
        let asset_directory = canonical_asset_directory.as_deref();
        let deserialized_battle = Battle {
            history: vec![],
            introduction: battle.introduction.clone(),
            random_provider,
            default_turn_actions: 1,
            background_image: battle
                .board
                .background
                .as_ref()
                .and_then(|background| background.image.clone()),
            characters: battle
                .teams
                .iter()
                .flat_map(|team| &team.members)
                .enumerate()
                .map(|(index, member)| {
                    (
                        CharacterId::new(index),
                        Character {
                            id: CharacterId::new(index),
                            name: member.name.clone(),
                            effects: member
                                .effects
                                .iter()
                                .map(|effect| EffectId::new(*effect))
                                .collect(),
                            race: match member.race {
                                battle_file::Race::Human => CharacterRace::Human,
                                battle_file::Race::Machine => CharacterRace::Machine,
                            },
                            hand: vec![],
                            remaining_actions: 0,
                            image: member.image.clone(),
                            deck: member
                                .cards
                                .iter()
                                .map(|card_id| CardInstance {
                                    card_id: CardId::new(*card_id),
                                    card_instance_id: CardInstanceId::new(
                                        current_card_instance_id.inc(),
                                    ),
                                })
                                .collect(),
                            discard: vec![],
                            health: Health::new(member.base_health),
                            max_health: Health::new(
                                member.max_health.unwrap_or(member.base_health),
                            ),
                            hand_size: member.hand_size.unwrap_or(battle.default_hand_size),
                            movement: 0,
                            default_movement: member
                                .movement
                                .unwrap_or(battle.default_movement.unwrap_or(0)),
                            contains: member
                                .contains
                                .iter()
                                .map(|content| match content {
                                    battle_file::Content::Card(id) => {
                                        crate::Content::Card(CardInstance::new(
                                            CardId::new(*id),
                                            CardInstanceId::new(current_card_instance_id.inc()),
                                        ))
                                    }
                                    battle_file::Content::Object(id) => {
                                        crate::Content::Object(ObjectInstance::new(
                                            ObjectId::new(*id),
                                            ObjectInstanceId::new(current_object_instance_id.inc()),
                                        ))
                                    }
                                })
                                .collect(),
                        },
                    )
                })
                .collect(),
            cards: battle
                .cards
                .iter()
                .map(|card| (CardId::new(card.id), deserialize_card(card)))
                .collect(),
            effects: battle
                .effects
                .iter()
                .enumerate()
                .map(|(index, effect)| (EffectId::new(index), deserialize_effect(effect)))
                .collect(),
            objects: battle
                .objects
                .iter()
                .enumerate()
                .map(|(index, object)| (ObjectId::new(index), deserialize_object(object)))
                .collect(),
            teams: battle
                .teams
                .iter()
                .enumerate()
                .map(|(index, team)| Team {
                    id: TeamId::new(index.try_into().unwrap()),
                    name: team.name.clone(),
                })
                .collect(),
            actors: join_all(team_character_ids.iter().map(
                move |(team_id, character_id, team_member)| {
                    let is_player = team_member.is_player;
                    async move {
                        let character_id = CharacterId::new(*character_id);
                        (
                            TeamId::new((*team_id).try_into().unwrap()),
                            if is_player {
                                if cfg!(feature = "terminal_ui") {
                                    Box::new(TerminalActor { character_id }) as Box<dyn Actor>
                                } else {
                                    Box::new(
                                        WebActor::new(character_id, asset_directory).await.unwrap(),
                                    ) as Box<dyn Actor>
                                }
                            } else {
                                Box::new(DumbActor { character_id }) as Box<dyn Actor>
                            },
                        )
                    }
                },
            ))
            .await,
            round: 0,
            board,
            asset_directory: canonical_asset_directory.clone(),
            end_conditions: end_conditions
                .iter()
                .map(|condition| EndCondition {
                    title: condition.title.clone(),
                    description: condition.description.clone(),
                    condition_type: condition.condition_type,
                    condition: match &condition.condition {
                        battle_file::EndConditionCriterion::TeamMemberDeath { ids } => {
                            EndConditionCriterion::TeamMemberDeath {
                                ids: ids.iter().map(|id| CharacterId::new(*id)).collect(),
                            }
                        }
                        battle_file::EndConditionCriterion::ObjectOwned {
                            character_id,
                            object_id,
                        } => EndConditionCriterion::ObjectOwned {
                            character_id: CharacterId::new(*character_id),
                            object_id: ObjectId::new(*object_id),
                        },
                    },
                })
                .collect(),
            end_state: None,
        };
        Ok(deserialized_battle)
    }
}

fn deserialize_card_action(card_action: &battle_file::CardAction) -> crate::CardAction {
    match card_action {
        battle_file::CardAction::Damage {
            target,
            amount,
            area,
        } => crate::CardAction::Damage {
            target: deserialize_target(target),
            amount: normalize_maybe_u64_range(amount),
            area: area
                .as_ref()
                .map(normalize_maybe_u64_range)
                .unwrap_or(U64Range(0, 0)),
        },
        battle_file::CardAction::Heal {
            target,
            amount,
            area,
        } => crate::CardAction::Heal {
            target: deserialize_target(target),
            amount: normalize_maybe_u64_range(amount),
            area: area
                .as_ref()
                .map(normalize_maybe_u64_range)
                .unwrap_or(U64Range(0, 0)),
        },
        battle_file::CardAction::GainAction { target, amount } => crate::CardAction::GainAction {
            target: deserialize_target(target),
            amount: normalize_maybe_u64_range(amount),
        },
        battle_file::CardAction::Move { target, amount } => crate::CardAction::Move {
            target: deserialize_target(target),
            amount: normalize_maybe_u64_range(amount),
        },
        battle_file::CardAction::Effect {
            target,
            effect,
            chance,
        } => crate::CardAction::Effect {
            target: deserialize_target(target),
            effect: EffectId::new(*effect),
            chance: deserialize_chance(chance),
        },
        battle_file::CardAction::RemoveEffect {
            target,
            effect,
            chance,
        } => crate::CardAction::RemoveEffect {
            target: deserialize_target(target),
            effect: EffectId::new(*effect),
            chance: deserialize_chance(chance),
        },
        battle_file::CardAction::ReduceEffect {
            target,
            effect,
            amount,
            chance,
        } => crate::CardAction::ReduceEffect {
            target: deserialize_target(target),
            effect: EffectId::new(*effect),
            amount: amount.unwrap_or(1),
            chance: deserialize_chance(chance),
        },
        battle_file::CardAction::DestroySelf { chance } => crate::CardAction::DestroySelf {
            chance: deserialize_chance(chance),
        },
    }
}

fn deserialize_chance(chance: &Option<f64>) -> crate::Chance {
    let real_chance = chance.unwrap_or(1f64);
    crate::Chance::new(((u32::MAX as f64) * real_chance) as u32)
}

fn deserialize_effect(effect: &battle_file::Effect) -> crate::Effect {
    crate::Effect {
        id: crate::EffectId::new(effect.id),
        name: effect.name.clone(),
        description: effect.description.clone(),
        image: effect.image.clone(),
        actions: effect.actions.iter().map(deserialize_card_action).collect(),
        triggers: effect
            .triggers
            .as_ref()
            .map(|triggers| triggers.iter().map(deserailize_trigger).collect())
            .unwrap_or_default(),
    }
}

fn deserialize_object(object: &battle_file::Object) -> crate::Object {
    crate::Object {
        id: crate::ObjectId::new(object.id),
        name: object.name.clone(),
        description: object.description.clone(),
        image: object.image.clone(),
    }
}

fn deserialize_card(card: &battle_file::Card) -> crate::Card {
    crate::Card {
        id: crate::CardId::new(card.id),
        name: card.name.clone(),
        description: card.description.clone(),
        flavor: card.flavor.clone(),
        range: card.range.unwrap_or(0),
        actions: card.actions.iter().map(deserialize_card_action).collect(),
    }
}

fn deserialize_target(target: &battle_file::Target) -> crate::Target {
    match target {
        battle_file::Target::Me => crate::Target::Me,
        battle_file::Target::Others => crate::Target::Others,
        battle_file::Target::Any => crate::Target::Any,
    }
}

fn deserailize_trigger(trigger: &battle_file::Trigger) -> crate::Trigger {
    match trigger {
        battle_file::Trigger::Death => crate::Trigger::Death,
        battle_file::Trigger::TurnStart => crate::Trigger::TurnStart,
    }
}

#[cfg(test)]
mod tests {
    use crate::{Chance, battle_deserialize::deserialize_chance};

    #[test]
    fn test_deserialize_chance() {
        assert_eq!(deserialize_chance(&None), Chance::new(u32::MAX));
        assert_eq!(deserialize_chance(&Some(1f64)), Chance::new(u32::MAX));
        assert_eq!(deserialize_chance(&Some(0f64)), Chance::new(0));
        assert_eq!(deserialize_chance(&Some(0.5)), Chance::new(u32::MAX / 2));
    }
}
