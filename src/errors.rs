/// error enums for unified access
use thiserror::Error;
use crate::maths::{Cord, Rect};

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
pub enum ValidateError {
    #[error("Map invalid: {0}")]
    MapErr(String),
    #[error("Room isolated: {0}")]
    RoomIsolatedError(Rect),
    #[error("Player invalid at {1}: {0}")]
    PlayerErr(String, Cord),
    #[error("Monster invalid at {1}: {0}")]
    MonsterErr(String, Cord),
}
/* TODOs: 
add FOV
health bar
fix map width != length issue
err: 
occasional isolated room
occasional validation false negative
*/