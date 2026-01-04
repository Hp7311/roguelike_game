/// handles monsters and players attacking
use crate::state::State;
use crate::CONSTANTS::{ATTACK_RANGE, PLAYER_STRENGTH};


pub fn handle_entities(state: &mut State) {
    
    // DESIGN player first, monster second. Same range
    // if in range, attack
    for monster in state.monsters.iter_mut() {

        if state.player.pos.in_range(&monster.pos, ATTACK_RANGE) {
            // player
            monster.info.hp -= PLAYER_STRENGTH;
            if monster.info.hp <= 0 {
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
                state.player.hp -= monster.info.strength as i32;
                state.logs.add_to_log(&format!(
                    "{} at {} dealt {} damage to you", monster.info.name, monster.pos, monster.info.strength
                ));
            }
        }
    }
}
