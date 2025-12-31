// NOTE map inside module due to various methods
mod map;

// access in State through map::Map instead of map::map::Map
pub use map::Map;