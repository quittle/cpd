use crate::{
    Action, ActionError, Actor, Attack, BattleText, Board, BoardItem, Card, CardAction, CardId,
    Character, CharacterId, DeclareWrappedType, Effect, EffectId, GridLocation, Health,
    RandomProvider, Target, Trigger, U64Range, battle_file, battle_markup,
};
use serde::Serialize;
use std::collections::HashMap;
use std::path::PathBuf;
use std::process::ExitCode;

DeclareWrappedType!(TeamId, id, u64);

#[derive(Debug, Serialize)]
pub struct Team {
    pub id: TeamId,
    pub name: String,
}

#[derive(Debug)]
struct Turn {
    character: CharacterId,
}

type StoryCard = battle_file::StoryCard;

#[derive(Serialize)]
pub struct Battle {
    #[serde(skip)]
    pub actors: Vec<(TeamId, Box<dyn Actor>)>,
    pub characters: HashMap<CharacterId, Character>,
    pub introduction: Option<StoryCard>,
    pub teams: Vec<Team>,
    pub history: Vec<BattleText>,
    #[serde(skip)]
    pub random_provider: Box<dyn RandomProvider>,
    pub round: u16,
    pub cards: HashMap<CardId, Card>,
    pub effects: HashMap<EffectId, Effect>,
    pub default_turn_actions: u64,
    #[serde(skip)]
    pub asset_directory: Option<PathBuf>,
    pub board: Board,
    pub background_image: Option<String>,
}

unsafe impl Sync for Battle {}

impl Battle {
    pub fn get_character(&self, actor: &dyn Actor) -> &Character {
        &self.characters[actor.get_character_id()]
    }

    pub fn get_team_for_actor(&self, actor: &dyn Actor) -> Option<TeamId> {
        for (team_id, other_actor) in &self.actors {
            if actor.get_character_id() == other_actor.get_character_id() {
                return Some(*team_id);
            }
        }
        None
    }

    pub fn get_team_from_id(&self, id: TeamId) -> Option<&Team> {
        self.teams.iter().find(|&team| team.id == id)
    }

    fn build_turns(&self) -> Vec<Turn> {
        let mut ret = vec![];
        for (_team_id, actor) in &self.actors {
            if self.get_character(actor.as_ref()).is_dead() {
                continue;
            }

            ret.push(Turn {
                character: *actor.get_character_id(),
            });
        }
        ret
    }

    pub fn get_actor(&self, character_id: &CharacterId) -> Option<&dyn Actor> {
        for (_team_id, actor) in &self.actors {
            if actor.get_character_id() == character_id {
                return Some(actor.as_ref());
            }
        }
        None
    }

    pub fn get_mut_actor(&mut self, character_id: &CharacterId) -> Option<&mut dyn Actor> {
        for (_team_id, actor) in &mut self.actors {
            if actor.get_character_id() == character_id {
                return Some(actor.as_mut());
            }
        }
        None
    }

    pub fn require_actor(&self, character_id: &CharacterId) -> &dyn Actor {
        self.get_actor(character_id)
            .unwrap_or_else(|| panic!("Unable to find actor with character id: {character_id}"))
    }

    pub fn require_mut_actor(&mut self, character_id: &CharacterId) -> &mut dyn Actor {
        self.get_mut_actor(character_id)
            .unwrap_or_else(|| panic!("Unable to find actor with character id: {character_id}"))
    }

    /// Checks if only one team is alive and returns that team. Returns None if multiple teams are alive or if None are
    pub fn check_only_one_team_alive(&self) -> Option<TeamId> {
        let mut cur_id = None;
        for (team_id, actor) in &self.actors {
            if !self.get_character(actor.as_ref()).is_dead() {
                if cur_id.is_some() && cur_id != Some(*team_id) {
                    return None;
                }
                cur_id = Some(*team_id);
            }
        }
        cur_id
    }

    fn get_all_character_amounts_in_range(
        &self,
        target_id: CharacterId,
        area: &U64Range,
        amount: &U64Range,
    ) -> Vec<(CharacterId, u64)> {
        let range = area.resolve(self.random_provider.as_ref());
        let (attack_x, attack_y) = self.board.find(&BoardItem::Character(target_id)).unwrap();

        self.get_characters_in_range(
            GridLocation {
                x: attack_x,
                y: attack_y,
            },
            range,
        )
        .iter()
        .map(|id| (*id, amount.resolve(self.random_provider.as_ref())))
        .collect()
    }

    fn is_in_range(
        &self,
        range: u64,
        character: CharacterId,
        target_character: CharacterId,
    ) -> bool {
        self.board
            .distance(
                BoardItem::Character(character),
                BoardItem::Character(target_character),
            )
            .unwrap_or(0)
            <= range
    }

    fn get_characters_in_range(&self, location: GridLocation, range: u64) -> Vec<CharacterId> {
        self.board
            .find_chars_in_range(location, range.try_into().unwrap())
    }

