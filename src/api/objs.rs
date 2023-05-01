use serde::{Serialize, Deserialize};

use crate::api::Direction;

pub trait ActionInfo {
    fn action(&self) -> String;

    fn executed(&self) -> bool;
}

/// Gives basic information about a game
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ApiError {
    #[serde(alias = "description")]
    pub desc : String,
    pub error : bool
}

impl ApiError {
    pub fn new(desc : String) -> Self {
        Self {
            desc,
            error: true
        }
    }
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.desc)
    }
}

impl std::error::Error for ApiError { }

/// Gives basic information about a game
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GameInfo {
    pub gameid: String,
    pub running: bool, 
    pub level: String
}

// Movement
    /// Gives basic information about a movement command sent
    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct MoveInfo {
        action : String,
        executed : bool,
        #[serde(alias = "move")]
        pub moved : bool
    }

    impl ActionInfo for MoveInfo {
        fn action(&self) -> String {
            self.action.clone()
        }

        fn executed(&self) -> bool {
            self.executed
        }
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct DashInfo {
        action : String,
        executed : bool,
        #[serde(alias = "blocksDashed")]
        pub dist : u32,
        #[serde(alias = "damageTaken")]
        pub damage : f32
    }

    impl ActionInfo for DashInfo {
        fn action(&self) -> String {
            self.action.clone()
        }

        fn executed(&self) -> bool {
            self.executed
        }
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct TeleportInfo {
        action : String,
        executed : bool,
        #[serde(alias = "landedInWall")]
        pub in_wall : bool
    }

    impl ActionInfo for TeleportInfo {
        fn action(&self) -> String {
            self.action.clone()
        }

        fn executed(&self) -> bool {
            self.executed
        }
    }
// 

// Detection
    /// Actual results of the radar
    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct RadarResult {
        pub north : u16,
        pub east : u16, 
        pub south : u16, 
        pub west : u16,
        pub sameblock : u16
    }

    impl RadarResult {
        pub fn best_move_dir(&self) -> Direction {
            let mut max_dir : Direction = Direction::North;
            let mut max_dir_val = 0;

            if self.north > max_dir_val {
                max_dir = Direction::North;
                max_dir_val = self.north;
            }

            if self.east > max_dir_val {
                max_dir = Direction::East;
                max_dir_val = self.east;
            }

            if self.south > max_dir_val {
                max_dir = Direction::South;
                max_dir_val = self.south;
            }

            if self.west > max_dir_val {
                max_dir = Direction::West;
            }

            max_dir
        } 
    }

    /// Gives basic information about a radar command sent
    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct RadarInfo {
        action : String,
        executed : bool,

        #[serde(alias = "radarResults")]
        pub results : RadarResult
    }

    impl RadarInfo {
        pub fn best_move_dir(&self) -> Direction {
            self.results.best_move_dir()
        } 
    }

    impl ActionInfo for RadarInfo {
        fn action(&self) -> String {
            self.action.clone()
        }

        fn executed(&self) -> bool {
            self.executed
        }
    }

    /// Gives basic information about a radar command sent
    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct ScanInfo {
        action : String,
        executed : bool,

        #[serde(alias = "differenceToNearestPlayer")]
        pub nearest : crate::OptPos
    }

    impl ActionInfo for ScanInfo {
        fn action(&self) -> String {
            self.action.clone()
        }

        fn executed(&self) -> bool {
            self.executed
        }
    }

    /// Gives basic information about a radar command sent
    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct PeakInfo {
        action : String,
        executed : bool,

        pub players : u16,
        pub dist : Option<i32>
    }

    impl ActionInfo for PeakInfo {
        fn action(&self) -> String {
            self.action.clone()
        }

        fn executed(&self) -> bool {
            self.executed
        }
    }
// 

// Hit
    /// Gives basic information about a radar command sent
    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct HitInfo {
        action : String,
        executed : bool,

        pub critical : u32,
        pub hit : u32
    }

    impl ActionInfo for HitInfo  {
        fn action(&self) -> String {
            self.action.clone()
        }

        fn executed(&self) -> bool {
            self.executed
        }
    }

    /// Gives basic information about a radar command sent
    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct ShootInfo {
        action : String,
        executed : bool,

        #[serde(alias = "hitSomeone")]
        pub hit : bool
    }

    impl ActionInfo for ShootInfo {
        fn action(&self) -> String {
            self.action.clone()
        }

        fn executed(&self) -> bool {
            self.executed
        }
    }

    /// Gives basic information about a radar command sent
    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct UltInfo {
        action : String,
        executed : bool,

        #[serde(alias = "hitcount")]
        pub hits : u32
    }

    impl ActionInfo for UltInfo {
        fn action(&self) -> String {
            self.action.clone()
        }

        fn executed(&self) -> bool {
            self.executed
        }
    }
// 

/// Kills and deaths stats for a player
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct PerfStats {
    pub deaths : u32,
    pub kills : u32
}

/// Stats about the current level 
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct LevelStats {
    #[serde(alias = "deathsleft")]
    pub max_deaths : u32, 

    #[serde(alias = "levelid")]
    pub id : u32, 

    pub name : String,
    pub progress : f32,

    #[serde(alias = "remainingtime")]
    pub t_rem : f32
}

/// Stats about the player health 
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct HealthStats {
    #[serde(alias = "currenthealth")]
    pub current : f32,
    #[serde(alias = "maxhealth")]
    pub max : f32
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct StatsInfo {
    action : String,
    executed : bool,

    #[serde(alias = "stats")]
    pub perf : PerfStats,
    pub level : LevelStats, 
    pub health : HealthStats
}

impl ActionInfo for StatsInfo {
    fn action(&self) -> String {
        self.action.clone()
    }

    fn executed(&self) -> bool {
        self.executed
    }
}

