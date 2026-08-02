use std::path::PathBuf;

use crate::{Battle, RandomProvider, game_file};

pub struct Game {
    pub name: String,
    pub description: String,
    pub battles: Vec<Battle>,
}

impl Game {
    pub async fn deserialize(
        data: &str,
        asset_directory: Option<PathBuf>,
        random_provider: Box<dyn RandomProvider>,
    ) -> Result<Self, String> {
        let game = game_file::Game::parse_from_str(data)?;
        let battles = deserialize_battles(&game.battles, asset_directory, random_provider).await?;
        Ok(Game {
            name: game.name,
            description: game.description,
            battles: game.battles,
        })
    }
}

fn deserialize_battles(
    battles: &[game_file::Battle],
    asset_directory: Option<PathBuf>,
    random_provider: Box<dyn RandomProvider>,
) -> Result<Vec<Battle>, String> {
    return Ok(battles.iter().map(|battle) Battle::deserialize(data, asset_directory, random_provider))
    let mut battles = Vec::new();
    for battle_file in battle_files {
        let battle = Battle::deserialize(&battle_file.data, asset_directory.clone(), random_provider.clone()).await?;
        battles.push(battle);
    }
    Ok(battles)
}