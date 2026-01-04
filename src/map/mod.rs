/// contains Map
use crossterm::{
    QueueableCommand,
    cursor::{
        MoveTo, MoveToNextLine
    },
    style::Print,
};
use std::io::Write;

mod dig_map;

use crate::CONSTANTS::{
    MAP_WIDTH, MAP_LENGTH, MAP_TOP_OFFSET
};
use crate::entities::{Player, Monster};
use crate::state::StateError;  // TODO temporary

#[derive(Clone, PartialEq)]
pub enum Tile {
    Wall,
    Floor,
}

use Tile::*;

/// main Map struct containing Wall or Floor
pub struct Map {
    pub map: Vec<Tile>,
}


impl Map {
    
    pub fn new() -> Self {
        Self {
            map: vec![Wall; MAP_WIDTH * MAP_LENGTH],
        }
    }
    
    /// dig rooms
    pub fn dig_rooms(&mut self) -> Result<(), StateError> {
        self.map = dig_map::dig(self)?  // from original map -> self should be all Wall
    }
    
    /// render map
    pub fn render(&self) -> std::io::Result<()> {
        let mut stdout = std::io::stdout();

        stdout.queue(MoveTo((MAP_TOP_OFFSET + 1).try_into().unwrap(), 0))?
            .queue( Print( format!("{}", "-".repeat(MAP_LENGTH * 4 + 1))) )?;

        for (i, tile) in self.map.iter().enumerate() {
            print!("|");
            match tile {
                Wall => stdout.queue(Print(" # |"))?,
                Floor => stdout.queue(Print("   |"))?,
            };
            if (i + 1) % MAP_LENGTH == 0 {
                stdout.queue(MoveToNextLine(1))?;
            }
        }

        stdout.queue( Print( format!("{}", "-".repeat(MAP_LENGTH * 4 + 1))) )?;
        
        stdout.flush()?;
        Ok(())
    }
}