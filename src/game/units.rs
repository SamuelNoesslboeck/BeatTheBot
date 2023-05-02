use serde::{Serialize, Deserialize};

/// Represents a relative position to an object in X and Y 
#[derive(Clone, Copy, Debug, Serialize, Deserialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct RelPos(pub i32, pub i32);

/// Represents the absolute world position in X and Y
#[derive(Clone, Copy, Debug, Serialize, Deserialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct AbsPos(pub i32, pub i32);

/// Represents a position to an object in X and Y that can be undefined in some coordinates
#[derive(Clone, Copy, Debug, Serialize, Deserialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct OptPos(pub Option<i32>, pub Option<i32>);

impl OptPos {
    pub fn try_conv(&self) -> Option<RelPos> {
        if let Some(x) = self.0 {
            if let Some(y) = self.1 {
                return Some(RelPos(x, y));
            }
        }

        None
    }
}