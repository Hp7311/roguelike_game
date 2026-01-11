/// error enums for unified access
use thiserror::Error;
use crate::maths::Cord;

/// map::dig_map
#[derive(Debug, Error)]
pub enum BuildError {
    #[error("Two rooms with same center: {0}")]
    RoomCenterSame(String),

}

#[derive(Debug, Error)]
pub enum SpawnError {
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("{0} at {1}")]
    InvalidPlace(String, Cord),
}

#[derive(Debug, Error)]
pub enum OutOfBoundError {
    #[error("{0}")]
    OutOfBound(String),
    
}