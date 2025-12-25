use crossterm::event::{read, Event, KeyCode};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};
use crossterm::execute;
use crossterm::terminal;
use crossterm::cursor::MoveTo;
use std::io::stdout;

mod map;
mod entities;
mod constants;

use entities::MoveReturn;

#[derive(Debug, PartialEq)]
enum RunState {
    PreRun,
    AwaitingInput,
    PlayerTurn,
    MonsterTurn,
    GameOver,
    GameWon,
}

#[derive(Debug)]
struct State {
    // Map (Vec<Vec<Tile>>)
    // Tile enum: Wall, Floor, Player, Monster
    map: map::Map,
    
    runstate: RunState,
}

impl State {
    fn render(&self) {
        execute!(
            stdout(),
            terminal::Clear(terminal::ClearType::All),
        ).unwrap();
        
        self.map.draw();
    }

    fn run_monsters(&mut self) {
        self.map = self.map.handle_monsters();
    }
    
    fn run_player(&mut self) {
        self.map = self.map.handle_player()
    }
}

fn get_keystroke() -> char {
    let key: char;
    loop {
        if let Event::Key(event) = read().unwrap() {
            if let KeyCode::Char(c) = event.code {
                key = c;
                break;
            }
        }
    }
    
    key
}

fn main() -> Result<(), std::io::Error> {
    
    let mut gs = State {
        map: map::init_map(),
        runstate: RunState::PreRun,
    };

    
    loop {
        
        match gs.runstate {
        
            RunState::PreRun => {
                // re-initialise map
                gs = State {
                    map: map::init_map(),
                    runstate: RunState::AwaitingInput,
                };
            },
            
            RunState::AwaitingInput => {
                execute!(
                    stdout(),
                    MoveTo(0, 11)
                ).unwrap();
                
                let key = get_keystroke();
                match key {
                    'w' | 'a' | 's' | 'd' => {
                        gs.map = gs.map.clear_log();
                        match gs.map.move_player(key) {
                            MoveReturn::Failure => gs.runstate = RunState::MonsterTurn,
                            MoveReturn::Success(themap) => {
                                gs.map = themap;
                                gs.runstate = RunState::PlayerTurn;
                            },
                        }  // fixes dealing damage when bumping against Wall BUT don't let PlayerTurn do more
                    },
                    
                    'q' => { break },
                    'r' => gs.runstate = RunState::PreRun,  // TODO delete when not debuggin
                    _ => {},
                }
            },
            
            RunState::PlayerTurn => {
                gs.run_player();
                gs.runstate = RunState::MonsterTurn;
            },
            
            RunState::MonsterTurn => {
                gs.run_monsters();
                gs.runstate = RunState::AwaitingInput;
            },
            
            RunState::GameOver => {
                gs.map = gs.map
                    .clear_log()
                    .add_to_log("You died!\nq to quit, r to restart.");
                disable_raw_mode()?;
                gs.map.print_logs();
                enable_raw_mode()?;
                
                match get_keystroke() {
                    'q' => { break },
                    'r' => gs.runstate = RunState::PreRun,
                    _ => {},
                }
                
            },
            
            RunState::GameWon => {
                gs.map = gs.map
                    .clear_log()
                    .add_to_log("You won the game!\nr for restart, q if quit.");
                    
                disable_raw_mode()?;
                gs.map.print_logs();
                enable_raw_mode()?;
                
                match get_keystroke() {
                    'q' => { break },
                    'r' => gs.runstate = RunState::PreRun,
                    _ => {},
                }
            },
        }
        
        gs.map = gs.map.delete_dead();
        
        
        if !gs.map.player_exists() && !(gs.runstate == RunState::PreRun) {
            gs.runstate = RunState::GameOver;
        }
        
        if !gs.map.monsters_exists() && gs.runstate == RunState::AwaitingInput {
            gs.runstate = RunState::GameWon;
        }

        
        disable_raw_mode()?;
        gs.render();  // prints log here
        enable_raw_mode()?;
    }
    
    disable_raw_mode()?;
    Ok(())
}

// SOLVED disable dealing damage to monsters when -> Wall
// TODO add monsters AI 
// TODO add monster name to log messages
// TODO decide whether a version of map can be completed

// TODO gold system
// TODO increasing difficulty of levels
// remember to disable_raw_mode before rendering anything.
// remember to change how to avoid dealing damage to monsters when changing PlayerTurn