/// provides Logs struct for logging actions in game

pub struct Logs {
    msg: String,
}

impl Logs {
    pub fn new() -> Self {
        Logs {
            msg: String::new(),
        }
    }
    
}