/// core struct State used by main.rs

use std::io::stdout;
use crossterm::{
    terminal::{
        Clear, ClearType, enable_raw_mode, disable_raw_mode
    },
    event::{KeyCode,Event},
    execute,
};

use crate::map::Map;
use crate::logs::Logs;
use crate::entities::{
    Player, Monster
};


#[derive(Debug)]
pub struct State {
    map: Map,
    logs: Logs,
    move_dir: Option<Direction>,
    player: Option<Player>,
    monsters: Option<Vec<Monster>>,
    
    game_won: bool,
    game_lost: bool,
}

#[derive(PartialEq)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}


impl State {
    
    /// initalize map, logs
    pub fn init() -> Self {
        Self {
            map: Map::new(),
            logs: Logs::new(),
            move_dir: None,
            player: None,
            monsters: None,
            
            game_won: false,
            game_lost: false,
        }
    } 
    
    /// digs rooms and corridors
    pub fn dig_floors(mut self) -> Result<Self, StateError> {
        self.map = self.map.dig_rooms();  // TODO implement later
            
        Ok(self)
    }
    
    pub fn add_player(mut self) -> Self {
    
        if self.player.is_none() {
            self.player = Some(Player::spawn(&self.map));
        }
        
        self
    }
    
    pub fn add_monsters(mut self) -> Self {
        
        if self.monsters.is_none() {
            self.monsters = Some(Monster::spawn(&self.map));
        }
        
        self
    }
    
    /// check if version of map doable
    pub fn validate(self) -> Result<Self, StateError> {
        Ok(self)  // not important since rooms are hand drawn and connected
    }
    
    
    /// modifys `move_dir` when received
    pub fn get_input(&mut self) -> std::io::Result<Self> {
        use Direction::*;
        enable_raw_mode()?;
    
        loop {
            if let Event::Key(event) = read().unwrap() {
                match event.code {
                    KeyCode::Up    | KeyCode::Char('w') => self.move_dir = Up,
                    KeyCode::Down  | KeyCode::Char('s') => self.move_dir = Down,
                    KeyCode::Left  | KeyCode::Char('a') => self.move_dir = Left,
                    KeyCode::Right | KeyCode::Char('d') => self.move_dir = Right,
                    _ => continue,
                }
                break;
            }
        }
        disable_raw_mode()?;
    
        Ok(self)
    }
    
    
    /// move monsters and player
    pub fn move_entities(&mut self) -> Self {
        self.player.move_to(&self);  // not sure whether this will consume self and if it actually modifys
        
        self.monsters = entities::move_monsters(&self);
        
        self
    }
    
    /// handle collisions, attacks etc
    pub fn handle_entities(&mut self) -> Self {
        entities::handle_entities(&self);
        self
    }
    
    /// delete entities with health < 1, assign struct variants if won/lost
    pub fn delete_dead(&mut self) -> Self {
        
    }
    
    /// renders map, log, with entities (maybe last move?)
    pub fn render(&self) -> Self {
        
    }
    
    /// performs re-initialization if lost/won
    pub fn handle_gameover(&mut self) {  // correct signature?!
        
    }
}
