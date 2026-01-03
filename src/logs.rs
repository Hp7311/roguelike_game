/// provides Logs struct for logging actions in game
use crossterm::{
    QueueableCommand, cursor, style::Print
};
use std::io::Write;

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
    
    pub fn render(&self) {
        
        let mut stdout = std::io::stdout();

        stdout.queue(Print("{}", self.msg));

        stdout.flush();
    }
}