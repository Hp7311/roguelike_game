/// contains Map
use crossterm::{
    execute,
    cursor::MoveTo,
};
mod dig_map;

use crate::CONSTANTS::{MAP_WIDTH, MAP_HEIGHT, CURSOR_DRAW_MAP};
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
    
    /// render map
    pub fn render(&self) {
        execute!(stdout(), MoveTo(CURSOR_DRAW_MAP, 0));
        
        println!("-".repeat(MAP_LENGTH * 4 + 1));
        
        for (i, tile) in self.map.iter().enumerate() {
            print!("|")
            if (i + 1) % MAP_LENGTH == 0 {
                match tile {
                    Wall => println!(" # |"),
                    Floor => println!("   |"),
                }
            }
            else {
                match tile {
                    Wall => print!(" # "),
                    Floor => print!("   "),
                }
            }
        }
        
        println!("-".repeat(MAP_LENGTH * 4 + 1));
    }
}