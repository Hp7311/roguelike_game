//! provides Logs struct for logging actions in game
use crossterm::{
    QueueableCommand,
    cursor::{MoveToNextLine, MoveTo},
    style::{
        Print, SetForegroundColor, Color, ResetColor
    },
};
use std::io::Write;

use crate::constants::{MAP_LENGTH, MAP_TOP_OFFSET};

#[derive(Debug)]
pub struct Logs {
    msg: String,
    lost: bool,
    won: bool,
}

impl Logs {
    pub fn new() -> Self {
        Self {
            msg: String::new(),
            lost: false,
            won: false,
        }
    }
    
    pub fn add_to_log(&mut self, msg: &str) {
        self.msg.push_str(msg);
        self.msg.push('\n');
    }
    
    pub fn render(&self) -> std::io::Result<()> {
        
        let mut stdout = std::io::stdout();
        // normal logs
        stdout.queue(MoveTo(
                0, (MAP_TOP_OFFSET + *MAP_LENGTH + 2 + 4) as u16  // + 4 to avoid log messages in debug mode
            ))?
            .queue(Print( self.msg.to_string() ))?
            .queue(MoveToNextLine(1))?;
        
        stdout.queue(MoveTo(0, 2))?;

        // won/lost logs
        if self.lost {
            stdout.queue(SetForegroundColor(Color::Red))?
                .queue(Print("You lost!!"))?;
        } else if self.won {
            stdout.queue(SetForegroundColor(Color::Cyan))?
                .queue(Print("You won!!"))?;
        }

        stdout.queue(ResetColor)?;
        stdout.flush()?;
        Ok(())
    }

    /// clear all fields every move
    pub fn clear(&mut self) {
        *self = Self::new();
    }

    pub fn won(&mut self) {
        self.won = true;
    }

    pub fn lost(&mut self) {
        self.lost = true;
    }
}