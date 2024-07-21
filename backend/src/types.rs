use std::{convert::Infallible, fmt::Display};

use revm::db::{CacheDB, EmptyDBTyped};
use serde::{Deserialize, Serialize};

use crate::game::Game;

pub trait Playable {
    fn play(&self, game: &mut Game);
}

pub type Evm = revm::Evm<'static, (), CacheDB<EmptyDBTyped<Infallible>>>;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Strategy {
    Analyst,
    Whale,
    Degen,
}

impl Display for Strategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Strategy::Analyst => write!(f, "Analyst"),
            Strategy::Degen => write!(f, "Degen"),
            Strategy::Whale => write!(f, "Whale"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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
