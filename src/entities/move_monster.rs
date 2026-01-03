/// handles moving monsters logic using a scent map
use crate::state::State;
use crate::entities::Monster;
use crate::map::{Map, Tile};
use crate::maths::Cord;
use crate::state::Direction;
use Direction::*;
use std::collections::VecDeque;

const NSWE: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
const NSWE_DIRS: [Direction; 4] = [Right, Down, Left, Up];


pub fn move_monsters(state: &State) -> Vec<Monster> {
    let mut ret = Vec::new(); //state.monsters;
    let s_map = get_scent_map(state.map.map, state.player.unwrap().pos);
    
    for monster in state.monsters.unwrap() {

        let mut monster = monster;
        
        match look_around(monster.pos, &s_map) {  // known size problem
            Up => monster.pos.x -= 1,
            Down => monster.pos.x += 1,
            Right => monster.pos.y += 1,
            Down => monster.pos.y -= 1,
        }
        
        ret.push(monster);
    }
    
    ret
}


/// returns where the monster sould move
fn look_around(pos: Cord, scent_map: &Vec<Option<u32>>) -> Direction {  // known size problem

    let mut smallest = None;
    let mut ret = None;
    
    for (i, move_by) in NSWE.iter().enumerate() {
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
                    ret = Some(NSWE_DIRS[i]);
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
    
        for move_by in &NSWE {
        
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
