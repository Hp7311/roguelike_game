/// provides Player and Monster (in State)
use crate::maths::Cord;
use crate::map::{Map, Tile};
use rand::prelude::*;

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
}

impl Player {
    pub fn spawn(map: &Map) -> Self {
        let mut rng = rand::rng();
        let indexes: Vec<_> = (0..map.map.len()).collect();
        
        let mut chosen_index = 0;
        
        loop {
            chosen_index = indexes.choose(&mut rng)
            if map.map[chosen_index] == Tile::Floor {
                break;  // if spawn on floor
            }
        }
        
        Self {
            pos: Cord::from_1d(chosen_index),
            hp: CONSTANTS::PLAYER_HEALTH,
        }
    }
}

impl Monster {
    pub fn spawn(map: &Map) -> Vec<Self> {
        let mut rng = rand::rng();
        let indexes: Vec<_> = (0..map.map.len()).collect();
        let mut chosen_index = 0;
        
        let mut monsters = Vec::new();
        
        for _ in 0..CONSTANTS::MONSTER_NUMBER {  // atomatically generate appriopriate num in fuuture
            
            loop {
                chosen_index = indexes.choose(&mut rng);
                if map.map[chosen_index] == Tile::Floor {
                    break;
                }
            }
            
            monsters.push( Self {
                pos: Cord::from_1d(&chosen_index),
                info: get_rand_monster(),
            });
            
        }
        
        monsters
    }
}

impl MonsterInfo {

    /// new MonsterInfo for convienence in all_monsters_info()
    fn new(glyph: char, name: String, hp: i32) -> Self {
        Self {
            glyph,
            name,
            hp,
        }
    }
}

/// get a random Monster
fn get_rand_monster() -> Monster {
    let mut rng = rand::rng();
    
    all_monsters_info.choose(&mut rng)
}

/// ALL MONSTERS DEFINED HERE
fn all_monsters_info() -> Vec<MonsterInfo> {
    vec![
        MonsterInfo::new('G', "Globin", 10),
        MonsterInfo::new('D', "Dalek", 30),
        MonsterInfo::new('D', "Dragon", 20),
    ]
}
