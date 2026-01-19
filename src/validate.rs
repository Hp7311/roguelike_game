//! function to test if State is valid. returns Err in dev mode, restart in release mode.
use crate::errors::ValidateError;
use crate::entities::get_nswe;
use ValidateError::*;
use crate::state::State;
use crate::map::Tile;
use crate::maths::Cord;
use crate::constants::{
    MAP_LENGTH, MAP_WIDTH
};


pub fn validate_state(state: State) -> Result<State, ValidateError> {
    validate_map(&state)?;
    validate_player(&state)?;
    validate_monsters(&state)?;

    Ok(state)
}

fn validate_map(state: &State) -> Result<(), ValidateError> {

    let map = &state.map.map;
    // floors exist
    if !map.contains(&Tile::Floor) {
        return Err(MapErr("Map all walls".into()))
    }

    // correct length
    if map.len() != *MAP_LENGTH * *MAP_WIDTH {
        return Err(MapErr(format!("Incorrect length: {}", state.map.map.len())))
    }


    // rooms (by dumb method)
    for room in &state.rooms {
        let mut tunnel_tiles = 0;

        for edge in room.get_edges() {
            //info!("Got edge {edge}");
            for ((x, y), _) in get_nswe() {

                let shifted_x = edge.x as i32 + x;
                let shifted_y = edge.y as i32 + y;

                if shifted_x < 0 || shifted_y < 0 {
                    continue;
                }
                let shifted_cords: Cord = Cord::new(shifted_x as usize, shifted_y as usize);

                if shifted_cords.in_rect(room) {
                    continue;
                }

                if state.map.map.get(shifted_cords.get_1d()).is_none() {
                    continue;
                }

                if state.map.map[shifted_cords.get_1d()] == Tile::Wall {
                    continue;
                }
                //info!("Got one");
                tunnel_tiles += 1;
            }
        }
        
        //info!("");

        if tunnel_tiles == 0 {
            /*println!("Invalid?");
            //dbg!(room);*/
            state.clear_screen().unwrap();
            state.render().unwrap();
            //std::process::exit(1);
            dbg!(&state.rooms);
            return Err(RoomIsolatedError(room.clone()));
        }
    }


    Ok(())
}


/// player on floor and a room
fn validate_player(state: &State) -> Result<(), ValidateError> {
    
    let player_pos = state.player.as_ref()
        .ok_or(PlayerErr("not spawned yet".into(), Cord::new(0, 0)))?
        .pos;

    let tile = state.map.map.get(player_pos.get_1d())
        .ok_or(PlayerErr("out of bound".into(), player_pos))?;

    if !(*tile == Tile::Floor) {
        return Err(PlayerErr("on wall".into(), player_pos));
    }
    
    let in_room = state.rooms.iter()
        .any(|r| player_pos.in_rect(r));

    if !in_room {
        return Err(PlayerErr("not in a room".into(), player_pos));
    }

    Ok(())
}


/// monsters on floor
fn validate_monsters(state: &State) -> Result<(), ValidateError> {
    let monsters = state.monsters.as_ref()
        .ok_or(MonsterErr("not spawned yet".into(), Cord::new(0, 0)))?;  // Cord<0, 0> = N/A

    let player_pos = state.player.as_ref()
        .ok_or(PlayerErr("not spawned yet".into(), Cord::new(0, 0)))?
        .pos;

    let map = &state.map.map.clone();
    let rooms = &state.rooms;

    if monsters.iter().any(|m| m.pos == player_pos) {
        return Err(MonsterErr("standing on player".into(), player_pos))
    }

    for monster in monsters.iter() {

        let tile = map.get(monster.pos.get_1d())
            .ok_or(MonsterErr("out of bound".into(), monster.pos))?;
        
        if *tile == Tile::Wall {
            return Err(MonsterErr("on wall".into(), monster.pos));
        }

        if !(rooms.iter().any(|r| monster.pos.in_rect(r))) {
            return Err(MonsterErr("not in a room".into(), monster.pos));
        }
    }

    Ok(())
}
