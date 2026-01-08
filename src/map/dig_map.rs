/// provides functions for drawing floors /*adding entities*/ on top of initial map
use crate::CONSTANTS::{
    MAX_ROOM_NUM, MAX_ROOM_LENGTH, MAX_ROOM_WIDTH,
    MAP_LENGTH, MAP_WIDTH, RANDOM_CORRIDOR_NUM
};
use crate::maths::{Cord, Rect};
use crate::map::{Map, Tile};
use crate::state::StateError;  // TODO temporary
use rand::prelude::*;
use log::{
    debug, info
};
use std::collections::VecDeque;

/*pub enum BuildError {  // TODO implement it
    General(String),
}*/

/// digs rooms and connect them per constants.rs
pub fn dig(map: &mut Map) {  // TODO actual error handling

    let mut rng = rand::rng();

    let mut rects: Vec<Rect> = Vec::new();

    // generate n number of rooms

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
            info!("Room: {}", rect_built);
            rects.push(rect_built);
            break;
        }

        // draw the rect
        let all_pixels: Vec<Cord> = rects[room_num].get_all_pixels();

        for pixel in all_pixels {
            map.map[pixel.get_1d()] = Tile::Floor;
        }
    }
    
    // dig corridors
    let mut center_cords = Vec::new();

    for rect in rects {
        center_cords.push(rect.get_center());
        //info!("Got center cord {} of {}", rect.get_center(), rect)
    }


    let floors: Vec<Cord> = dig_regular_corridors(center_cords.clone());
    for fl in floors {
        map.map[fl.get_1d()] = Tile::Floor;
    }

    let random_floors = dig_random_corridors(center_cords.clone());

    for fl in random_floors {
        map.map[fl.get_1d()] = Tile::Floor
    }
}

enum Tunnel {
    Horizontal,
    Vertical,
}



/// takes vector of Rect middle points, returns vector of floors that should be dug
fn dig_regular_corridors(centers: Vec<Cord>) -> Vec<Cord> {
    let mut ret = Vec::new();

    for pair in centers.windows(2) {
        let previous = &pair[0];
        let current = &pair[1];

        let dir = if rand::random() {
            Tunnel::Horizontal
        } else {
            Tunnel::Vertical
        };
        //info!("General: Got point1: {}, point2: {}", previous, current);
        let tunnel = dig_tunnel_general(&previous, &current, dir);
        ret.extend(tunnel);
        //info!("Success")
    }
    
    ret
}


fn dig_random_corridors(centers: Vec<Cord>) -> Vec<Cord> {
    let mut rng = rand::rng();
    let mut ret = Vec::new();
    
    for _ in 0..RANDOM_CORRIDOR_NUM {
        let first = centers.choose(&mut rng).unwrap().clone();
        let mut second = centers.choose(&mut rng).unwrap().clone();
        loop {
            if second != first {
                break;
            }
            second = centers.choose(&mut rng).unwrap().clone();
        }
        //info!("Random: Got point1: {}, point2: {}", first, second);
        if rand::random() {
            ret.extend(dig_tunnel_general(&first, &second, Tunnel::Horizontal));  // may still complain about expect &T got T
        }
        else {
            ret.extend(dig_tunnel_general(&first, &second, Tunnel::Vertical));  // may still complain about expect &T got T
        }
        //info!("Success");
    }
    
    ret
}


