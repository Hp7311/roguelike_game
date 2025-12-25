#[derive(Debug, PartialEq, Clone)]
pub enum Tile {
    Player(i32),
    Wall,
    Monster(MonsterType),
    Floor,
}


#[derive(Debug, PartialEq, Clone, Copy)]
pub struct MonsterType {
    pub hp: i32,
    pub glyph: char,
    pub strength: u32,
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
