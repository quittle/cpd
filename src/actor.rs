use crate::*;

use async_trait::async_trait;

#[async_trait]
pub trait Actor: Sync {
    /// Perform action on turn
    async fn act(&self, battle: &Battle) -> ActionResult;

    /// Receive damage. Override to perform reactions/dodge/etc
    fn damage(&mut self, _attacker: CharacterId, damage: Attack) {
        let character = self.get_mut_character();
        character.health -= damage;
    }

    /// Heal
    fn heal(&mut self, _healer: CharacterId, healing: Health) {
        let character = self.get_mut_character();
        character.health += healing;
    }

    /// Get the underlying `Character`
    fn get_character(&self) -> &Character;

    /// Get the underlying `Character`
    fn get_mut_character(&mut self) -> &mut Character;

    /// Called when the game is over
    async fn on_game_over(&self, battle: &Battle);
}
