use serde::{Serialize, Deserialize};

/// Represents a relative position to an object in X and Y 
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct RelPos(pub i32, pub i32);

/// Represents the absolute world position in X and Y
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct AbsPos(pub i32, pub i32);

/// Represents a position to an object in X and Y that can be undefined in some coordinates
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct OptPos(pub Option<i32>, pub Option<i32>);