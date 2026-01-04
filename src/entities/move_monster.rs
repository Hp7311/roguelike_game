/// handles moving monsters logic using a scent map
use crate::state::State;
use crate::entities::Monster;
use crate::map::{Map, Tile};
use crate::maths::Cord;
use crate::state::Direction;
use Direction::*;
use std::collections::{VecDeque, HashMap};

/*[(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
const NSWE_DIRS: [Direction; 4] = [Right, Down, Left, Up];*/


pub fn move_monsters(state: &mut State) {// -> Vec<Monster> {
    
    let s_map = get_scent_map(state.map.map.clone(), state.player.pos.clone());

    for monster in &mut state.monsters {
        let direction = look_around(&monster.pos, &s_map);
        
        match direction {  // known size problem
            Up => monster.pos.x -= 1,
            Down => monster.pos.x += 1,
            Right => monster.pos.y += 1,
            Left => monster.pos.y -= 1,
        }
    }
}


/// returns where the monster sould move
fn look_around(pos: &Cord, scent_map: &Vec<Option<u32>>) -> Direction {  // known size problem

    let mut smallest = None;
    let mut ret = None;
    
    for (move_by, dir) in get_nswe() {
        let x = pos.x as i32 + move_by.0;  // problem accessing the tuple
        let y = pos.y as i32 + move_by.1;
        
        if x < 0 || y < 0 {
            continue;
        }
        
        let shifted_cords = Cord::new(x as usize, y as usize);
        
        if let Some(tile) = scent_map.get(shifted_cords.get_1d())
            && let Some(num) = tile {
            
            if let Some(sm) = smallest {
                if num < sm {
                    smallest = Some(num);
                    ret = Some(dir);
                }
            }
            else {
                smallest = Some(num)
            }
        }
    }
    
    ret.unwrap()
}


/// returns scent map
fn get_scent_map(map: Vec<Tile>, player: Cord) -> Vec<Option<u32>> {
    let mut ret = vec![None; map.len()];
    let mut queue = VecDeque::new();
    
    
    // start with 0
    ret[player.get_1d()] = Some(0);
    queue.push_back(player);
    
    while let Some(exploring) = queue.pop_front() {
    
        let num_tobe_passed = ret[exploring.get_1d()].unwrap() + 1;
    
        for (move_by, _) in get_nswe() {
        
            let x = exploring.x as i32 + move_by.0;
            let y = exploring.y as i32 + move_by.1;
            
            if x < 0 || y < 0 {  // save conversion to usize later
                continue;
            }
            
            let shifted_cords = Cord::new(x as usize, y as usize);
            
            // if not out of bound
            if let Some(tile) = map.get(shifted_cords.get_1d()) {
            
                if *tile == Tile::Wall {  // looking on wall
                    continue;
                }
                
                // scent it and add it to the queue
                ret[shifted_cords.get_1d()] = Some(num_tobe_passed);
                queue.push_back(shifted_cords)
            }
        }
    }
    
    ret
}


fn get_nswe() -> HashMap<(i32, i32), Direction> {
    HashMap::from([
        ((0, 1), Right),
        ((0, -1), Left),
        ((1, 0), Down),
        ((-1, 0), Up),
    ])
}