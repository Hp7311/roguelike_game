use crate::entities::{Tile, MonsterType, MoveReturn};
use crate::entities;
use crate::constants;
use rand::Rng;
use rand::prelude::IndexedRandom;
use crossterm::cursor::{MoveToColumn, MoveTo};
use crossterm::execute;
use log::info;


#[derive(Debug, Clone, PartialEq)]
pub struct Map (pub Vec<Vec<Tile>>, pub String);


impl Map {

    pub fn draw(&self) {
        // prints current hp
        execute!(
            std::io::stdout(),
            MoveTo(0, 0)
        ).unwrap();
        
        match self.get_player_hp() {
            Some(urhp) => entities::print_hp(urhp),
            None       => entities::print_hp(0),
        }
        
        // prints gold amount
        println!("Your gold: {}", entities::get_gold());
        
        // prints map
        for inner in &self.0 {
            execute!(
                std::io::stdout(), MoveToColumn(0)
            ).unwrap();
            
            for tile in inner {
                let print_ch = match tile {
                    Tile::Wall => "# ".to_string(),
                    Tile::Player(_) => "@ ".to_string(),
                    Tile::Monster(monstype) => format!("{} ", monstype.glyph),
                    Tile::Floor => "- ".to_string(),
                };
                print!("{}", print_ch);
            }
            println!();
        }
        
        self.print_logs()
    }
    
    // these 3 functions mainly for main.rs use
    pub fn clear_log(&self) -> Self {
        Self (self.0.clone(), String::new())
    }
    
    pub fn print_logs(&self) {
        println!("{}", self.1)
    }
    
    pub fn add_to_log(&self, msg: &str) -> Self {
        let mut logs = self.1.clone();
        logs.push_str(msg);
    
        Self (self.0.clone(), logs)
    }
    
    fn get_player_hp(&self) -> Option<i32> {
    
        for inner in self.0.clone() {
            for tile in inner {
                if let Tile::Player(hp) = tile {
                    return Some(hp)
                }
            }
        }
        
        None
        
    }
    
    pub fn handle_monsters(&self) -> Self {
        let mut return_vec = self.0.clone();
        let mut return_log = self.1.clone();
        let mut monster_list = Vec::new();
        let mut player_pos = None;
        
        for (a, inner) in return_vec.iter().enumerate() {
            for (i, tile) in inner.iter().enumerate() {
                match tile {
                    Tile::Monster(monstertype) => {
                        monster_list.push(
                            ( (a, i), monstertype.clone() )
                        );
                    },
                    
                    Tile::Player(_) => player_pos = Some((a, i)),
                    _ => {},
                }
            }
        }
        
        info!("Made it to front of moves_monsters");
        return_vec = entities::moves_monsters(return_vec);
        
        for monster in monster_list {
            if let Some(player) = player_pos
            
                && entities::in_range(monster.0, player, 2) {
                
                    return_vec = Map (return_vec, return_log.clone()).attack(
                        monster.0, player
                    );
                    
                    if let Tile::Player(playerhp) = return_vec[player.0][player.1] {
                        if playerhp < 1 {
                            return_log.push_str(
                                &format!(
                                    "{} at {:?} obliterated you.\n",
                                    monster.1.name, monster.0
                                )
                            );
                            break;
                        } else {
                            return_log.push_str(
                                &format!(
                                    "{} at {:?} dealt {} damage to you.\n",
                                    monster.1.name, monster.0, monster.1.strength
                                )
                            );
                        }
                    }
                }
        }
        
        Self (return_vec, return_log)
    }
    
