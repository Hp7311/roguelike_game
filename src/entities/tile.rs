use crate::utils::Cord;

/// Tile trait, tile structs implement it
pub trait Tile {
    fn passable(&self) -> bool;
    fn was_seen(&self) -> bool;
    fn get_pos(&self) -> Cord;
}


/// Monster tile
#[derive(PartialEq, Clone)]
pub struct Monster {
    hp:            i32,
    pos:           Cord,
    glyph:         char,
    strength:      u32,
    health:        i32,
    name:          String,
    player_damage: u32,
    gold_provided: u32,
}


/// Player tile
#[derive(PartialEq, Clone)]
pub struct Player {
    hp:     i32,
    pos:    Cord,
    health: i32,
}


/// Wall struct
#[derive(PartialEq, Clone)]
pub struct Wall {
    pos: Cord,
}


/// Floor struct
#[derive(PartialEq, Clone)]
pub struct Floor {
    pos: Cord,
}


