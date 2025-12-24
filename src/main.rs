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
    
    fn get_death_screen(&self) {
        self.map.get_player_stats()
    }
}

fn main() -> Result<(), std::io::Error> {

    // --- INITIALIZATION --
    enable_raw_mode()?;
    
    let mut gs = State {
        map: map::init_map(),
        runstate: RunState::PreRun,
    };

    // --- CORE GAME LOOP ---
    loop {
        match gs.runstate {
            RunState::PreRun => {
                gs.runstate = RunState::AwaitingInput;
            },
            RunState::AwaitingInput => {
                execute!(
                    stdout(),
                    MoveTo(0, 11)
                );
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
                gs.get_death_screen();
                
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
            gs.runstate = RunState::GameOver
        }

        // --- THE RENDER STEP ---
        gs.render();
    }
    disable_raw_mode()?;
    
    Ok(())
}
