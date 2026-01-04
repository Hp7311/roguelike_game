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

use move_player::move_player;
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
        
        loop {
            chosen_index = *indexes.choose(&mut rng).unwrap();  // trying deref
            if map.map[chosen_index] == Tile::Floor {
                break;  // if spawn on floor
            }
        }
        
        Self {
            pos: Cord::from_1d(chosen_index),
            hp: PLAYER_HEALTH,
        }
    }
    
    pub fn move_to(&mut self, state: &State) {
        self.pos = move_player(self.pos, state);
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
            
            loop {
                chosen_index = *indexes.choose(&mut rng).unwrap();  // deref for &Type and Type issue
                if map.map[chosen_index] == Tile::Floor {
                    break;
                }
            }
            
            monsters.push( Self {
                pos: Cord::from_1d(chosen_index),
                info: get_rand_monster(),
            });
            
        }
        
        monsters
    }
    
    pub fn move_to(&mut self, state: &State) -> Vec<Self> {
        move_monsters(state)
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
    
    *all_monsters_info().choose(&mut rng).unwrap()
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
pub fn delete_dead(state: &State) -> State {
    let mut ret = state;
    
    if ret.player.hp <= 0 {
        ret.game_lost = true;
        return *ret;  // deref trying to convert &State to State
    }
    
    ret.monsters = Some(state.monsters.iter()
        .filter(|&monster| monster.info.hp > 0)
        .map(|&monster| monster)
        .collect::<Vec<_>>()
    );
    
    if ret.monsters.len() == 0 {
        ret.game_won = true;
    }
    
    *ret  // deref trying to convert &State to State
}