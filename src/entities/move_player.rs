/// functions to do with moving player
use crate::map::{Map, Tile};
use crate::maths::Cord;
use crate::state::State;
use crate::CONSTANTS::{MAP_WIDTH, MAP_LENGTH};
use crate::state::Direction::*;

pub fn move_player(state: &mut State)  {
    let direction = state.move_dir.clone().expect("No moving direction found in State");
    let (target_x, target_y) = (state.player.pos.x as isize, state.player.pos.y as isize);
    let moved_cords;
    
    match direction {
        Up => moved_cords = (target_x - 1, target_y),
        Down => moved_cords = (target_x + 1, target_y),
        Left => moved_cords = (target_x, target_y - 1),
        Right => moved_cords = (target_x, target_y + 1),
    }
    
    if moved_cords.0 < 0 || moved_cords.1 < 0 {
        return ();
    }

    // convert to usize Cord after checking
    let moved_cords = Cord::new(moved_cords.0 as usize, moved_cords.1 as usize);
    
    if moved_cords.x >= MAP_WIDTH || moved_cords.y >= MAP_LENGTH {
        return ();
    }
    let move_on_monster = state.monsters
        .iter()
        .any(|m| m.pos == moved_cords);
    if move_on_monster {
        return ();
    }
    
    state.player.pos = moved_cords;
}
