/// provides functions for drawing floors /*adding entities*/ on top of initial map
use crate::CONSTANTS::{
    MAX_ROOM_NUM, MAX_ROOM_LENGTH, MAX_ROOM_WIDTH,
    MAP_LENGTH, MAP_WIDTH, RANDOM_CORRIDOR_NUM
};
use crate::maths::{Cord, Rect};
use crate::map::Map;
use crate::map::Tile;
use crate::state::StateError;  // TODO temporary
use rand::prelude::*;

/*pub enum BuildError {  // TODO implement it
    General(String),
}*/

/// digs rooms and connect them per constants.rs
pub fn dig(map: Map) -> Result<Map, StateError> {  // TODO actual error handling
    
    let mut rng = rand::rng();
    let map = map.map;
    let mut ret = map.map;
    
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
            
            
            if !rect_built.can_fit() {
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
            break;
        }
        
        // draw the rect
        let all_pixels: Vec<Cord> = rects[room_num].get_all_pixels();
        
        all_pixels.iter()
            .for_each(|&pixel| {
                ret[pixel.x][pixel.y] = Tile::Floor;
            })
    }
    
    // dig corridors
    let mut center_cords = Vec::new();
    
    for rect in rects {
        center_cords.push(rect.get_center());
    }
    
    let floors: Vec<Cord> = dig_regular_corridors(center_cords);
    floors.iter()
        .for_each(|fl| ret[fl.x][fl.y] = Tile::Floor);
        
    let random_floors = dig_random_corridors(center_cords);
    random_floors.iter()
        .for_each(|fl| ret[fl.x][fl.y] = Tile::Floor);
    
    Ok(Map {map: ret})
    
}

enum Tunnel {
    Horizontal,
    Vertical,
}



/// takes vector of Rect middle points, returns vector of floors that should be dug
fn dig_regular_corridors(centers: Vec<Cord>) -> Vec<Cord> {
    let mut ret = Vec::new();
    let mut previous_point = centers[0];
    
    for point in &centers[1..] {
        if rand::random() {
            ret.extend(dig_tunnel_general(previous_point, point, Tunnel::Horizontal));
        }
        else {
            ret.extend(dig_tunnel_general(previous_point, point, Tunnel::Vertical));
        }
    }
    
    ret
}


fn dig_random_corridors(centers: Vec<Cord>) -> Vec<Cord> {
    let mut rng = rand::rng();
    let mut ret = Vec::new();
    
    for _ in 0..RANDOM_CORRIDOR_NUM {
        let first = centers.choose(&mut rng).unwrap();
        let mut second = centers.choose(&mut rng).unwrap();
        loop {
            if second == first {
                break;
            }
            second = centers.choose(&mut rng).unwrap();
        }
        
        if rand::random() {
            ret.extend(dig_tunnel_general(first, second, Tunnel::Horizontal));  // may still complain about expect &T got T
        }
        else {
            ret.extend(dig_tunnel_general(first, second, Tunnel::Vertical));  // may still complain about expect &T got T
        }
    }
    
    ret
}