/// Main operating func on digging corridors. takes two points and return cords that should be dug -> floor
fn dig_tunnel_general(point1: &Cord, point2: &Cord, dir: Tunnel) -> Vec<Cord> {
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
        });  // not idiomatic
    
    // determine where they intersect
    let mut intersect = Cord::new(0, 0);  // not perfect
    
    point1_ext.iter()
        .any(|p1| {
            if point2_ext.iter()
                .any(|p2| p1 == p2)
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
                    .filter(|cord| {
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
                    .cloned()
                    .collect::<Vec<_>>() );
            }
            
            
            // intersect at right of point1
            else if intersect.y > point1.y {
            
                // push ___ part of tunnel
                point1_ext.iter()
                    .for_each(|horiz_cords| {
                        if horiz_cords.y < point1.y || horiz_cords.y > intersect.y {
                            
                        }
                        else {
                            ret.push(horiz_cords.clone())  // hopefully fixes the expected Vec<&Cord> found Vec<Cord> issue
                        }
                    });
                    
                
                // push | part of tunnel
                
                // intersect below point2
                if intersect.x > point2.x {
                
                    point2_ext.iter()
                        .for_each(|vert_cords| {
                            // using >= to not include intersect twice
                            if vert_cords.x < point2.y || vert_cords.x >= intersect.x {
                                
                            } else {
                                ret.push(vert_cords.clone());
                            }
                        })
                }
                
                // intersect above point2
                else if intersect.x < point2.x {
                    
                    point2_ext.iter()
                        .for_each(|vert_cords| {
                            if vert_cords.x <= intersect.x || vert_cords.x > point2.x {
                                
                            }
                            else {
                                ret.push(vert_cords.clone());
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
                    .for_each(|horiz_cords| {
                        if horiz_cords.y < intersect.y || horiz_cords.y > point1.y {
                            
                        }
                        else {
                            ret.push(horiz_cords.clone())
                        }
                    });
                
                // push | part of tunnel
                
                // intersect below point2
                if intersect.x > point2.x {
                    point2_ext.iter()
                        .for_each(|vert_cords| {
                            // >= to avoid duplicate
                            if vert_cords.x < point2.x || vert_cords.x >= intersect.x {
                                
                            }
                            else {
                                ret.push(vert_cords.clone());
                            }
                        })
                }
                
                // intersect above point2
                else if intersect.x < point2.x {
                    point2_ext.iter()
                        .for_each(|vert_cords| {
                            if vert_cords.x <= intersect.x || vert_cords.x > point2.x {
                                
                            }
                            else {
                                ret.push(vert_cords.clone());
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
        
        
        
        Vertical => {  // point1 going vertically
            
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
                    .cloned()
                    .collect::<Vec<_>>() );
            }
            
            // two points parallel
            // intersect at right of point2
            else if intersect.y > point2.y {
            
                // push ___ part of tunnel
                point2_ext.iter()
                    .for_each(|horiz_cords| {
                        if horiz_cords.y < point2.y || horiz_cords.y > intersect.y {
                            
                        }
                        else {
                            ret.push(horiz_cords.clone());
                        }
                    });
                    
                
                // push | part of tunnel
                
                // intersect below point1
                if intersect.x > point1.x {
                
                    point1_ext.iter()
                        .for_each(|vert_cords| {
                            // using >= to not include intersect twice
                            if vert_cords.x < point1.y || vert_cords.x >= intersect.x {
                                
                            } else {
                                ret.push(vert_cords.clone());
                            }
                        })
                }
                // intersect above point2
                else if intersect.x < point1.x {
                    
                    point1_ext.iter()
                        .for_each(|vert_cords| {
                            if vert_cords.x <= intersect.x || vert_cords.x > point1.x {
                                
                            }
                            else {
                                ret.push(vert_cords.clone());
                            }
                        })
                }
                // Horizontal left
                else if intersect.x == point1.x {
                    // already covered
                }

                else {
                    panic!("Impossible two rooms with same center");
                }
            }
            
            
            // intersect at left of point2
            else if intersect.y < point2.y {
            
                // push ____ part of tunnel
                point2_ext.iter()
                    .for_each(|horiz_cords| {
                        if horiz_cords.y < intersect.y || horiz_cords.y > point2.y {
                            
                        }
                        else {
                            ret.push(horiz_cords.clone());
                        }
                    });
                
                // push | part of tunnel
                
                // intersect below point1
                if intersect.x > point1.x {
                    point1_ext.iter()
                        .for_each(|vert_cords| {
                            // >= to avoid duplicate
                            if vert_cords.x < point1.x || vert_cords.x >= intersect.x {
                                
                            }
                            else {
                                ret.push(vert_cords.clone());
                            }
                        })
                }
                
                // intersect above point1
                else if intersect.x < point1.x {
                    point1_ext.iter()
                        .for_each(|vert_cords| {
                            if vert_cords.x <= intersect.x || vert_cords.x > point1.x {
                                
                            }
                            else {
                                ret.push(vert_cords.clone());
                            }
                        })
                }
                // Horizontal left
                else if intersect.x == point1.x {
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
    }
    
    ret
}