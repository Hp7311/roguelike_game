/// provides Logs struct for logging actions in game
use crossterm::{
    QueueableCommand, cursor, style::Print
};
use std::io::Write;

use crate::constants::{MAP_WIDTH, MAP_TOP_OFFSET};

pub struct Logs {
    msg: String,
}

impl Logs {
    pub fn new() -> Self {
        Self {
            msg: String::new(),
        }
    }
    
    pub fn add_to_log(&mut self, msg: &str) {
        self.msg.push_str(msg);
        self.msg.push_str("\n");
    }
    
    pub fn render(&self) -> std::io::Result<()> {
        
        let mut stdout = std::io::stdout();
        stdout.queue(cursor::MoveTo(
                0, (MAP_TOP_OFFSET + MAP_WIDTH + 2 + 4) as u16  // + 4 to avoid log messages in debug mode
            ))?
            .queue(Print( format!("{}", self.msg) ))?;

        stdout.flush()?;
        Ok(())
    }

    pub fn clear(&mut self) {
        self.msg = String::new();
    }
}