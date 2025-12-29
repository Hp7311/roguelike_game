## Status
In development.

## Description
Roguelike game in ASCII.

## Usage
```bash
git clone https://github.com/Hp7311/roguelike_game.git
cd roguelike_game
cargo run
```

## Adjusting stats
map's width and length: `constants.rs`.  
player's health: `constants.rs`.  
number of monsters: `constants.rs`.  
vector of monsters to choose from: `entities::mod.rs` in `get_monsters`.