    /// Attempts to carry out the action. If the action (legal or no) consumes an action, returns true
    fn handle_action(&mut self, actor: &CharacterId, action: Action) -> bool {
        let character = &self.characters[actor];
        match action {
            Action::Pass => {
                self.history.push(battle_markup![
                    @id(&character.name),
                    " took no action",
                ]);
                let character = self.characters.get_mut(actor).unwrap();
                character.remaining_actions = 0;
                character.movement = 0;

                true
            }
            Action::Move(target, location) => {
                if actor != &target || character.movement == 0 {
                    return false;
                }

                if let Some((x, y)) = self.board.find(&BoardItem::Character(target)) {
                    if location.is_adjacent(&GridLocation { x, y })
                        && !matches!(
                            self.board.grid.get(location.x, location.y),
                            Some(BoardItem::Character(_))
                        )
                    {
                        self.characters.get_mut(&target).unwrap().movement -= 1;

                        self.board.grid.clear(x, y);

                        let prev_contents = self.board.grid.set(
                            location.x,
                            location.y,
                            BoardItem::Character(target),
                        );
                        match prev_contents {
                            None => {}
                            Some(BoardItem::Card(card_id)) => {
                                self.characters.get_mut(&target).unwrap().hand.push(card_id);
                            }
                            Some(BoardItem::Character(_)) => {
                                panic!("Character should not be in the way of movement");
                            }
                        }

                        return true;
                    }
                }

                false
            }
            Action::Act(card_id, target_id) => {
                let card = &self.cards[&card_id];
                let actual_target = if *actor == target_id {
                    Target::Me
                } else {
                    Target::Others
                };

                if !card.target().is_super_set(&actual_target) {
                    return false;
                }

                let target_character = &self.characters[&target_id];
                if target_character.is_dead() {
                    return false;
                }

                if !self.is_in_range(card.range, *actor, target_id) {
                    return false;
                }

                if character.remaining_actions == 0 {
                    return false;
                }

                let history_entry = battle_markup![
                    @id(&character.name),
                    " used ",
                    @attack(&card.name),
                    " on ",
                    @id(&target_character.name),
                    ". "
                ];
                self.history.push(history_entry);

                self.characters.get_mut(actor).unwrap().remaining_actions -= 1;

                for action in card.actions.clone() {
                    // If the action specifically targets me, then force it to target the actor
                    // rather than the potentially other target.
                    let target_id = if action.target() == &Target::Me {
                        actor
                    } else {
                        &target_id
                    };

                    self.try_run_card_action(*actor, *target_id, &action);
                }

                // Remove card from hand
                let hand = &mut self.characters.get_mut(actor).unwrap().hand;
                hand.remove(hand.iter().position(|id| id == &card_id).unwrap());
                self.characters
                    .get_mut(actor)
                    .unwrap()
                    .discard
                    .push(card_id);

                true
            }
        }
    }

    pub async fn advance(&mut self) -> Result<(), ExitCode> {
        self.round += 1;
        self.history
            .push(battle_markup![format!("--- Round {}", self.round)]);
        let turns = self.build_turns();
        for turn in turns {
            let character = self.characters.get_mut(&turn.character).unwrap();
            character.refresh_hand(self.random_provider.as_ref());
            character.remaining_actions = character
                .get_default_turn_actions()
                .unwrap_or(self.default_turn_actions);
            character.movement = character.default_movement;

            if character.is_dead() {
                continue;
            }

            while self.characters[&turn.character].remaining_actions > 0
                || self.characters[&turn.character].movement > 0
            {
                let actor: &dyn Actor = self.require_actor(&turn.character);
                let action_result = actor.act(self).await;
                match action_result {
                    Ok(request) => {
                        self.handle_action(&turn.character, request);
                    }
                    Err(ActionError::Failure(failure)) => {
                        println!("Error processing {}: {}", turn.character, failure.message);
                    }
                    Err(ActionError::Exit(exit_code)) => {
                        return Err(exit_code);
                    }
                }
            }
            if self.check_only_one_team_alive().is_some() {
                return Ok(());
            }
        }
        Ok(())
    }

    pub async fn run_to_completion(&mut self) -> Result<(), ExitCode> {
        let mut surviving_team = None;
        while surviving_team.is_none() {
            self.advance().await?;
            surviving_team = self.check_only_one_team_alive()
        }
        let team_id = surviving_team.unwrap();
        let team = self.get_team_from_id(team_id).unwrap();
        self.history
            .push(battle_markup![format!("{} won.", team.name)]);

        for (_, actor) in &self.actors {
            actor.on_game_over(self).await;
        }
        Ok(())
    }

