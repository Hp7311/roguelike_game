/// entry point

mod state;
mod entities;
mod utils;
mod constants;

use state::State;


fn main() -> Result<(), std::io::Error> {
    
    State::init()
        .run()?;
    
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
