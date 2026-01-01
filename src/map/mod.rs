/// contains Map
mod dig_map;

use crate::CONSTANTS::{MAP_WIDTH, MAP_HEIGHT};
use crate::entities::{Player, Monster};

#[derive(PartialEq)]
pub enum Tile {
    Wall,
    Floor,
}

use Tile::*;

/// main Map struct containing Wall or Floor
pub struct Map {
    map: Vec<Tile>,
}

enum BuildError {}

impl Map {
    
    pub fn new() -> Self {
        Self {
            map: vec![Wall; MAP_WIDTH * MAP_LENGTH],
        }
    }
    
    /// dig rooms
    pub fn dig_rooms(mut self) -> Result<Self, BuildError> {
        dig_map::dig(self)?  // from original map -> self should be all Wall
    }
    
    /*/// checks if player can reach every monster
    pub fn can_reach(&self, player: Player, monster: Vec<Monster>) -> Result<Self, StateError> {
        
    }*/
}