    fn try_run_card_action(
        &mut self,
        actor: CharacterId,
        target_id: CharacterId,
        action: &CardAction,
    ) -> bool {
        let mut history_entry = vec![];

        // If the action specifically targets me, then force it to target the actor
        // rather than the potentially other target.
        let target_id = if action.target() == &Target::Me {
            &actor
        } else {
            &target_id
        };

        let target_character = self.characters.get_mut(target_id).unwrap();
        match action {
            CardAction::Damage { amount, area, .. } => {
                for (attacked_character_id, value) in
                    self.get_all_character_amounts_in_range(*target_id, area, amount)
                {
                    let attacked_character =
                        self.characters.get_mut(&attacked_character_id).unwrap();

                    if attacked_character.is_dead() {
                        continue;
                    }

                    history_entry.extend(battle_markup![@damage(&value), " damage to ", @id(&attacked_character.name), ". " ]);
                    attacked_character.health -= Attack::new(value);

                    if attacked_character.is_dead() {
                        for effect_id in attacked_character.effects.clone() {
                            self.try_run_effect(
                                attacked_character_id,
                                attacked_character_id,
                                effect_id,
                                Trigger::Death,
                            );
                        }
                    }
                }
            }
            CardAction::Heal { amount, area, .. } => {
                for (healed_character_id, value) in
                    self.get_all_character_amounts_in_range(*target_id, area, amount)
                {
                    let healed_character = self.characters.get_mut(&healed_character_id).unwrap();

                    history_entry.extend(battle_markup!["Healed ", @damage(&value), ". "]);

                    healed_character.heal(Health::new(value));
                }
            }
            CardAction::GainAction { amount, .. } => {
                let value = amount.resolve(self.random_provider.as_ref());
                history_entry.extend(battle_markup![format!(
                    "Gained {} action{}. ",
                    value,
                    if value != 1 { "s" } else { "" }
                )]);
                target_character.remaining_actions += value;
            }
            CardAction::Move { amount, .. } => {
                let value = amount.resolve(self.random_provider.as_ref());
                history_entry.extend(battle_markup![format!("Moved {} spaces. ", value)]);
                target_character.movement += value;
            }
        }

        self.history.push(history_entry);

        true
    }

    fn try_run_effect(
        &mut self,
        actor: CharacterId,
        target_id: CharacterId,
        effect_id: EffectId,
        trigger: Trigger,
    ) {
        let effect = &self.effects[&effect_id];
        if !effect.has_trigger(trigger) {
            return;
        }

        for action in effect.actions.clone() {
            self.try_run_card_action(actor, target_id, &action);
        }
    }
}

#[cfg(test)]
mod tests {
    use futures::executor::block_on;

    use crate::{Battle, DefaultRandomProvider};

    #[tokio::test]
    async fn test_deserialize() -> Result<(), String> {
        let battle_json = r#"{
            "title": "Example Game",
            "description": "Example Description",
            "default_hand_size": 2,
            "board": { "width": 2, "height": 2 },
            "cards": [
                {
                    "id": 0,
                    "name": "Kick",
                    "description": "Deal 123 damage",
                    "range": 1,
                    "actions": [
                        {
                            "type": "damage",
                            "target": "others",
                            "amount": 123
                        }
                    ]
                },
                {
                    "id": 1,
                    "name": "Punch",
                    "description": "Deal 456 damage",
                    "range": 888,
                    "actions": [
                        {
                            "type": "damage",
                            "target": "others",
                            "amount": 456
                        }
                    ]
                }
            ],
            "teams": [
                {
                    "name": "Team A",
                    "members": [
                        {
                            "name": "Member A1",
                            "race": "Human",
                            "base_health": 5,
                            "cards": [0],
                            "hand_size": 1,
                            "location": [0, 0]
                        },
                        {
                            "name": "Member A2",
                            "race": "Human",
                            "base_health": 5,
                            "cards": [1],
                            "location": [0, 1]
                        }
                    ]
                },
                {
                    "name": "Team B",
                    "members": [
                        {
                            "name": "Member B1",
                            "race": "Human",
                            "base_health": 15,
                            "cards": [0],
                            "location": [1, 0]
                        }
                    ]
                }
            ]
        }"#;
        let mut battle =
            Battle::deserialize(battle_json, None, Box::<DefaultRandomProvider>::default()).await?;
        assert_eq!(battle.history.len(), 0);
        assert_eq!(battle.teams.len(), 2);
        assert_eq!(battle.teams[0].name, "Team A".to_string());
        assert_eq!(battle.teams[0].id.id, 0);
        assert_eq!(battle.teams[1].name, "Team B".to_string());
        assert_eq!(battle.teams[1].id.id, 1);
        assert_eq!(battle.actors.len(), 3);
        assert_eq!(battle.actors[0].0.id, 0);
        assert_eq!(
            battle.characters[battle.actors[0].1.get_character_id()].name,
            "Member A1"
        );
        assert_eq!(
            battle.characters[battle.actors[0].1.get_character_id()].hand_size,
            1
        );
        assert_eq!(battle.actors[1].0.id, 0);
        assert_eq!(
            battle.characters[battle.actors[1].1.get_character_id()].name,
            "Member A2"
        );
        assert_eq!(
            battle.characters[battle.actors[1].1.get_character_id()].hand_size,
            2
        );
        assert_eq!(battle.actors[2].0.id, 1);
        assert_eq!(
            battle.characters[battle.actors[2].1.get_character_id()].name,
            "Member B1"
        );

        block_on(battle.run_to_completion()).unwrap();
        Ok(())
    }
}
