mod handlers;
mod server;
#[expect(clippy::module_inception)]
mod web_actor;

pub use web_actor::BattleState; // For codegen
pub use web_actor::WebActor;
