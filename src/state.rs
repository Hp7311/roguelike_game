/// core struct State used by main.rs

use std::{any, io::stdout};
use crossterm::{
    terminal::{
        Clear, ClearType, enable_raw_mode, disable_raw_mode
    },
    event::{KeyCode, Event, read},
    cursor::MoveTo,
    execute,
};
use log::{info, debug};

use crate::constants::{MAP_TOP_OFFSET, MAP_WIDTH};
use crate::map::Map;
use crate::logs::Logs;
use crate::entities::{
    Player, Monster, move_monsters, move_player, handle_entities, delete_dead
};
use crate::gold::render_gold;
use crate::maths::{
    Rect, Direction::*, Direction,
};


pub struct State {
    pub map: Map,
    pub logs: Logs,
    pub move_dir: Option<Direction>,
    pub player: Option<Player>,
    pub monsters: Option<Vec<Monster>>,
    rooms: Vec<Rect>,
    
    pub game_won: bool,
    pub game_lost: bool,
}


impl State {
    
    /// initalize map, logs
    pub fn init() -> Self {
        info!("Reached init()");
        Self {
            map: Map::new(),
            logs: Logs::new(),
            move_dir: None,
            player: None,
            monsters: None,
            rooms: Vec::new(),
            
            game_won: false,
            game_lost: false,
        }
    }
    
    /// digs rooms and corridors
    pub fn dig_floors(mut self) -> anyhow::Result<Self> {
        info!("Reached dig_floors()");
        self.rooms = self.map.dig_rooms()?;
        info!("Dug rooms");
        self.player = Some(Player::spawn(&self.map, &self.rooms)?);
        info!("Spawned players");
        self.monsters = Some(Monster::spawn(
            &self.map,
            &self.rooms,
            &self.player.as_ref().unwrap().get_pos()
        )?);
        info!("Spawned monsters");

        // debugging
        self.logs.add_to_log(&format!("\nPlayer at {}", self.player.as_ref().unwrap().pos));
        for monster in self.monsters.as_ref().unwrap().iter() {
            self.logs.add_to_log(&format!("Monster at {}", monster.pos));
        }
            
        Ok(self)
    }
    
    /// check if version of map doable
    pub fn validate(self) -> anyhow::Result<Self> {  // TODO actually validate
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
    pub fn get_input(&mut self) -> anyhow::Result<&mut Self> {
        //info!("Reached get_input()");

        execute!(stdout(), MoveTo(0, (MAP_TOP_OFFSET + MAP_WIDTH + 2) as u16))?;
        enable_raw_mode()?;
    
        loop {
            if let Event::Key(event) = read()? {
                match event.code {
                    KeyCode::Up    | KeyCode::Char('w') => self.move_dir = Some(Up),
                    KeyCode::Down  | KeyCode::Char('s') => self.move_dir = Some(Down),
                    KeyCode::Left  | KeyCode::Char('a') => self.move_dir = Some(Left),
                    KeyCode::Right | KeyCode::Char('d') => self.move_dir = Some(Right),
                    KeyCode::Char('r') => {
                        disable_raw_mode()?;
                        *self = Self::init()
                            .dig_floors()?
                            .validate()?;
                        self.clear_screen()?
                            .render()?; // duplicate with main
                        self.get_input()?;  // so it doesn't jump to moving entities
                    },
                    KeyCode::Esc   | KeyCode::Char('q') => {
                        disable_raw_mode()?;
                        execute!(stdout(), MoveTo(0, (MAP_TOP_OFFSET + MAP_WIDTH + 2 + 8) as u16))?;  // avoid covering logs
                        std::process::exit(1);
                    },
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
    pub fn move_entities(&mut self) -> anyhow::Result<&mut Self> {
        info!("Reached move_entities()");
        //self.player.move_to(&self);

        move_player(self)?;

        move_monsters(self)?;
        
        Ok(self)
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
    pub fn render(&mut self) -> anyhow::Result<&mut Self> {

        execute!(stdout(), MoveTo(0, 0))?;
        
        render_gold()?;
        self.map.render()?;
        self.player.as_ref().unwrap()
            .render()?;

        for monster in self.monsters.as_ref().unwrap() {
            monster.render()?;
        }
        
        self.logs.render()?;
        

        Ok(self)  // trying derefencing, says &Self not Self
    }
    
    /// performs re-initialization if lost/won
    pub fn handle_gameover(&mut self) -> anyhow::Result<()> {  // correct signature?!
        info!("Reached handle_gameover()");
        if self.game_lost || self.game_won {
            *self = Self::init()
                .dig_floors()?
                .validate()?;  // duplicates with main

            if self.game_won {
                self.logs.add_to_log("You won!");
            }
            else if self.game_lost {
                self.logs.add_to_log("You lost!");
            }
        }
        
        Ok(())
    }
}

// DONE modify State to remove the unnecessary Option in player and monsters