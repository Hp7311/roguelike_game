// provides Map
use crate::entities::Tile;
use crate::entities;
use rand::Rng;
use crossterm::cursor::MoveToColumn;
use crossterm::execute;
use std::io::Write;

#[derive(Debug)]
pub struct Map(Vec<Vec<Tile>>);


impl Map {
    pub fn draw(&self) {
    
        let mut stdout = std::io::stdout().lock();
        execute!(
            stdout, MoveToColumn(0)
        ).unwrap();
        
        for inner in &self.0 {
            execute!(
                stdout, MoveToColumn(0)
            ).unwrap();
            
            for tile in inner {
                let print_ch = match tile {
                    Tile::Wall => "# ",
                    Tile::Player => "@ ",
                    Tile::Monster => "M ",  // TODO add more
                    Tile::Floor => "- ",
                };
                write!(stdout, "{}", print_ch).unwrap();
            }
            writeln!(stdout).unwrap();
        }
    }
    
    /*pub fn handle_monsters(&self) -> Self {
        return_map = Map(init_map)
        for small in &self.0 {
            for tile in small {
                if tile == Tile::Monster {
                    if entities::monster_can_see_you(self) {
                        entities::monster_move_to_player(self);
                    }
                }
                
            }
        }
    }*/
    
    pub fn move_player(&self, to: char) -> Self {
        // moves player to a direction and returns new map
        
        let mut return_vec = self.0.clone();
        let mut player_pos = None;
        
        // determine player position
        for (a, inner) in return_vec.iter().enumerate() {
            for (i, tile) in inner.iter().enumerate() {
                if *tile == Tile::Player {
                    player_pos = Some((a, i))
                }
            }
        }
        
        if let Some((a, i)) = player_pos{
        
            match to {
        
                'w' => {
                    if a != 0 && return_vec[a-1][i] == Tile::Floor {
                        return_vec[a][i] = Tile::Floor;
                        return_vec[a-1][i] = Tile::Player;
                    }
                },
            
                'a' => {
                    if i != 0 && return_vec[a][i-1] == Tile::Floor {
                        return_vec[a][i] = Tile::Floor;
                        return_vec[a][i-1] = Tile::Player;
                    }
                },
            
                's' => {
                    if a != return_vec.len()-1 && return_vec[a+1][i] == Tile::Floor {
                        return_vec[a][i] = Tile::Floor;
                        return_vec[a+1][i] = Tile::Player;
                    }
                },
            
                'd' => {
                    if i != return_vec[0].len()-1 && return_vec[a][i+1] == Tile::Floor {
                        return_vec[a][i] = Tile::Floor;
                        return_vec[a][i+1] = Tile::Player;
                    }
                },
            
                _ => {},
            }
        
        }
        
        Map (return_vec)
    }
    
    //pub fn handle_player(&self) {}
    //pub fn get_player_stats(&self)
    
    fn dig_floors(&self) -> Self {
        let mut return_vec = Vec::new();
        
        /*const WIDTH: i32 = self.0.len();
        const LENGTH: i32 = self.0.0.len();*/
        
        for inner in &self.0 {
            return_vec.push(Vec::new());
            let index: usize = return_vec.len() - 1;
            
            for _ in inner {
                
                // TODO add corridors, not random
                let b = rand::random();
                if b {
                    return_vec[index].push(Tile::Floor);
                } else {
                    return_vec[index].push(Tile::Wall);
                }
            }
        }
        
        Map (return_vec)
    }
    
    fn add_player(&self) -> Self {
        let mut return_vec = self.0.clone();
        let x = rand::rng().random_range(..return_vec.len());
        let y = rand::rng().random_range(..return_vec[0].len());
        return_vec[x][y] = Tile::Player;
        Map (return_vec)
    }
}


pub fn init_map() -> Map {
    /*   length
      |---------|
      
      -
      |
 width|
      |
      -
    */
    const LENGTH: i32 = 10;
    const WIDTH: i32 = 10;
    let mut return_vec = Vec::new();
    
    for inner_vec in 0..WIDTH {
        return_vec.push(Vec::new());
        for _ in 0..LENGTH {
            return_vec[inner_vec as usize].push(Tile::Wall);
        }
    }
    
    Map (return_vec)
        .dig_floors()
        .add_player()
    
}