    pub fn move_player(&self, to: char) -> MoveReturn {
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
        
        if let Some((a, i)) = player_pos
            
            && let Tile::Player(player_hp) = return_vec[a][i] {
            
                match to {
        
                    'w' => {
                        if a != 0 && return_vec[a-1][i] == Tile::Floor {
                            return_vec[a][i] = Tile::Floor;
                            return_vec[a-1][i] = Tile::Player(player_hp);
                        } else {
                            return MoveReturn::Failure;
                        }
                    },
            
                    'a' => {
                        if i != 0 && return_vec[a][i-1] == Tile::Floor {
                            return_vec[a][i] = Tile::Floor;
                            return_vec[a][i-1] = Tile::Player(player_hp);
                        } else {
                            return MoveReturn::Failure;
                        }
                    },
            
                    's' => {
                        if a != return_vec.len()-1 && return_vec[a+1][i] == Tile::Floor {
                            return_vec[a][i] = Tile::Floor;
                            return_vec[a+1][i] = Tile::Player(player_hp);
                        } else {
                            return MoveReturn::Failure;
                        }
                    },
            
                    'd' => {
                        if i != return_vec[0].len()-1 && return_vec[a][i+1] == Tile::Floor {
                            return_vec[a][i] = Tile::Floor;
                            return_vec[a][i+1] = Tile::Player(player_hp);
                        } else {
                            return MoveReturn::Failure;
                        }
                    },
            
                    _ => {},
                }
            }
        
        MoveReturn::Success( Self (return_vec, self.1.clone()) )
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
                    monster_list.push(
                        (a, i)
                    );
                }
            }
        }
        
        let mut return_log = self.1.clone();
        
        if let Some(player) = player_pos {
            for monster in monster_list {
                if entities::in_range(player, monster, 3) {
                    return_vec = Map (return_vec, return_log.clone()).attack(
                        player,
                        monster,
                    );
                    let (a, i) = monster;
                    
                    if let Tile::Monster(mtype) = &return_vec[a][i] {
                    
                        if mtype.hp < 1 {
                            return_log.push_str(
                                &format!(
                                    "You obliterated {} at {:?}!\n",
                                    mtype.name, monster
                                )
                            );
                            entities::add_to_gold(mtype.gold).unwrap();
                            
                        } else {
                            return_log.push_str(
                                &format!(
                                    "You attacked {} at {:?} of {} damage!\n",
                                    mtype.name, monster, mtype.player_strength_to
                                )
                            );
                        }
                    }
                    
                }
            }
        }
        
        Self (return_vec, return_log)
    }
    
    pub fn delete_dead(&self) -> Self {
        let mut return_vec = self.0.clone();
        let return_log = self.1.clone();
        
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
        
        Self (return_vec, return_log)
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
    
    pub fn monsters_exists(&self) -> bool {
        let mut monsters_exists = false;
        
        for inner in &self.0 {
            for tile in inner {
                if let Tile::Monster(_) = tile {
                    monsters_exists = true;
                    break;
                }
            }
        }
        
        monsters_exists
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
        
        Self (return_vec, self.1.clone())
    }
    
    fn add_player(&self) -> Self {
        let mut return_vec = self.0.clone();
        let x = rand::rng().random_range(..return_vec.len());
        let y = rand::rng().random_range(..return_vec[0].len());
        return_vec[x][y] = Tile::Player(constants::PLAYER_HEALTH as i32);
        
        Self (return_vec, self.1.clone())
    }
    
    fn add_monsters(&self, num: u32) -> Self {
    
        let monster_types = entities::get_monsters();
        
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
        Self (return_vec, self.1.clone())
    }
    
    fn attack(&self, attacker: (usize, usize), victum: (usize, usize)) -> Vec<Vec<Tile>> {
        let mut return_vec = self.0.clone();
        
        let (att_x, att_y) = attacker;
        let (v_x, v_y) = victum;
        
        match &return_vec[v_x][v_y] {
        
            Tile::Player(hp) => {
                if let Tile::Monster(monstertype) = &return_vec[att_x][att_y] {
                    return_vec[v_x][v_y] = Tile::Player(
                        hp - monstertype.strength as i32
                    );
                }
            },
            
            Tile::Monster(monstertype) => {
            
                return_vec[v_x][v_y] = Tile::Monster (
                    MonsterType {
                        gold:               monstertype.gold,
                        player_strength_to: monstertype.player_strength_to,
                        hp:                 monstertype.hp - monstertype.player_strength_to as i32,
                        glyph:              monstertype.glyph,
                        strength:           monstertype.strength,
                        name:               monstertype.name.clone(),
                    }
                );
                
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
    
    let length = constants::LENGTH;
    let width  = constants::WIDTH;
    let mut return_vec = Vec::new();
    
    for inner_vec in 0..width {
        return_vec.push(Vec::new());
        
        for _ in 0..length {
            return_vec[inner_vec as usize].push(Tile::Wall);
        }
    }
    
    // validate if map can be completed
    loop {
        let return_map = Map (return_vec.clone(), String::new())
            .dig_floors()
            .add_player()
            .add_monsters(constants::MONSTER_NUMBER);
        if entities::check_map_valid(return_map.clone()) {
            return return_map;
        }
    }
    
    
}
