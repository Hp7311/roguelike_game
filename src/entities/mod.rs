mod check_map_valid;
mod moves_monsters;
pub mod tile;

pub use crate::entities::moves_monsters::moves_monsters;
pub use crate::entities::check_map_valid::check_map_valid;

use crate::constants;


pub fn in_range(source: (usize, usize), target: (usize, usize), range: usize) -> bool {
    let range = range + 1;
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



pub fn print_hp(hp: i32) {
    let divide = (hp as f32) / (constants::PLAYER_HEALTH as f32);
    
    match divide {
        0.0        => println!("{}", get_health_bar_func(0)),
        0.0..0.1   => println!("{}", get_health_bar_func(1)),
        0.1..0.2   => println!("{}", get_health_bar_func(2)),
        0.2..0.3   => println!("{}", get_health_bar_func(3)),
        0.3..0.4   => println!("{}", get_health_bar_func(4)),
        0.4..0.5   => println!("{}", get_health_bar_func(5)),
        0.5..0.6   => println!("{}", get_health_bar_func(6)),
        0.6..0.7   => println!("{}", get_health_bar_func(7)),
        0.7..0.8   => println!("{}", get_health_bar_func(8)),
        0.8..0.9   => println!("{}", get_health_bar_func(9)),
        0.9..=1.0  => println!("{}", get_health_bar_func(10)),
        _          => panic!("Player health more than maxmum health"),
    }
    // IDEA do divide / 0.1 + 1 ignore decimal and pass to func
}


fn get_health_bar_func(remain: usize) -> String {
    format!("Health: [{}{}]", "#".repeat(remain), "-".repeat(10 - remain))
}