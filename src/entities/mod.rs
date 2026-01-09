/// provides Player and Monster (in State)
use crate::maths::{Cord, Rect, check_cord_in_any_room};
use crate::map::{Map, Tile};
use crate::state::{State, Direction};
use crate::constants::{
    MAP_TOP_OFFSET, MAP_WIDTH, MONSTER_NUMBER, PLAYER_HEALTH
};

use rand::prelude::*;
use crossterm::{
    QueueableCommand,
    cursor::MoveTo,
    style::Print,
};
use std::io::Write;
use log::info;

mod move_player;
mod move_monster;
mod handle;

pub use move_player::move_player;
pub use move_monster::move_monsters;
pub use handle::handle_entities;
pub use move_monster::get_nswe;

#[derive(Clone)]
pub struct Player {
    pos: Cord,
    hp: i32,
}


pub struct Monster {
    pos: Cord,
    info: MonsterInfo,
}

#[derive(Clone)]
struct MonsterInfo {
    glyph: char,
    name: String,
    hp: i32,
    strength: u32,
}

impl Player {

    pub fn get_pos(&self) -> Cord {
        self.pos.clone()
    }

    pub fn spawn(map: &Map, rooms: &Vec<Rect>) -> Self {
        let mut rng = rand::rng();
        let indexes: Vec<_> = (0..map.map.len()).collect();
        
        let mut chosen_index = 0;
        
        if map.map.iter().any(|tile| *tile == Tile::Floor) {
            loop {
                chosen_index = *indexes.choose(&mut rng).unwrap();  // trying deref
                if map.map[chosen_index] == Tile::Floor && check_cord_in_any_room(&rooms, Cord::from_1d(chosen_index)){
                    break;  // if spawn on floor
                }
            }
        }
        else {  // not initialized yet
            panic!("Map has no floor");
        }
        
        Self {
            pos: Cord::from_1d(chosen_index),
            hp: PLAYER_HEALTH,
        }
    }
    
    
    pub fn render(&self) -> std::io::Result<()> {
        let x = MAP_TOP_OFFSET + self.pos.x + 2;
        let y = (self.pos.y + 1) * 2 - 1;
        let mut stdout = std::io::stdout();

        stdout.queue(MoveTo(y.try_into().unwrap(), x.try_into().unwrap()))?
            .queue(Print("@"))?
            .queue(MoveTo(0, (MAP_TOP_OFFSET + MAP_WIDTH + 2) as u16))?;
        stdout.flush()?;
        
        info!("Player at {}", self.pos);

        Ok(())
    }
}


impl Monster {
    pub fn spawn(map: &Map, rooms: &Vec<Rect>, player: &Cord) -> Vec<Self> {
        let mut rng = rand::rng();
        let indexes: Vec<_> = (0..map.map.len()).collect();
        let mut chosen_index = 0;
        
        let mut monsters = Vec::new();
        
        for _ in 0..MONSTER_NUMBER {  // atomatically generate appriopriate num in fuuture
            
            if map.map.iter().any(|tile| *tile == Tile::Floor) {

                loop {
                    chosen_index = *indexes.choose(&mut rng).unwrap();

                    // validate chosen index
                    if map.map[chosen_index] == Tile::Floor 
                        && check_cord_in_any_room(rooms, Cord::from_1d(chosen_index))
                        && (player.get_1d() != chosen_index) {  // does not spawn on player

                        break;
                    }
                }
            }
            else {  // not dug floors yet
                panic!("No floors in map");
            }
            
            monsters.push( Self {
                pos: Cord::from_1d(chosen_index),
                info: get_rand_monster(),
            });
            
        }
        
        monsters
    }
    
    /// prints a single monster
    pub fn render(&self) -> std::io::Result<()> {
        let x = MAP_TOP_OFFSET + self.pos.x + 2;
        let y = (self.pos.y + 1) * 2 - 1;
        
        let mut stdout = std::io::stdout();
        stdout.queue(MoveTo(y.try_into().unwrap(), x.try_into().unwrap()))?
            .queue(Print(format!("{}", self.info.glyph)))?;
        
        stdout.flush()?;
        Ok(())
    }
}

impl MonsterInfo {

    /// new MonsterInfo for convienence in all_monsters_info()
    fn new(glyph: char, name: &str, hp: i32, strength: u32) -> Self {
        Self {
            glyph,
            name: name.to_string(),
            hp,
            strength,
        }
    }
}

/// get a random Monster
fn get_rand_monster() -> MonsterInfo {  // may say expected &--- found ---
    let mut rng = rand::rng();
    
    all_monsters_info().choose(&mut rng).unwrap().clone()
}


/// ALL MONSTERS DEFINED HERE
fn all_monsters_info() -> Vec<MonsterInfo> {
    vec![
        MonsterInfo::new('G', "Globin", 10, 5),
        MonsterInfo::new('D', "Dalek", 30, 25),
        MonsterInfo::new('C', "Cyberman", 20, 20),
    ]
}


/// delete dead units
pub fn delete_dead(state: &mut State) {
    
    if state.player.as_ref().unwrap().hp <= 0 {
        state.game_lost = true;
        return;
    }
    
    state.monsters.as_mut().unwrap()
        .retain(|monster| monster.info.hp > 0);  // filter out dead monsters
    
    if state.monsters.as_ref().unwrap().len() == 0 {
        state.game_won = true;
    };
}