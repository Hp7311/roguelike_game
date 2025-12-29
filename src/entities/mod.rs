mod check_map_valid;
mod moves_monsters;

pub use crate::entities::moves_monsters::moves_monsters;
pub use crate::entities::check_map_valid::check_map_valid;

use crate::map::Map;
use std::fs;

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

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
    pub player_strength_to: u32,
    pub gold: u32,
}

pub enum MoveReturn {
    Success(Map),
    Failure,
}

pub fn in_range(source: (usize, usize), target: (usize, usize), range: usize) -> bool {
    let (source_x, source_y) = source;
    let (target_x, target_y) = target;
    
    source_x.abs_diff(target_x) < range && source_y.abs_diff(target_y) < range
}


pub fn get_monsters() -> Vec<MonsterType> {
    vec![
        MonsterType { player_strength_to: 5,  hp: 10, gold: 2, glyph: 'G', strength: 20, name: "Goblin".to_string()},
        MonsterType { player_strength_to: 15, hp: 20, gold: 4, glyph: 'O', strength: 10, name: "Orc".to_string() },
        MonsterType { player_strength_to: 5,  hp: 15, gold: 3, glyph: 'E', strength: 5,  name: "Elf".to_string() },
        MonsterType { player_strength_to: 25, hp: 50, gold: 10, glyph: 'D', strength: 50, name: "Dalek".to_string() },
    ]
}


fn write_to_gold_file(amount: u32) -> std::io::Result<()> {
    fs::write("gold.txt", amount.to_string())?;
    Ok(())
}


fn get_gold_amount() -> std::io::Result<u32> {
    let gold_file = fs::read_to_string("gold.txt")?;
    
    let gold_amount: u32 = gold_file
        .trim()
        .parse()
        .expect("error converting file string to integer");
    
    Ok(gold_amount)
}

pub fn add_to_gold(amount: u32) -> std::io::Result<()> {
    let gold = get_gold();
    let total = gold + amount;
    println!("New gold amount: {}", total);
    write_to_gold_file(total)?;
    Ok(())
}

pub fn get_gold() -> u32 {
    match get_gold_amount() {
        Ok(gold) => gold,
        Err(_) => {
            fs::write("gold.txt", "0").unwrap();
            0
        },
    }
}