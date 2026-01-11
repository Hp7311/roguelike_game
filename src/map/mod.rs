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

use crate::{constants::{
    MAP_LENGTH, MAP_TOP_OFFSET, MAP_WIDTH
}, errors::BuildError};
use crate::maths::Rect;
use crate::entities::{Player, Monster};

#[derive(Clone, PartialEq, Debug)]
pub enum Tile {
    Wall,
    Floor,
}

use Tile::*;

/// main Map struct containing Wall or Floor
#[derive(Debug)]
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
    pub fn dig_rooms(&mut self) -> Result<Vec<Rect>, BuildError> {
        dig_map::dig(self)
    }
    
    /// render map
    pub fn render(&self) -> std::io::Result<()> {
        let mut stdout = std::io::stdout();

        stdout.queue(MoveTo(0, (MAP_TOP_OFFSET + 1).try_into().unwrap()))?
            .queue( Print( format!("{}", "-".repeat(MAP_LENGTH * 2))) )?
            .queue( MoveToNextLine(1) )?;

        for (i, tile) in self.map.iter().enumerate() {
            //print!("|");
            match tile {
                Wall => stdout.queue(Print(" #"))?,
                Floor => stdout.queue(Print("  "))?,
            };
            if (i + 1) % MAP_LENGTH == 0 {
                stdout.queue(MoveToNextLine(1))?;
            }
        }

        stdout.queue( Print( format!("{}", "-".repeat(MAP_LENGTH * 2))) )?;
        
        stdout.flush()?;

        Ok(())
    }
}