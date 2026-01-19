//! configure the stats
//! monster number, and map length/width are controlled through environmental vars
//! room length/width and number are dynamic

use std::env;
use std::sync::LazyLock;
use crate::entities::MonsterInfo;

pub const PLAYER_HEALTH: i32 = 100;
pub const PLAYER_STRENGTH: i32 = 10;

pub static MONSTER_NUMBER: LazyLock<usize> = LazyLock::new(|| {
    env::var("MONSTER_NUMBER")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or((*MAP_LENGTH * *MAP_WIDTH) / 200)  // dynamic number
});

pub const ATTACK_RANGE: u32 = 1;

pub const ROOM_NUM: LazyLock<usize> = LazyLock::new(|| (*MAP_LENGTH * *MAP_WIDTH) / 200);
pub const MAX_ROOM_LENGTH: LazyLock<usize> = LazyLock::new(|| *MAP_LENGTH / 3);
pub const MAX_ROOM_WIDTH: LazyLock<usize> = LazyLock::new(|| *MAP_WIDTH / 3);
pub const RANDOM_CORRIDOR_NUM: u32 = 4;

// gold
// health
// win/lost logs
pub const MAP_TOP_OFFSET: usize = 3;


/// IMPORTANT: MAP_WIDTH refers to the -----
/// while MAP_LENGTH refers to the "height"

pub static MAP_WIDTH: LazyLock<usize> = LazyLock::new(|| {
    env::var("MAP_WIDTH")
        .ok() 
        .and_then(|s| s.parse().ok()) 
        .unwrap_or(40)
});
pub static MAP_LENGTH: LazyLock<usize> = LazyLock::new(|| {
    env::var("MAP_LENGTH")
        .ok()
        .and_then(|s| s.parse().ok()) 
        .unwrap_or(25)
});



/// ALL MONSTERS DEFINED HERE
pub fn all_monsters_info() -> Vec<MonsterInfo> {
    vec![
        MonsterInfo::new('G', "Globin", 10, 5),
        MonsterInfo::new('D', "Dalek", 30, 25),
        MonsterInfo::new('C', "Cyberman", 20, 20),
    ]
}