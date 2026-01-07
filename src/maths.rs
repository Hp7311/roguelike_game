/// math tools to help with position, drawing rooms etc
use crate::map::Map;
use crate::CONSTANTS::{MAP_WIDTH, MAP_LENGTH};

/// access 1D map wit 2D cords
#[derive(PartialEq, Clone, Debug)]
pub struct Cord {
    pub x: usize,
    pub y: usize,
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
    
    /// check if two Cords in the specifed range
    pub fn in_range(&self, other: &Self, range: u32) -> bool {
        let range = range as usize;
        
        if self.x.abs_diff(other.x) <= range
            && self.y.abs_diff(other.y) <= range {
            
            return true
        }
        
        false
    }
}

impl std::fmt::Display for Cord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
impl std::fmt::Display for Rect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Start at {}, down {}, right {}", self.start, self.width, self.length)
    }
}

#[derive(Clone, Debug)]
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
    pub fn overlaps_with(&self, other: &Self) -> bool {

        let self_points = self.get_all_pixels();
        let other_points = other.get_all_pixels();
        
        
        self_points.iter()
            .any(|point| {
                other_points.iter()
                    .any(|other_point| other_point == point )
            })
    }
    
    
    /// whether self can fit in the Map
    pub fn can_fit(&self) -> bool {
    
        let start: Cord = self.start.clone();
        let right_lower: Cord = Cord::new(
            start.x + self.width - 1,
            start.y + self.length - 1,
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
        
        for x in self.start.x..=(self.start.x + self.width - 1) {
            for y in self.start.y..=(self.start.y + self.length - 1) {
                all_pixels.push( Cord::new(x, y) );
            }
        }
        
        all_pixels
    }
    
    /// returns center point of the rect
    pub fn get_center(&self) -> Cord {
        let x = self.start.x + self.width / 2;  // "/" gets the integer result
        let y = self.start.y + self.length / 2;
        
        Cord::new(x, y)
    }
}

impl From<(usize, usize)> for Cord {
    fn from(v: (usize, usize)) -> Self {
        Self::new(v.0, v.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_overlap() {
        /// test to see if overlap() working
        let rect1 = Rect::new(
            Cord::new(2, 2), 3, 4
        );
        let rect2 = Rect::new(
            Cord::new(2, 2), 2, 3
        );
        assert_eq!(rect1.overlaps_with(&rect2), true);
    }
    #[test]
    fn test_all_pixels() {
        /// see if all_pixels working
        let rect = Rect::new(
            Cord::new(3, 2), 1, 2
        );
        let expected = vec![
            Cord::from((3, 2)),
            Cord::from((4, 2)),
        ];
        assert_eq!(rect.get_all_pixels(), expected);
    }
    #[test]
    fn test_center() {
        let rect = Rect::new(Cord::new(0, 0), 4, 4);

        assert_eq!(rect.get_center(), Cord::new(2, 2));
    }
}