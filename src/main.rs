use crossterm::event::{read, Event, KeyCode};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};
use crossterm::execute;
use crossterm::terminal;
use crossterm::cursor::MoveTo;
use std::io::stdout;

mod map;
mod entities;
mod constants;

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
                        gs.map = gs.map.clear_log()
                            .move_player(key);
                            
                        gs.runstate = RunState::PlayerTurn;
                    },
                    'q' => { break },
                    'r' => gs.runstate = RunState::PreRun,
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
                    .add_to_log("You died!");
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
        
        if !gs.map.player_exists() {
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

// TODO disable dealing damage to monsters when -> Wall
// TODO add monsters AI 
// TODO add monster name to log messages
// TODO decide whether a version of map can be completed

// TODO gold system
// TODO increasing difficulty of levels
