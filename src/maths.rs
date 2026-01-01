/// math tools to help with position, drawing rooms etc

/// access 1D map wit 2D cords
pub struct Cord {
    x: usize,
    y: usize,
}

impl Cord {
    fn new(x: usize, y: usize) -> Self {  // modify integer type
        Cord { x, y }
    }
    
    /// gets 1D index from the 2D index
    fn get_1d(&self) -> usize {
        self.x * CONSTANTS::MAP_WIDTH + self.y
    }
    
    /// get 2D index from flat index
    fn from_1d(i: usize) -> Self {
        let x = i % CONSTANTS::MAP_WIDTH;
        let y = i / CONSTANTS::MAP_WIDTH;
        
        Self { x, y }
    }
}

