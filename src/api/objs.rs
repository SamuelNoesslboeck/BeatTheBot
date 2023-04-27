use serde::{Serialize, Deserialize};

pub trait Action {
    fn action(&self) -> String;

    fn executed(&self) -> bool;
}

/// Gives basic information about a game
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GameInfo {
    pub gameid: String,
    pub running: bool, 
    pub level: String
}

/// Gives basic information about a movement command sent
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MoveInfo {
    action : String,
    executed : bool,
    pub r#move : bool
}

/// Actual results of the radar
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RadarResult {
    pub north : u32,
    pub east : u32, 
    pub south : u32, 
    pub west : u32,
    pub sameblock : u32
}

/// Gives basic information about a radar command sent
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RadarInfo {
    action : String,
    executed : bool,

    #[serde(alias = "radarResults")]
    pub results : RadarResult
}