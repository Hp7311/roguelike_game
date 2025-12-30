/// moves monsters using scent map
use crate::entities::Tile;
use std::collections::VecDeque;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Cord<T> {
    x: T,
    y: T,
}


pub fn moves_monsters(map: Vec<Vec<Tile>>) -> Vec<Vec<Tile>> {
    let mut return_vec = map.clone();
    let d_map = get_map(map.clone());
    let monster_list = get_monster_cords(&map);
    //info!("{:?}", d_map);
    
    // unwrap d_map of Option
    let d_map = d_map.iter()
        .map(|item| {
            item.iter()
                .map(|tile| {
                    if let Some(num) = tile { *num } else { 1000 } // not stable
                })  // relies on d_map correct
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
        
        
    for monster in monster_list {
    
        let mut nswe_cords = Vec::new();
        
        // get NSWE cords
        for (mv_x, mv_y) in [(0, 1), (0, -1isize), (1, 0), (-1isize, 0)] {
            // monster looks around it
            let shifted_cords = Cord {
                x: monster.x as isize + mv_x,
                y: monster.y as isize + mv_y,
            };
            
            // if out of bound
            if shifted_cords.x < 0 || shifted_cords.y < 0 || shifted_cords.x >= return_vec.len() as isize || shifted_cords.y >= return_vec.len() as isize{
                continue;
            }
            
            nswe_cords.push( Cord {
                x: shifted_cords.x as usize,
                y: shifted_cords.y as usize,
            });
        }
        
        // get smallest on scent map
        let mut smallest = d_map[nswe_cords[0].x][nswe_cords[0].y];
        let mut move_to: Cord<usize> = nswe_cords[0];
        
        nswe_cords.iter()
            .for_each(|cord| {
                if d_map[cord.x][cord.y] < smallest {
                    smallest = d_map[cord.x][cord.y];
                    move_to  = *cord;
                }
            });
        
        // if about to move on another monster/player
        if matches!(return_vec[move_to.x][move_to.y], Tile::Player(_))
            || matches!(return_vec[move_to.x][move_to.y], Tile::Monster(_)) {
            
            continue;
        }
        
        // move monster
        return_vec[move_to.x][move_to.y]
            = return_vec[monster.x][monster.y].clone();
            
        return_vec[monster.x][monster.y] = Tile::Floor;
    }
    
    return_vec
}




fn get_monster_cords(map: &[Vec<Tile>]) -> Vec<Cord<usize>> {
    map.iter()
        .enumerate()
        .flat_map(|(x, row)| {
            row.iter()
                .enumerate()
                .map(move |(y, tile)| (Cord {x, y}, tile))
        })
        .filter(|(_, tile)| matches!(**tile, Tile::Monster(_)))
        .map(|(cord, _)| cord)
        .collect()
}


/// returns A* map using player pos
fn get_map(map: Vec<Vec<Tile>>) -> Vec<Vec<Option<u32>>> {
    let width = map.len();
    let length = map[0].len();
    
    let mut d_map = vec![
        vec![None; length]; width
    ];
    
    let player_pos = map.iter()
        .enumerate()
        .find_map(|(x, row)| {
            row.iter()
                .position(|item| matches!(item, Tile::Player(_)))
                .map(|y| Cord { x, y })
        }).unwrap();
    
    let mut queue = VecDeque::new();
    
    // from player_pos, explore tiles and mark them
    d_map[player_pos.x][player_pos.y] = Some(0);
    queue.push_back(player_pos);
    
    while let Some(explore_cords) = queue.pop_front() {
        // get number to fill in the tile
        let mark_num = d_map[explore_cords.x][explore_cords.y].unwrap()
            + 1;
        
        for (mv_x, mv_y) in [(0, 1), (0, -1isize), (1, 0), (-1isize, 0)] {
        
            // get the moved cords
            let shifted_cords = Cord {
                x: explore_cords.x as isize + mv_x,
                y: explore_cords.y as isize + mv_y,
            };
            
            // look if its off map
            if shifted_cords.x < 0 || shifted_cords.y < 0 {
                continue;
            }
            
            let shifted_cords = Cord {
                x: shifted_cords.x as usize,
                y: shifted_cords.y as usize,
            };
            
            // if already filled
            if let Some(inner) = d_map.get(shifted_cords.x) {
                if let Some(tile) = inner.get(shifted_cords.y) {
                
                    if tile.is_some() {
                        continue;
                    }
                } 
                else {
                    continue;
                }
            }
            else {
                continue;
            }
            
            // if tile in map And not blocked, push the exploring tile and assign it to a num
            if let Some(inner) = map.get(shifted_cords.x)
                && let Some(tile) = inner.get(shifted_cords.y) {
                    
                    // if the flowing tile is Floor or Monster
                    if *tile == Tile::Floor || matches!(*tile, Tile::Monster(_)) {
                        queue.push_back(shifted_cords);
                        d_map[shifted_cords.x][shifted_cords.y] = Some(mark_num);
                    }
                }
        }
    }
    
    d_map
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_dmap() {
        use crate::entities::Tile::*;
        let test_map = vec![
            vec![Floor, Floor, Player(3)],
            vec![Floor, Wall, Wall],
            vec![Wall, Floor, Floor],
        ];
        
        assert_eq!(
            get_map(test_map).iter()
                .map(|item| {
                    item.iter()
                        .map(|tile| {
                            if let Some(num) = tile { return *num } else { return 1000 } // not stable
                        })  // relies on d_map correct
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>(),
            vec![
                vec![2, 1, 0],
                vec![3, 1000, 1000],
                vec![1000; 3],
            ]
        );
    }
}