use crate::map::Map;
use crate::entities::Tile;
use std::collections::VecDeque;

pub fn check_map_valid(map: Map) -> bool {
    let mut monster_list = Vec::new();
    let mut player_pos = (0, 0);
    
    let return_vec = map.0.clone();
    
    for (a, inner) in return_vec.iter().enumerate() {
        for (i, tile) in inner.iter().enumerate() {
            match tile {
                Tile::Player(_) => player_pos = (a, i),
                Tile::Monster(_) => {
                    monster_list.push(
                        (a, i)
                    )
                },
                _ => {},
            }
        }
    }
    
    for monster in monster_list {
        if !(can_reach(return_vec.clone(), player_pos, monster)) {
            return false
        }
    }
    
    true
}

fn can_reach(map: Vec<Vec<Tile>>, start: (usize, usize), target: (usize, usize)) -> bool {
    // BFS calculating if reachable
    
    let length = map.len();
    let width = map[0].len();
    let mut queue = VecDeque::new();
    let mut visited = vec![
        vec![false; length];
        width
    ];
    
    
    queue.push_back(start);
    
    // while queue not empty
    while let Some((exploring_x, exploring_y)) = queue.pop_front() {
    
        // if exploring tile is target, YES
        if (exploring_x, exploring_y) == target {
            return true;
        }
        
        let target = (target.0 as isize, target.1 as isize);
        
        // exlore surrounding 4 tiles
        for (shift_index_x, shift_index_y) in [(0, 1), (0, -1isize), (-1isize, 0), (1, 0)] {
            let shifted_x = exploring_x as isize + shift_index_x;
            let shifted_y = exploring_y as isize + shift_index_y;
            
            // if surrounding tile is target, YES
            if (shifted_y, shifted_y) == target {
                return true;
            }
            
            if shifted_x < 0 || shifted_x > width as isize - 1 {
                continue;
            }
            if shifted_y < 0 || shifted_y > length as isize -1 {
                continue;
            }
            
            let shifted_x = shifted_x as usize;
            let shifted_y = shifted_y as usize;
            
            // if not wall and not visited before
            if !(map[shifted_x][shifted_y] == Tile::Wall) && !visited[shifted_x][shifted_y] {
                queue.push_back((shifted_x, shifted_y));
                visited[shifted_x][shifted_y] = true;
            }
        }
        
    }
    false
}