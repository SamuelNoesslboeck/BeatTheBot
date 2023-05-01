use crate::AbsPos;

#[derive(Clone, Debug)]
pub struct Map {
    pub size : AbsPos,

    pub kills_goal : Option<u32>,
    pub max_time : f32
}

impl Map {
    pub const DEFAULT_LEVEL : Self = Self {
        size: AbsPos(15, 15),
        kills_goal: Some(20),
        max_time: 900.0
    };

    pub const BIGGER_MAP_LEVEL : Self = Self {
        size: AbsPos(25, 25),
        kills_goal: None,
        max_time: 900.0
    }; 

    pub const MORE_BOTS_LEVEL : Self = Self {
        size: AbsPos(25, 25),
        kills_goal: Some(10),
        max_time: 900.0
    }; 

    pub const NEW_BOTS_LEVEL : Self = Self {
        size: AbsPos(15, 15),
        kills_goal: None, 
        max_time: 900.0
    };
}

impl Default for Map {
    fn default() -> Self {
        Map::DEFAULT_LEVEL
    }
}