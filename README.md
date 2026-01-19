## Description
Roguelike game in ASCII.

## Usage
```bash
git clone https://github.com/Hp7311/roguelike_game.git
cd roguelike_game
cargo run
```  
Move by WSAD or arrow keys. q to quit.

## Stats
in `constants.rs`  
Map data and Monster number can be controlled through environmental variables.  

## Features
Dynamically rendered room stats and monster number  
Customize monster names and stats  
Gold system  

## Architecture
1D map of `Tile` with `Player` and `Monster` struct,   
`main.rs` interacts with `State` struct.  

Built with Rust :)
