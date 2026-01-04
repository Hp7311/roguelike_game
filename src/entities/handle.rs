/// handles monsters and players attacking
use crate::state::State;
use crate::CONSTANTS::{ATTACK_RANGE, PLAYER_STRENGTH};


pub fn handle_entities(state: &State) -> State {
    
    let mut ret = state;
    
    // DESIGN player first, monster second. Same range
    // if in range, attack
    for monster in ret.monsters.iter_mut() {

        if player.pos.in_range(&monster.pos, ATTACK_RANGE) {
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
                ret.player.hp -= monster.info.strength as i32;
                ret.logs.add_to_log(&format!(
                    "{} at {} dealt {} damage to you", monster.info.name, monster.pos, monster.info.strength
                ));
            }
        }
    }
    
    *ret
}
