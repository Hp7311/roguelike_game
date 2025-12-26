mod check_map_valid;

pub use crate::entities::check_map_valid::check_map_valid;
use crate::map::Map;

#[derive(Debug, PartialEq, Clone)]
pub enum Tile {
    Player(i32),
    Wall,
    Monster(MonsterType),
    Floor,
}


#[derive(Debug, PartialEq, Clone)]
pub struct MonsterType {
    pub hp: i32,
    pub glyph: char,
    pub strength: u32,
    pub name: String,
}

pub enum MoveReturn {
    Success(Map),
    Failure,
}

pub fn in_range(source: (usize, usize), target: (usize, usize)) -> bool {
    let (source_x, source_y) = source;
    let (target_x, target_y) = target;
    
    if source_x.abs_diff(target_x) < 3 &&
        source_y.abs_diff(target_y) < 3 
    {
        true
    } else {
        false
    }
}


pub fn get_monsters() -> Vec<MonsterType> {
    vec![
        MonsterType { hp: 10, glyph: 'G', strength: 20, name: "Goblin".to_string() },
        MonsterType { hp: 20, glyph: 'O', strength: 10, name: "Orc".to_string() },
        MonsterType { hp: 15, glyph: 'E', strength: 5, name: "Elf".to_string() },
        MonsterType { hp: 50, glyph: 'D', strength: 50, name: "Dalek".to_string() },
    ]
}