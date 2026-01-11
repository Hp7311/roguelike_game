/// handles monsters and players attacking
use crate::state::State;
use crate::constants::{ATTACK_RANGE, PLAYER_STRENGTH};
use crate::gold::add_to_gold;


pub fn handle_entities(state: &mut State) {
    
    // player first, monster second. Same range
    
    for monster in state.monsters.as_mut().unwrap() {

        // if in range
        if state.player.as_ref().unwrap().pos.in_range(&monster.pos, ATTACK_RANGE) {

            // player
            monster.info.hp -= PLAYER_STRENGTH;
            if monster.info.hp <= 0 {
                add_to_gold(monster.info.hp.try_into().unwrap()).unwrap();
                state.logs.add_to_log(&format!(
                    "You obliterated {} at {}", monster.info.name, monster.pos
                ));
            }
            else {
                state.logs.add_to_log(&format!(
                    "You dealt {} damage to {} at {}", PLAYER_STRENGTH, monster.info.name, monster.pos
                ));
            }

            // monster
            if monster.info.hp > 0 {
                state.player.as_mut().unwrap().hp -= monster.info.strength as i32;
                state.logs.add_to_log(&format!(
                    "{} at {} dealt {} damage to you", monster.info.name, monster.pos, monster.info.strength
                ));
            }
        }
    }
}
