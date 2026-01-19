/// contains Map
use crossterm::{
    QueueableCommand,
    cursor::{
        MoveTo, MoveToNextLine
    },
    style::{
        Print, SetForegroundColor, Color, ResetColor
    },
};
use std::io;
use std::io::Write;

mod dig_map;

use crate::{constants::{
    MAP_LENGTH, MAP_TOP_OFFSET, MAP_WIDTH
}, errors::BuildError};
use crate::maths::Rect;

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
            map: vec![Wall; *MAP_WIDTH * *MAP_LENGTH],
        }
    }
    
    pub fn dig_rooms(&mut self) -> Result<Vec<Rect>, BuildError> {
        dig_map::dig(self)
    }
    
    /// 1 space between each Tile
    pub fn render(&self) -> io::Result<()> {
        let mut stdout = io::stdout();

        stdout.queue(MoveTo(0, MAP_TOP_OFFSET.try_into().unwrap()))?;
        stdout.queue( Print( "-".repeat(*MAP_WIDTH * 2).to_string()) )?
            .queue( MoveToNextLine(1) )?;

        for (i, tile) in self.map.iter().enumerate() {
            match tile {
                Wall => {
                    stdout.queue(SetForegroundColor(Color::Grey))?
                        .queue(Print(" #"))?
                        .queue(ResetColor)?
                },
                Floor => {
                    stdout.queue(Print("  "))?
                },
            };
            if (i + 1) % *MAP_WIDTH == 0 {
                stdout.queue(MoveToNextLine(1))?;
            }
        }

        stdout.queue( Print( "-".repeat(*MAP_WIDTH * 2).to_string()) )?;
        
        stdout.flush()?;

        Ok(())
    }
}