#[derive(Debug, PartialEq, Clone)]
pub enum Tile {
    Player,
    Wall,
    Monster(MonsterType),
    Floor,
}

#[derive(Debug, PartialEq, Clone)]
pub struct MonsterType {
    pub hp: i32,
    pub glyph: char,
}