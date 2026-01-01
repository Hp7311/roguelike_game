/// entry point

mod state;
mod entities;
mod map;
mod logs;

mod maths;
mod gold;
mod CONSTANTS;

use state::{State, StateError};


fn main() -> Result<(), StateError> {
    
    let mut gs = State::init()
        .dig_floors()?
        .add_player()
        .add_monsters()
        .validate()?;

        
    // turn-based game loop
    loop {
        gs.get_input()?
            .move_entities()
            .handle_entities()
            .delete_dead()
            .render()
            .handle_gameover();
    }
    
    
    Ok(())
}

// SOLVED disable dealing damage to monsters when -> Wall
// SOLVED add monsters AI
// SOLVED add monster name to log messages
// SOLVED player has bonuses aganst certain monsters
// SOLVED decide whether a version of map can be completed

// SOLVED gold system
// SOLVED monster moves toward player
// TODO monsters have different speed
// SOLVED arrow keys to move
// DOING architecture improvement
// TODO health bar, level system, things to do with gold etc.
// TODO player refills HP
// TODO increasing difficulty of levels
// TODO dynamic amount of monsters according to map size
// TODO FOV for player
// TODO better UI
// TODO more diversity of tiles. Bonus tiles for HP, etc.
// ALWAYS improve reusability
// for now, all monsters move towards player
