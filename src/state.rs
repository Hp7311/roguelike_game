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
use log::info;
use thiserror::Error;

use crate::map::Map;
use crate::logs::Logs;
use crate::entities::{
    Player, Monster, move_monsters, move_player, handle_entities, delete_dead
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

#[derive(PartialEq, Clone)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}

#[derive(Debug, Error)]
pub enum StateError {  // TODO implement StateError and use it to handle errors ( thiserror, anyhow )
    #[error("General error: {0}")]
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
        info!("Reached init()");
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
    pub fn dig_floors(mut self) -> Self {
        info!("Reached dig_floors()");
        self.map.dig_rooms();
            
        self
    }
    
    /// check if version of map doable
    pub fn validate(self) -> Result<Self, StateError> {  // TODO actually validate
        info!("Reached validate()");
        Ok(self)  // not important since rooms are hand drawn and connected
    }
    
    /// clear screen before game loop
    pub fn clear_screen(&mut self) -> std::io::Result<&mut Self> {
        info!("Reached clear_screen()");
        execute!(stdout(), Clear(ClearType::All), MoveTo(0, 0)).unwrap();

        Ok(self)  // will derefencing change self from &State to State?
    }
    
    /// modifys `move_dir` when received and clears log
    pub fn get_input(&mut self) -> std::io::Result<&mut Self> {
        //z`info!("Reached get_input()");
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
        info!("Reached move_entities()");
        //self.player.move_to(&self);

        move_player(self);
        info!("Passed move_player");

        move_monsters(self);
        
        self
    }
    
    /// handle collisions, attacks etc
    pub fn handle_entities(&mut self) -> &mut Self {
        info!("Reached handle_entities()");
        handle_entities(self);
        self
    }
    
    /// delete entities with health < 1, assign struct variants if won/lost
    pub fn delete_dead(&mut self) -> &mut Self {
        info!("Reached delete_dead()");
        delete_dead(self);
        self
    }
    
    /// renders map, log, with entities (maybe last move?)
    pub fn render(&mut self) -> std::io::Result<&mut Self> {
        execute!(stdout(), MoveTo(0, 0));
        //info!("Reached render()");
        self.map.render()?;
        self.player.render()?;

        for monster in &self.monsters {
            monster.render()?;
        }
        
        self.logs.render()?;
        
        Ok(self)  // trying derefencing, says &Self not Self
    }
    
    /// performs re-initialization if lost/won
    pub fn handle_gameover(&mut self) -> Result<(), StateError>{  // correct signature?!
        info!("Reached handle_gameover()");
        *self = Self::init()
                .dig_floors()
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