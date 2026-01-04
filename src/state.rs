/// core struct State used by main.rs

use std::io::stdout;
use crossterm::{
    terminal::{
        Clear, ClearType, enable_raw_mode, disable_raw_mode
    },
    event::{KeyCode, Event, read},
    cursor::MoveTo,
    execute,
};

use crate::map::Map;
use crate::logs::Logs;
use crate::entities::{
    Player, Monster, move_monsters, handle_entities, delete_dead
};


pub struct State {
    pub map: Map,
    pub logs: Logs,
    pub move_dir: Option<Direction>,
    pub player: Player,
    pub monsters: Vec<Monster>,
    
    pub game_won: bool,
    pub game_lost: bool,
}

#[derive(PartialEq)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}

#[derive(Debug)]
pub enum StateError {  // TODO implement StateError and use it to handle errors ( thiserror, anyhow )
    General(String),
}

impl From<std::io::Error> for StateError {
    fn from(v: std::io::Error) -> Self {
        StateError::General("I/O error".to_string())
    }
}

impl State {
    
    /// initalize map, logs
    pub fn init() -> Self {
        Self {
            map: Map::new(),
            logs: Logs::new(),
            move_dir: None,
            player: Player::spawn(&Map::new()),
            monsters: Monster::spawn(&Map::new()),
            
            game_won: false,
            game_lost: false,
        }
    } 
    
    /// digs rooms and corridors
    pub fn dig_floors(mut self) -> Result<Self, StateError> {
        self.map.dig_rooms()?;
            
        Ok(self)
    }
    
    /// check if version of map doable
    pub fn validate(self) -> Result<Self, StateError> {  // TODO actually validate
        Ok(self)  // not important since rooms are hand drawn and connected
    }
    
    /// clear screen before game loop
    pub fn clear_screen(&self) -> &Self {
        execute!(stdout(), Clear(ClearType::All), MoveTo(0, 0));
        
        self  // will derefencing change self from &State to State?
    }
    
    /// modifys `move_dir` when received and clears log
    pub fn get_input(&mut self) -> std::io::Result<&mut Self> {
        use Direction::*;
        enable_raw_mode()?;
    
        loop {
            if let Event::Key(event) = read()? {
                match event.code {
                    KeyCode::Up    | KeyCode::Char('w') => self.move_dir = Some(Up),
                    KeyCode::Down  | KeyCode::Char('s') => self.move_dir = Some(Down),
                    KeyCode::Left  | KeyCode::Char('a') => self.move_dir = Some(Left),
                    KeyCode::Right | KeyCode::Char('d') => self.move_dir = Some(Right),
                    _ => continue,
                }
                break;
            }
        }
        disable_raw_mode()?;

        self.logs.clear();
    
        Ok(self)  // &mut self not supported?? trying self.clone()
    }
    
    
    /// move monsters and player
    pub fn move_entities(&mut self) -> &mut Self {
        self.player.move_to(&self);
        
        self.monsters = move_monsters(&self);
        
        self
    }
    
    /// handle collisions, attacks etc
    pub fn handle_entities(&mut self) -> Self {
        handle_entities(&self)
    }
    
    /// delete entities with health < 1, assign struct variants if won/lost
    pub fn delete_dead(&mut self) -> Self {
        delete_dead(&self)
    }
    
    /// renders map, log, with entities (maybe last move?)
    pub fn render(&self) -> &Self {
        self.map.render();
        self.player.render();

        for monster in self.monsters {
            monster.render();
        }
        
        self.logs.render();
        
        self  // trying derefencing, says &Self not Self
    }
    
    /// performs re-initialization if lost/won
    pub fn handle_gameover(&mut self) -> Result<(), StateError>{  // correct signature?!
        *self = Self::init()
                .dig_floors()?
                .validate()?;

        if self.game_won {
            self.logs.add_to_log("You won!");
        }
        else if self.game_lost {
            self.logs.add_to_log("You lost!");
        }
        
        Ok(())
    }
}

// DONE modify State to remove the unnecessary Option in player and monsters