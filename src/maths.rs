/// math tools to help with position, drawing rooms etc
use crate::map::Map;
use crate::CONSTANTS{MAP_WIDTH, MAP_HEIGHT}

/// access 1D map wit 2D cords
pub struct Cord {
    x: usize,
    y: usize,
}

impl Cord {
    pub fn new(x: usize, y: usize) -> Self {  // modify integer type
        Cord { x, y }
    }
    
    /// gets 1D index from the 2D index
    pub fn get_1d(&self) -> usize {
        self.x * MAP_WIDTH + self.y
    }
    
    /// get 2D index from flat index
    pub fn from_1d(i: usize) -> Self {
        let x = i % MAP_WIDTH;
        let y = i / MAP_WIDTH;
        
        Self { x, y }
    }
}


/// a Rectangle from start -> down/right
pub struct Rect {
    start: Cord,
    length: usize,
    width: usize,
}

impl Rect {
    pub fn new(start: Cord, length: usize, width: usize) -> Self {
        Self {
            start,
            length,
            width,
        }
    }
    
    /// checks if self overlaps with another Rect
    // PS maybe more lightweight method?
    pub fn overlaps_with(&self, other: Self) -> bool {
        let self_start = self.start;
        let self_points = self.get_all_pixels();
        
        let other_start = other.start;
        let other_points = other.get_all_pixels();
        
        
        let overlap_points = self_points.iter()
            .any(|&point| {
                other_points.iter()
                    .any(|&other_point| if other_point == point )
            });
            
            
        // check if any overlap_points
        if overlap_points {
            false
        } else {
            true
        }
    }
    
    
    /// whether self can fit in the Map
    pub fn can_fit(&self, target: &Map) -> bool {
    
        let start: Cord = self.start;
        let right_lower: Cord = Cord::new(
            start.x + self.width,
            start.y + self.length
        );
        
        // PS this relies on map passed to this function being correctly built from CONSTANTS
        if start.x >= MAP_WIDTH || start.y >= MAP_LENGTH {
            return false;
        }
        if right_lower.x >= MAP_WIDTH || right_lower.y >= MAP_LENGTH {
            return false;
        }
        
        true
    }
    
    /// returns Vec of all points in the rect
    pub fn get_all_pixels(&self) -> Vec<Cord> {
        let mut all_pixels = Vec::new();
        
        for x in self.start.x..(self.start.x + self.width) {
            for y in self.start.y..(self.start.y + self.length) {
                all_pixels.push( Cord::new(x, y) );
            }
        }
        
        all_pixels
    }
}