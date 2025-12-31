/// core struct State used by main.rs

use std::io::stdout;
use crossterm::{
    terminal::{
        Clear, ClearType, enable_raw_mode, disable_raw_mode
    },
    execute,
};


#[derive(Debug, PartialEq)]
pub enum RunState {
    PreRun,
    AwaitingInput,
    PlayerTurn,
    MonsterTurn,
    GameLost,
    GameWon,
}

use RunState::*;

#[derive(Debug)]
pub struct State {
    map: map::Map,
    logs: logs::Log,
    runstate: RunState,
}


impl State {
    
    /// creates a new State instance
    pub fn new() -> Self {
        Self {
            map: map::Map::init(),
            logs: logs::Logs::init(),
            runstate: PreRun,
        }
    }
    
    /// called from main.rs to run the entire project
    pub fn run(&self) -> anyhow::Result<()>{
        loop {
            match self.RunState {
                
            }
            
            self.finalise();  // delete dead, check win etc.
            self.render();
        }
    }
    
    fn render(&self) {
        execute!(
            stdout(),
            Clear(ClearType::All),
        ).unwrap();
        
        self.map.draw();
        self.logs.draw();
    }

    fn finalise(&mut self) {
        self.map.delete_dead();
        self.map.check_game();
    }
}

fn get_keystroke() -> char {
    enable_raw_mode().unwrap();
    
    let key: char;
    loop {
        if let Event::Key(event) = read().unwrap() {
            match event.code {
                KeyCode::Up => {
                    key = 'w';
                    break;
                },
                KeyCode::Down => {
                    key = 's';
                    break;
                },
                KeyCode::Left => {
                    key = 'a';
                    break;
                },
                KeyCode::Right => {
                    key = 'd';
                    break;
                },
                KeyCode::Char(c) if matches!(c, 'w' | 's' | 'a' | 'd' | 'q' | 'r') => {
                    key = c;
                    break;
                },
                _ => {},
            }
        }
    }
    
    disable_raw_mode().unwrap();
    
    key
}

