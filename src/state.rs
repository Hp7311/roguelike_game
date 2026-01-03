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
    Player, Monster, move_monsters, handle_entities
};


pub struct State {
    pub map: Map,
    pub logs: Logs,
    pub move_dir: Option<Direction>,
    pub player: Option<Player>,
    pub monsters: Option<Vec<Monster>>,
    
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
        self.map = self.map.dig_rooms()?;
            
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
    pub fn validate(self) -> Result<Self, StateError> {  // TODO actually validate
        Ok(self)  // not important since rooms are hand drawn and connected
    }
    
    /// clear screen before game loop
    pub fn clear_screen(&self) -> &Self {
        execute!(stdout(), Clear(ClearType::All), MoveTo(0, 0));
        
        self  // will derefencing change self from &State to State?
    }
    
    /// modifys `move_dir` when received
    pub fn get_input(&mut self) -> std::io::Result<&mut Self> {
        use Direction::*;
        enable_raw_mode()?;
    
        loop {
            if let Event::Key(event) = read().unwrap() {
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
    
        Ok(self)  // &mut self not supported?? trying self.clone()
    }
    
    
    /// move monsters and player
    pub fn move_entities(&mut self) -> &mut Self {
        self.player.unwrap().move_to(&self);  // not sure whether this will consume self and if it actually modifys
        
        self.monsters = Some(move_monsters(&self));
        
        self  // trying to borrow self, says &mut self not supported
    }
    
    /// handle collisions, attacks etc
    pub fn handle_entities(&mut self) -> Self {
        handle_entities(&self)
    }
    
    /// delete entities with health < 1, assign struct variants if won/lost
    pub fn delete_dead(&mut self) -> Self {
        crate::entities::delete_dead(&self)
    }
    
    /// renders map, log, with entities (maybe last move?)
    pub fn render(&self) -> &Self {
        self.map.render();
        self.player.unwrap().render();
        for monster in self.monsters.unwrap() {
            monster.render();
        }
        
        self.logs.render();
        
        self  // trying derefencing, says &Self not Self
    }
    
    /// performs re-initialization if lost/won
    pub fn handle_gameover(&mut self) {  // correct signature?!
        if self.game_won {
            *self = Self::init()  // self::init() didn't work
                .dig_floors().unwrap()  // TODO temporary
                .add_player()
                .add_monsters()
                .validate().unwrap();  // TODO temporary
            self.logs.add_to_log("You won!");
        }
        else if self.game_lost {
            *self = Self::init()
                .dig_floors().unwrap()  // TODO temporary
                .add_player()
                .add_monsters()
                .validate().unwrap();  // TODO temporary
            self.logs.add_to_log("You lost!");
        }
    }
}