/// Main operating func on digging corridors. takes two points and return cords that should be dug -> floor
fn dig_tunnel_general(point1: Cord, point2: Cord, dir: Tunnel) -> Vec<Cord> {
    use Tunnel::*;

    // extend the two points based on rand and `dir`

    let mut point1_ext = Vec::new();
    let mut point2_ext = Vec::new();
    let mut ret = Vec::new();

    // the point that goes ______
    (0..MAP_LENGTH).collect::<Vec<_>>()
        .iter()
        .for_each(|&y| {
            match dir {
                Horizontal => point1_ext.push(Cord::new(point1.x, y)),
                Vertical => point2_ext.push(Cord::new(point2.x, y)),
            }
        });

    // the point that goes |
    (0..MAP_WIDTH).collect::<Vec<_>>()
        .iter()
        .for_each(|&x| {
            match dir {
                Horizontal => point2_ext.push(Cord::new(x, point2.y)),
                Vertical => point1_ext.push(Cord::new(x, point1.y)),
            }
        });
    
    // determine where they intersect
    let mut intersect: Cord;
    
    point1_ext.iter()
        .any(|&p1| {
            if point2_ext.iter()
                .any(|&p2| p1 == p2)
            {
                intersect = Cord::new(p1.x, p1.y);
                return true
            }
            false
        });
    
    // push floors
    match dir {
        Horizontal => {  // point1 going horizonally
        
            // two points vertical
            if intersect.y == point1.y {
            
                // push final results to ret
                
                ret.extend( point2_ext.iter()  // iterate through x
                    .filter(|&cord| {
                        let x = cord.x;
                        
                        if point2.x > point1.x {  // if point2 below point1
                            if x < point1.x || x > point2.x {
                                // cut off excess
                                return false
                            }
                            true
                        }
                        else {  // point1 below point2
                            if x < point2.x || x > point1.x {
                                return false
                            }
                            true
                        }
                    })
                    .map(|&cord| cord)
                    .collect::<Vec<_>>() );
            }
            
            
            // intersect at right of point1
            else if intersect.y > point1.y {
            
                // push ___ part of tunnel
                point1_ext.iter()
                    .for_each(|&horiz_cords| {
                        if horiz_cords.y < point1.y || horiz_cords.y > intersect.y {
                            
                        }
                        else {
                            ret.push(horiz_cords)  // hopefully fixes the expected Vec<&Cord> found Vec<Cord> issue
                        }
                    });
                    
                
                // push | part of tunnel
                
                // intersect below point2
                if intersect.x > point2.y {
                
                    point2_ext.iter()
                        .for_each(|&vert_cords| {
                            // using >= to not include intersect twice
                            if vert_cords.x < point2.y || vert_cords.x >= intersect.x {
                                
                            } else {
                                ret.push(vert_cords);
                            }
                        })
                }
                
                // intersect above point2
                else if intersect.x < point2.x {
                    
                    point2_ext.iter()
                        .for_each(|&vert_cords| {
                            if vert_cords.x <= intersect.x || vert_cords.x > point2.x {
                                
                            }
                            else {
                                ret.push(vert_cords);
                            }
                        })
                }
                
                // Horizontal right
                else if intersect.x == point2.x {
                    // nothing to be done, already covered
                }
                
                else {
                    panic!("Impossible two rooms with same center");
                }
            }
            
            
            // intersect at left of point1
            else if intersect.y < point1.y {
            
                // push ____ part of tunnel
                point1_ext.iter()
                    .for_each(|&horiz_cords| {
                        if horiz_cords.y < intersect.y || horiz_cords.y > point1.y {
                            
                        }
                        else {
                            ret.push(horiz_cords)
                        }
                    });
                
                // push | part of tunnel
                
                // intersect below point2
                if intersect.x > point2.x {
                    point2_ext.iter()
                        .for_each(|&vert_cords| {
                            // >= to avoid duplicate
                            if vert_cords.x < point2.x || vert_cords.x >= intersect.x {
                                
                            }
                            else {
                                ret.push(vert_cords);
                            }
                        })
                }
                
                // intersect above point2
                else if intersect.x < point2.x {
                    point2_ext.iter()
                        .for_each(|&vert_cords| {
                            if vert_cords.x <= intersect.x || vert_cords.x > point2.x {
                                
                            }
                            else {
                                ret.push(vert_cords);
                            }
                        })
                }
                
                // Horizontal left
                else if intersect.x == point2.x {
                    // already covered
                }
                
                else {
                    panic!("Two rooms with same center point??!");
                }
            }
            
            // impossible
            else {
                panic!("Two rooms with same center point??!");
            }
        },
        
        
        
        Vertical => {
            // point1 going vertically
            
            // two points vertical
            if intersect.y == point2.y {
            
                // push final results to ret
                
                ret.extend( point1_ext.iter()  // iterate through x
                    .filter(|&cord| {
                        let x = cord.x;
                        
                        if point2.x > point1.x {  // if point2 below point1
                            if x < point1.x || x > point2.x {
                                return false;
                            }
                            true
                        }
                        else if point2.x < point1.x {  // point1 below point2
                            if x < point2.x || x > point1.x {
                                return false;
                            }
                            true
                        }
                        else {
                            panic!("2 room centers same");
                        }
                    })
                    .map(|&cord| cord)
                    .collect::<Vec<_>>() );
            }
            
            
            // intersect at right of point2
            else if intersect.y > point2.y {
            
                // push ___ part of tunnel
                point2_ext.iter()
                    .for_each(|&horiz_cords| {
                        if horiz_cords.y < point2.y || horiz_cords.y > intersect.y {
                            
                        }
                        else {
                            ret.push(horiz_cords);
                        }
                    });
                    
                
                // push | part of tunnel
                
                // intersect below point1
                if intersect.x > point1.y {
                
                    point1_ext.iter()
                        .for_each(|&vert_cords| {
                            // using >= to not include intersect twice
                            if vert_cords.x < point1.y || vert_cords.x >= intersect.x {
                                
                            } else {
                                ret.push(vert_cords);
                            }
                        })
                }
                // intersect above point2
                else if intersect.x < point1.x {
                    
                    point1_ext.iter()
                        .for_each(|&vert_cords| {
                            if vert_cords.x <= intersect.x || vert_cords.x > point1.x {
                                
                            }
                            else {
                                ret.push(vert_cords);
                            }
                        })
                }
                
                else {
                    panic!("Impossible two rooms with same center");
                }
            }
            
            
            // intersect at left of point2
            else if intersect.y < point2.y {
            
                // push ____ part of tunnel
                point2_ext.iter()
                    .for_each(|&horiz_cords| {
                        if horiz_cords.y < intersect.y || horiz_cords.y > point2.y {
                            
                        }
                        else {
                            ret.push(horiz_cords);
                        }
                    });
                
                // push | part of tunnel
                
                // intersect below point1
                if intersect.x > point1.x {
                    point1_ext.iter()
                        .for_each(|&vert_cords| {
                            // >= to avoid duplicate
                            if vert_cords.x < point1.x || vert_cords.x >= intersect.x {
                                
                            }
                            else {
                                ret.push(vert_cords);
                            }
                        })
                }
                
                // intersect above point1
                else if intersect.x < point1.x {
                    point1_ext.iter()
                        .for_each(|&vert_cords| {
                            if vert_cords.x <= intersect.x || vert_cords.x > point1.x {
                                
                            }
                            else {
                                ret.push(vert_cords);
                            }
                        })
                }
                
                else {
                    panic!("Two rooms with same center point??!");
                }
            }
            
            // impossible
            else {
                panic!("Two rooms with same center point??!");
            }
        },
    }
    
    ret
}