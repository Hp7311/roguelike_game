/// provides Player and Monster (in State)
use crate::maths::Cord;
use crate::map::{Map, Tile};
use crate::state::{State, Direction};
use crate::CONSTANTS::{
    PLAYER_HEALTH, MAP_TOP_OFFSET, MONSTER_NUMBER,
};

use rand::prelude::*;
use crossterm::{
    QueueableCommand,
    cursor::MoveTo,
    style::Print,
};
use std::io::Write;

mod move_player;
mod move_monster;
mod handle;

pub use move_player::move_player;
pub use move_monster::move_monsters;
pub use handle::handle_entities;

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
    pub fn spawn(map: &Map) -> Self {
        let mut rng = rand::rng();
        let indexes: Vec<_> = (0..map.map.len()).collect();
        
        let mut chosen_index = 0;
        
        if map.map.iter().any(|tile| *tile == Tile::Floor) {
            loop {
                chosen_index = *indexes.choose(&mut rng).unwrap();  // trying deref
                if map.map[chosen_index] == Tile::Floor {
                    break;  // if spawn on floor
                }
            }
        }
        else {  // not initialized yet
            chosen_index = *indexes.choose(&mut rng).unwrap();
        }
        
        Self {
            pos: Cord::from_1d(chosen_index),
            hp: PLAYER_HEALTH,
        }
    }
    
    
    pub fn render(&self) -> std::io::Result<()> {
        let x = MAP_TOP_OFFSET + self.pos.x * 2;
        let y = self.pos.y * 4 - 1;
        let mut stdout = std::io::stdout();

        stdout.queue(MoveTo(x.try_into().unwrap(), y.try_into().unwrap()))?
            .queue(Print("@"))?;
        stdout.flush()?;

        Ok(())
    }
}


impl Monster {
    pub fn spawn(map: &Map) -> Vec<Self> {
        let mut rng = rand::rng();
        let indexes: Vec<_> = (0..map.map.len()).collect();
        let mut chosen_index = 0;
        
        let mut monsters = Vec::new();
        
        for _ in 0..MONSTER_NUMBER {  // atomatically generate appriopriate num in fuuture
            
            if map.map.iter().any(|tile| *tile == Tile::Floor) {
                loop {
                    chosen_index = *indexes.choose(&mut rng).unwrap();  // deref for &Type and Type issue
                    if map.map[chosen_index] == Tile::Floor {
                        break;
                    }
                }
            }
            else {  // not dug floors yet
                chosen_index = *indexes.choose(&mut rng).unwrap();
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
        let x = MAP_TOP_OFFSET + self.pos.x * 2;
        let y = self.pos.y * 4 - 1;
        
        let mut stdout = std::io::stdout();
        stdout.queue(MoveTo(x.try_into().unwrap(), y.try_into().unwrap()))?
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
        MonsterInfo::new('D', "Dragon", 20, 20),
    ]
}


/// delete dead units
pub fn delete_dead(state: &mut State) {
    
    if state.player.hp <= 0 {
        state.game_lost = true;
        return;
    }
    
    state.monsters
        .retain(|monster| monster.info.hp > 0);  // filter out dead monsters
    
    if state.monsters.len() == 0 {
        state.game_won = true;
    };
}