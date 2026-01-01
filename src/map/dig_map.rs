/// provides functions for drawing floors /*adding entities*/ on top of initial map
use crate::CONSTANTS::{MAX_ROOM_NUM, MAX_ROOM_LENGTH, MAX_ROOM_WIDTH};
use crate::maths::{Cord, Rect};
use crate::map::Map;
use crate::map::Tile;
use rand::prelude::*;


/// digs rooms and connect them per constants.rs
pub fn dig(map: Map) -> Result<Map, BuildError> {
    let mut rng = rand::rng();
    let mut ret = map;
    
    let mut rects = Vec::new();
    
    // generate n number of rooms
    for room_num in 0..MAX_ROOM_NUM {
    
        loop {  // loop until satisfied
            let mut valid = false;
            let mut doesnt_overlap = Vec::new();
            
            let width = rng.random_range(1..MAX_ROOM_WIDTH);
            let length = rng.random_range(1..MAX_ROOM_LENGTH);
            let start_cords = Cord::new(rng.random_range(0..map.len()), rng.random_range(0..map.0.len()));
            
            let rect_built = Rect::new(start_cords, length, width);
            
            
            if !rect_built.can_fit(&map) {
                continue;
            }
            
            for rect in rects {
                if rect_built.overlaps_with(rect) {
                    break;
                }
                doesnt_overlap.push(true)
            }
            
            if !(doesnt_overlap.len() == rects.len()) {
                continue;
            }
            
            // all fine, pushes the built room into Vec
            rects.push(rect_built);
        }
        
        // draw the rect
        let all_pixels: Vec<Cord> = rects[room_num].get_all_pixels()
        
        all_pixels.iter()
            .for_each(|&pixel| {
                ret[pixel.x][pixel.y] = Tile::Floor;
            })
    }
    
    // TODO dig corridors
    
    Ok(ret)
    
}
