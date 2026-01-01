/// core struct State used by main.rs

use std::io::stdout;
use crossterm::{
    terminal::{
        Clear, ClearType, enable_raw_mode, disable_raw_mode
    },
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
    input: Option<char>,
    player: Option<Player>,
    monsters: Option<Vec<Monster>>,
    
    game_won: bool,
    game_lost: bool,
}


impl State {
    
    /// initalize map, logs
    pub fn init() -> Self {
        Self {
            map: Map::new(),
            logs: Logs::new(),
            input: None,
            player: None,
            monsters: None,
            
            game_won: false,
            game_lost: false,
        }
    } 
    
    /// digs rooms and corridors
    pub fn dig_floors(mut self) -> Result<Self, StateError> {
        self.map = Map.dig_rooms()?  // implement later
            .dig_corridors()?;
            
        Ok(self)
    }
    
    pub fn add_player(mut self) -> Self {
    
        if self.player.is_none() {
            self.player = Some(Player::spawn(&self.map));
        }
        
        self
    }
    
    pub fn add_monsters(mut self) -> Result<Self, StateError> {
        
        if self.monsters.is_none() {
            self.monsters = Some(Monster::spawn(&self.map))?;
        }
        
        Ok(self)
    }
    
    /// check if version of map doable
    pub fn validate(&self) -> Result<Self, StateError> {
        
    }
    
    /// modifys `input` when received
    pub fn get_input(mut self) -> std::io::Result<Self> {
        enable_raw_mode()?;
    
        loop {
            if let Event::Key(event) = read().unwrap() {
                match event.code {
                    KeyCode::Up => {
                        self.input = 'w';
                        break;
                    },
                    KeyCode::Down => {
                        self.input = 's';
                        break;
                    },
                    KeyCode::Left => {
                        self.input = 'a';
                        break;
                    },
                    KeyCode::Right => {
                        self.input = 'd';
                        break;
                    },
                    KeyCode::Char(c) if matches!(c, 'w' | 's' | 'a' | 'd' | 'q' | 'r') => {
                        self.input = c;
                        break;
                    },
                    _ => {},
                }
            }
        }
        disable_raw_mode()?;
    
        self
    }
    
    
    /// move monsters and player
    pub fn move_entities(mut self) -> Self {
        
    }
    
    /// handle collisions, attacks etc
    pub fn handle_entities(mut self) -> Self {
        
    }
    
    /// delete entities with health < 1, assign struct variants if won/lost
    pub fn delete_dead(mut self) -> Self {
        
    }
    
    /// renders map, log, with entities (maybe last move?)
    pub fn render(&self) -> Self {
        
    }
    
    /// performs re-initialization if lost/won
    pub fn handle_gameover(&mut self) {  // correct signature?!
        
    }
}
