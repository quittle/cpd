use std::path::PathBuf;

use crate::{
    Actor, Battle, Board, BoardItem, CardId, Character, CharacterId, CharacterRace, DumbActor,
    EffectId, Health, ObjectId, RandomProvider, Team, TeamId, TerminalActor, U64Range, battle_file,
    web_actor::WebActor,
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

        let mut board = Board::new(battle.board.width, battle.board.height);

        for cell in &battle.board.cells.unwrap_or_default() {
            match cell {
                battle_file::Cell::Card { card, location } => {
                    for (x, y) in location.iter() {
                        if !board.grid.is_valid(x, y) {
                            return Err(format!("Invalid card position: {x}, {y}"));
                        }
                        board.grid.set(x, y, BoardItem::Card(CardId::new(*card)));
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

        let max_team_size = battle
            .teams
            .iter()
            .map(|team| team.members.len())
            .max()
            .unwrap_or(0);
        {
            for (team_index, team) in battle.teams.iter().enumerate() {
                for (index, member) in team.members.iter().enumerate() {
                    let (x, y) = member.location;
                    if !board.grid.is_valid(x, y) {
                        return Err(format!("Invalid team member position: {x}, {y}"));
                    }
                    // Makes strong assumptions about the way character ids are picked, incrementing in the same order of team and member
                    if let Some(_prev_id) = board.grid.set(
                        x,
                        y,
                        BoardItem::Character(CharacterId::new(team_index * max_team_size + index)),
                    ) {
                        return Err(format!("Multiple entries found at {x}, {y}"));
                    }

                    for item in &member.contains {
                        match item {
                            battle_file::Content::Card(id) => {
                                if battle.cards.len() <= *id {
                                    return Err(format!(
                                        "Invalid card id {id} for {}",
                                        member.name
                                    ));
                                }
                            }
                            battle_file::Content::Object(id) => {
                                if battle.objects.len() <= *id {
                                    return Err(format!(
                                        "Invalid object id {id} for {}",
                                        member.name
                                    ));
                                }
                            }
                        }
                    }
                }
            }
        }

        let canonical_asset_directory =
            asset_directory.map(|path_buf| path_buf.canonicalize().unwrap());
        let asset_directory = canonical_asset_directory.as_deref();
        Ok(Battle {
            history: vec![],
            introduction: battle.introduction,
            random_provider,
            default_turn_actions: 1,
            background_image: battle
                .board
                .background
                .and_then(|background| background.image),
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
                                .map(|card_id| CardId::new(*card_id))
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
                                        crate::Content::Card(CardId::new(*id))
                                    }
                                    battle_file::Content::Object(id) => {
                                        crate::Content::Object(ObjectId::new(*id))
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
            actors: join_all(
                battle
                    .teams
                    .iter()
                    .enumerate()
                    .flat_map(|(team_index, team)| {
                        team.members
                            .iter()
                            .enumerate()
                            .map(move |(member_index, team_member)| {
                                let character_id =
                                    CharacterId::new(team_index * max_team_size + member_index);
                                async move {
                                    (
                                        TeamId::new(team_index.try_into().unwrap()),
                                        if team_member.is_player {
                                            if cfg!(feature = "terminal_ui") {
                                                Box::new(TerminalActor { character_id })
                                                    as Box<dyn Actor>
                                            } else {
                                                Box::new(
                                                    WebActor::new(character_id, asset_directory)
                                                        .await
                                                        .unwrap(),
                                                )
                                                    as Box<dyn Actor>
                                            }
                                        } else {
                                            Box::new(DumbActor { character_id }) as Box<dyn Actor>
                                        },
                                    )
                                }
                            })
                    }),
            )
            .await,
            round: 0,
            board,
            asset_directory: canonical_asset_directory,
        })
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
