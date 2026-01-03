/// provides Logs struct for logging actions in game

pub struct Logs {
    msg: String,
}

impl Logs {
    pub fn new() -> Self {
        Self {
            msg: String::new(),
        }
    }
    
    pub fn add_to_log(&mut self, msg: &str) {
        self.msg.push_str(msg);
        self.msg.push_str("\n");
    }
    
    pub fn render(&self) {
        println!("{}", self.msg);
    }
}