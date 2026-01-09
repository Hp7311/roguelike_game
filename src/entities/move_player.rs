/// functions to do with moving player
use crate::map::{Map, Tile};
use crate::maths::Cord;
use crate::state::State;
use crate::constants::{MAP_WIDTH, MAP_LENGTH};
use crate::state::Direction::*;

pub fn move_player(state: &mut State)  {
    let direction = state.move_dir.clone().expect("No moving direction found in State");
    let (target_x, target_y) = (
        state.player.as_ref().unwrap().pos.x as isize,
        state.player.as_ref().unwrap().pos.y as isize
    );
    let moved_cords;
    
    match direction {
        Up => moved_cords = (target_x - 1, target_y),
        Down => moved_cords = (target_x + 1, target_y),
        Left => moved_cords = (target_x, target_y - 1),
        Right => moved_cords = (target_x, target_y + 1),
    }
    
    if moved_cords.0 < 0 || moved_cords.1 < 0 {  // moving out of bound
        return;
    }

    // convert to usize Cord after checking
    let moved_cords = Cord::new(moved_cords.0 as usize, moved_cords.1 as usize);
    
    if moved_cords.x >= MAP_WIDTH || moved_cords.y >= MAP_LENGTH {  // moving out of bound
        return;
    }

    if state.map.map[moved_cords.get_1d()] == Tile::Wall {  // moving on wall
        return;
    }

    let move_on_monster = state.monsters.as_ref().unwrap()
        .iter()
        .any(|m| m.pos == moved_cords);

    if move_on_monster {  // moving on monster
        return;
    }
    
    state.player.as_mut().unwrap().pos = moved_cords;

}
