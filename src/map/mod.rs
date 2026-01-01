/// contains Map
use CONSTANTS::{MAP_WIDTH, MAP_HEIGHT}

#[derive(PartialEq)]
enum Tile {
    Wall,
    Floor,
}

use Tile::*;

/// main Map struct containing Wall or Floor
pub struct Map {
    map: Vec<Tile>,
}


impl Map {
    
    fn new() -> Self {
        Self {
            map: vec![Wall; MAP_WIDTH * MAP_HEIGHT],
        }
    }
}