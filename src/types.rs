use std::convert::Infallible;

use revm::db::{CacheDB, EmptyDBTyped};
use serde::{Deserialize, Serialize};

use crate::game::Game;

pub trait Playable {
    fn play(&self, game: &mut Game);
}

pub type Evm = revm::Evm<'static, (), CacheDB<EmptyDBTyped<Infallible>>>;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Strategy {
    Regular,
    Whale,
    Degen,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Strategies(pub Vec<(Strategy, u8)>);

#[derive(Debug, Serialize, Deserialize)]
pub struct Players(Vec<Player>);

#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    name: String,
    image: String,
    lore: String,
    stats: PlayerStats,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerStats {
    balance: u64,
    #[serde(rename = "playstyle")]
    play_style: String,
}
