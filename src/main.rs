use crossterm::event::{read, Event, KeyCode};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};
use crossterm::execute;
use crossterm::terminal;
use crossterm::cursor::MoveTo;
use std::io::stdout;

mod map;
mod entities;
mod constants;

#[derive(Debug)]
enum RunState {
    PreRun,
    AwaitingInput,
    PlayerTurn,
    MonsterTurn,
    GameOver,
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
        // TODO add drawing player stats
    }

    fn run_monsters(&mut self) {
        self.map = self.map.handle_monsters();
    }
    
    fn run_player(&mut self) {
        self.map = self.map.handle_player()
    }
}

fn main() -> Result<(), std::io::Error> {

    // --- INITIALIZATION ----
    
    let mut gs = State {
        map: map::init_map(),
        runstate: RunState::PreRun,
    };

    // --- CORE GAME LOOP ---
    loop {
        enable_raw_mode()?;
        match gs.runstate {
            RunState::PreRun => {
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
                let mut key = 'a';
                loop {
                    if let Event::Key(event) = read()? {
                        if let KeyCode::Char(c) = event.code {
                            key = c;
                            break;
                        }
                    }
                }
                if key == 'q' { break }
                match key {
                    'w' | 'a' | 's' | 'd' => {
                        gs.map = gs.map.clear_log();
                        gs.map = gs.map.move_player(key);
                        gs.runstate = RunState::PlayerTurn;
                    },
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
                println!("You died!");
                
                let mut key = 'a';
                loop {
                    if let Event::Key(event) = read()? {
                        if let KeyCode::Char(c) = event.code {
                            key = c;
                            
                        }
                    }
                    match key {
                        'q' => break,
                        'r' => gs.runstate = RunState::PreRun,
                        _ => {},
                    }
                }
                
            },
        }
        
        gs.map = gs.map.delete_dead();
        
        if !gs.map.player_exists() {
            gs.runstate = RunState::GameOver;
        }
        
        if !gs.map.monsters_exists() {
            gs.runstate = RunState::PreRun;
        }

        // --- THE RENDER STEP ---
        disable_raw_mode()?;
        gs.render();
    }
    disable_raw_mode()?;
    Ok(())
}

// TODO add monsters AI 
// TODO add monster name to log messages
// TODO decide whether a version of map can be completed

// TODO gold system
// TODO increasing difficulty of levels
