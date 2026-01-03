/// handles monsters and players attacking
use crate::state::State;
use crate::CONSTANTS::{ATTACK_RANGE, PLAYER_STRENGTH};


pub fn handle_entities(state: &State) -> State {
    
    let mut ret = state;
    
    // DESIGN player first, monster second. Same range
    // if in range, attack
    ret.monsters.unwrap().iter()
        .filter(|&monster| {
            ret.player.unwrap().pos
                .in_range(&monster.pos, ATTACK_RANGE)
        })
        .map(|&monster| {
            // player
            monster.info.hp -= PLAYER_STRENGTH;
            if monster.info.hp <= 0 {
                ret.logs.add_to_log(&format!(
                    "You obliterated {} at {}", monster.info.name, monster.pos
                ));
            }
            else {
                ret.logs.add_to_log(&format!(
                    "You dealt {} damage to {} at {}", PLAYER_STRENGTH, monster.info.name, monster.pos
                ));
            }
            
            // monster
            if monster.info.hp > 0 {
                ret.player.unwrap().hp -= monster.info.strength;
                ret.logs.add_to_log(&format!(
                    "{} at {} dealt {} damage to you", monster.info.name, monster.name, monster.info.strength
                ))
            }
        })
        .collect();
    
    *ret  // trying deref to pass expected State found &State
}
