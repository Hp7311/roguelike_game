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

## Stats
in `constants.rs`  
monsters in `entities::all_monsters_info`

## Architecture
1D map of `Wall`s and `Floor`s with `Player` and `Monster` struct,   
`main.rs` interacts with `State` struct.
