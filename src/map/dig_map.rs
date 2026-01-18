//! provides functions for drawing floors /*adding entities*/ on top of initial map
use crate::constants::{
    MAX_ROOM_NUM, MAX_ROOM_LENGTH, MAX_ROOM_WIDTH,
    MAP_LENGTH, MAP_WIDTH, RANDOM_CORRIDOR_NUM
};
use crate::maths::{Cord, Rect};
use crate::map::{Map, Tile};
use crate::errors::BuildError;

use rand::prelude::*;
use log::info;


/// digs rooms and connect them per constants.rs
pub fn dig(map: &mut Map) -> Result<Vec<Rect>, BuildError> {

    let mut rng = rand::rng();

    let mut rects: Vec<Rect> = Vec::new();

    // ROOMS

    for room_num in 0..MAX_ROOM_NUM {

        loop {  // loop until satisfied
            let width = rng.random_range(1..MAX_ROOM_WIDTH);
            let length = rng.random_range(1..MAX_ROOM_LENGTH);
            let start_cords = Cord::new(rng.random_range(0..MAP_WIDTH), rng.random_range(0..MAP_LENGTH));

            let rect_built = Rect::new(start_cords, length, width);


            if !rect_built.can_fit() {
                continue;
            }

            if rects.iter().any(|rect| rect_built.overlaps_with(rect) ) {
                continue;
            }

            // all fine, pushes the built room into Vec
            rects.push(rect_built);
            break;
        }

        // draw the rect
        let all_pixels: Vec<Cord> = rects[room_num].get_all_pixels();

        for pixel in all_pixels {
            map.map[pixel.get_1d()] = Tile::Floor;
        }
    }
    
    // CORRIDORS
    let mut center_cords = vec![];

    for rect in rects.iter() {
        center_cords.push(rect.get_center());
    }


    let floors: Vec<Cord> = dig_regular_corridors(center_cords.clone())?;
    

    for floor in floors {
        map.map[floor.get_1d()] = Tile::Floor;
    }


    let random_floors = dig_random_corridors(center_cords.clone())?;

    for floor in random_floors {
        map.map[floor.get_1d()] = Tile::Floor;
    }


    Ok(rects)

}

#[derive(Debug, PartialEq)]
enum TunnelDirection {
    Horizontal,
    Vertical,
}
use TunnelDirection::*;


/// takes vector of Rect middle points, returns vector of floors that should be dug
fn dig_regular_corridors(centers: Vec<Cord>) -> Result<Vec<Cord>, BuildError> {
    let mut ret = vec![];

    for pair in centers.windows(2) {
        let previous = &pair[0];
        let current = &pair[1];

        let dir = if rand::random() {
            TunnelDirection::Horizontal
        } else {
            TunnelDirection::Vertical
        };
        
        info!("Digging between {} and {}", previous, current);

        let tunnel = dig_tunnel_general(&previous, &current, dir)?;

        // dbg
        if tunnel.len() == 0 {
            info!("Err digging");
        }
        //info!("{:?}", tunnel);

        ret.extend(tunnel);
        
    }
    
    Ok(ret)
}


fn dig_random_corridors(centers: Vec<Cord>) -> Result<Vec<Cord>, BuildError> {
    let mut rng = rand::rng();
    let mut ret = vec![];
    
    for _ in 0..RANDOM_CORRIDOR_NUM {
        let first = centers.choose(&mut rng).unwrap().clone();
        let mut second = centers.choose(&mut rng).unwrap().clone();
        loop {
            if second != first {
                break;
            }
            second = centers.choose(&mut rng).unwrap().clone();
        }

        info!("Digging between {} and {} (rand)", first, second);

        if rand::random() {
            ret.extend(dig_tunnel_general(&first, &second, TunnelDirection::Horizontal)?);
        }
        else {
            ret.extend(dig_tunnel_general(&first, &second, TunnelDirection::Vertical)?);
        }
    }
    
    Ok(ret)
}


/// Main operating func on digging corridors. takes two points and return cords that should be dug to floor
/// Horizonal = draw tunnel from point1 horizonally
/// Vertical = draw tunnel from point1 vertically
fn dig_tunnel_general(point1: &Cord, point2: &Cord, dir: TunnelDirection) -> Result<Vec<Cord>, BuildError> {

    if point1 == point2 {
        return Err(BuildError::RoomCenterSame(
            format!("{} and {}", point1, point2)
        ));
    }
    let mut ret = vec![];

    // default to draw horizontal with point1
    let (hor, vert) = if dir == Horizontal {
        (*point1, *point2)
    } else {
        (*point2, *point1)
    };

    // extend hor until meets vert
    let mut pushing_cord = hor;

    loop {
        ret.push(pushing_cord);

        pushing_cord.y = if hor.y > vert.y {  // should go left
            pushing_cord.y - 1
        } else if hor.y < vert.y {  // should go right
            pushing_cord.y + 1
        } else {  // two points vertical
            break;
        };

        if pushing_cord.y == vert.y {  // push intersect point, exit
            ret.push(pushing_cord);
            break;
        }
    }

    info!("Finished horizonal");
    // extend vert until meets hor
    let mut pushing_cord = vert;

    loop {
        ret.push(pushing_cord);

        pushing_cord.x = if vert.x > hor.x {  // should go up
            pushing_cord.x - 1
        } else if vert.x < hor.x {  // should go down
            pushing_cord.x + 1
        } else {  // two points horizonal
            break;
        };

        if pushing_cord.x == hor.x {  // doesn't push intersect point, exit
            break;
        }
    }
    info!("Finished vertical");


    Ok(ret)
}
