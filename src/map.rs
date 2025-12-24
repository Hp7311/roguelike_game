// provides Map
use crate::entities::{Tile, MonsterType};
use crate::entities;
use rand::Rng;
use rand::seq::SliceRandom;
use rand::prelude::IndexedRandom;
use crossterm::cursor::MoveToColumn;
use crossterm::execute;
use std::io::Write;

#[derive(Debug, Clone)]
pub struct Map(Vec<Vec<Tile>>);


impl Map {

    pub fn draw(&self) {
        
        for inner in &self.0 {
            execute!(
                std::io::stdout(), MoveToColumn(0)
            ).unwrap();
            
            for tile in inner {
                let print_ch = match tile {
                    Tile::Wall => "# ".to_string(),
                    Tile::Player(hp) => "@ ".to_string(),
                    Tile::Monster(Type) => format!("{} ", Type.glyph),  // TODO add more
                    Tile::Floor => "- ".to_string(),
                };
                print!("{}", print_ch);
            }
            println!();
        }
        println!("Your HP: {}", self.get_player_hp());
    }
    
    fn get_player_hp(&self) -> i32 {
    
        for inner in self.0.clone() {
            for tile in inner {
                if let Tile::Player(hp) = tile {
                    return hp
                }
            }
        }
        panic!("No player");
    }
    
    pub fn handle_monsters(&self) -> Self {
        /*let return_vec = self.clone().0;
        let mut monster_list = Vec::new();
        
        for (a, inner) in return_vec.iter().enumerate() {
            for (i, tile) in inner.iter().enumerate() {
                if let Tile::Monster(monstertype) = tile {
                    monster_list.push((
                        (a, i), monstertype
                    ))
                }
            }
        }*/
        
        self.clone()
    }
    
    pub fn move_player(&self, to: char) -> Self {
        // moves player to a direction and returns new map
        
        let mut return_vec = self.0.clone();
        let mut player_pos = None;
        
        // determine player position
        for (a, inner) in return_vec.iter().enumerate() {
            for (i, tile) in inner.iter().enumerate() {
                if let Tile::Player(_) = *tile {
                    player_pos = Some((a, i))
                }
            }
        }
        
        if let Some((a, i)) = player_pos {
            
            if let Tile::Player(player_hp) = return_vec[a][i] {
            
                match to {
        
                    'w' => {
                        if a != 0 && return_vec[a-1][i] == Tile::Floor {
                            return_vec[a][i] = Tile::Floor;
                            return_vec[a-1][i] = Tile::Player(player_hp);
                        }
                    },
            
                    'a' => {
                        if i != 0 && return_vec[a][i-1] == Tile::Floor {
                            return_vec[a][i] = Tile::Floor;
                            return_vec[a][i-1] = Tile::Player(player_hp);
                        }
                    },
            
                    's' => {
                        if a != return_vec.len()-1 && return_vec[a+1][i] == Tile::Floor {
                            return_vec[a][i] = Tile::Floor;
                            return_vec[a+1][i] = Tile::Player(player_hp);
                        }
                    },
            
                    'd' => {
                        if i != return_vec[0].len()-1 && return_vec[a][i+1] == Tile::Floor {
                            return_vec[a][i] = Tile::Floor;
                            return_vec[a][i+1] = Tile::Player(player_hp);
                        }
                    },
            
                    _ => {},
                }
            }
        }
        
        Map (return_vec)
    }
    
    pub fn handle_player(&self) -> Self {
        let mut return_vec = self.0.clone();
        let mut player_pos = None;
        let mut monster_list = Vec::new();
        
        for (a, inner) in return_vec.iter().enumerate() {
            for (i, tile) in inner.iter().enumerate() {
                if let Tile::Player(_) = tile {
                    player_pos = Some((a, i));
                }
                if let Tile::Monster(_) = tile {
                    monster_list.push((a, i));
                }
            }
        }
        
        if let Some(player) = player_pos {
            for monster in monster_list {
                if entities::in_range(player, monster) {
                    return_vec = self.attack(
                        player,
                        monster,
                        entities::PLAYER_STRENGTH,
                    )
                }
            }
        }
        
        Map (return_vec)
    }
    
    pub fn get_player_stats(&self) {
        println!("You died!");
    }
    
    pub fn delete_dead(&self) -> Self {
        let mut return_vec = self.0.clone();
        
        for (a, inner) in self.0.clone().iter().enumerate() {
            for (i, tile) in inner.iter().enumerate() {
                match tile {
                    Tile::Player(hp) => {
                        if *hp < 1 {
                            return_vec[a][i] = Tile::Floor;
                        }
                    },
                    Tile::Monster(monstertype) => {
                        if monstertype.hp < 1 {
                            return_vec[a][i] = Tile::Floor;
                        }
                    },
                    _ => {},
                }
            }
        }
        
        Map (return_vec)
    }
    
    pub fn player_exists(&self) -> bool {
        let mut player_exists = false;
        
        for inner in &self.0 {
            for tile in inner {
                if let Tile::Player(_) = tile {
                    player_exists = true;
                    break;
                }
            }
        }
        
        player_exists
    }
    
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
        return_vec[x][y] = Tile::Player(100);
        Map (return_vec)
    }
    
    fn add_monsters(&self, num: u32) -> Self {
    
        let monster_types = vec![
            MonsterType { hp: 10, glyph: 'G', strength: 30 },  // goblin
            MonsterType { hp: 20, glyph: 'O', strength: 10 },  // orc
            MonsterType { hp: 15, glyph: 'E', strength: 15 },  // elf
        ];
        
        let mut rng = rand::rng();
    
        let mut return_vec = self.0.clone();
        for _ in 0..num {
            loop {
                let x = rng.random_range(..return_vec.len());
                let y = rng.random_range(..return_vec[0].len());
                let Some(mons_type) = monster_types.choose(&mut rng) else { panic!() };
                
                if return_vec[x][y] == Tile::Floor {
                    return_vec[x][y] = Tile::Monster(mons_type.clone());
                    break;
                }
            }
        }
        Map (return_vec)
    }
    
    fn attack(&self, attacker: (usize, usize), victum: (usize, usize), strength: u32) -> Vec<Vec<Tile>> {
        let mut return_vec = self.0.clone();
        
        let (att_x, att_y) = attacker;
        let (v_x, v_y) = victum;
        
        match return_vec[v_x][v_y] {
            Tile::Player(hp) => {
                if let Tile::Monster(monstertype) = return_vec[att_x][att_y] {
                    return_vec[v_x][v_y] = Tile::Player(
                        hp - strength as i32
                    );
                }
            },
            Tile::Monster(monstertype) => {
                if let Tile::Player(_) = return_vec[att_x][att_y] {
                    return_vec[v_x][v_y] = Tile::Monster(
                        MonsterType {
                            hp: monstertype.hp - entities::PLAYER_STRENGTH as i32,
                            glyph: monstertype.glyph,
                            strength: monstertype.strength,
                        }
                    )
                }
            },
            _ => {},
        }
        
        return_vec
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
        .add_monsters(10)
    
}