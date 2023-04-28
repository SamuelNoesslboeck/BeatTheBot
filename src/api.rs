use reqwest::{blocking::Client, StatusCode, Method};
use serde::de::DeserializeOwned;

mod objs;
pub use objs::*;

const ADDRESS : &str = "https://game-dd.countit.at/api";

#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West
}

impl ToString for Direction {
    fn to_string(&self) -> String {
        (*self as u64).to_string()
    }
}

pub struct Api {
    token : String,
    client : Client
}

impl Api {
    pub fn new(token : String) -> Self {
        Self {
            token: token,
            client: Client::new()
        }
    }

    // Requests
        /// Sends a general post request to the game-API
        #[inline(always)]
        fn request<T : DeserializeOwned>(&self, method : Method, url : &str) -> Result<T, crate::Error> {
            let req;
            
            if method == Method::GET {
                req = self.client.get(url)
                    .body("")
                    .send();
            } else {
                req = self.client.post(url)
                    .body("")
                    .send();
            }
            
            match req {
                Ok(res) => {
                    // Parse API-Error
                    if res.status() == StatusCode::BAD_REQUEST {
                        Err(Box::new(res.json::<ApiError>()?))
                    } else {
                        Ok(res.json::<T>()?)
                    }
                },
                Err(err) => { 
                    #[cfg(not(feature = "high-dbg"))]
                    let res = Err(Box::from(err));
                    
                    #[cfg(feature = "high-dbg")]
                    let res = Err(Box::from(dbg!(err)));


                    res
                }
            } 
        }

        /// Sends a game-specific get request to the game-API
        #[inline(always)]
        fn get_req_game<T : DeserializeOwned>(&self, path : &str) -> Result<T, crate::Error> {
            self.request(Method::GET, format!("{}{}/{}{}", ADDRESS, "/game", self.token, path).as_str())
        }

        /// Sends a game-specific get request to the game-API
        #[inline(always)]
        fn get_req_player<T : DeserializeOwned>(&self, path : &str) -> Result<T, crate::Error> {
            self.request(Method::GET, format!("{}{}/{}{}", ADDRESS, "/player", self.token, path).as_str())
        } 

        /// Sends a game-specific post request to the game-API
        #[inline(always)]
        fn post_req_game<T : DeserializeOwned>(&self, path : &str) -> Result<T, crate::Error> {
            self.request(Method::POST, format!("{}{}/{}{}", ADDRESS, "/game", self.token, path).as_str())
        }

        /// Sends a game-specific post request to the game-API
        #[inline(always)]
        fn post_req_player<T : DeserializeOwned>(&self, path : &str) -> Result<T, crate::Error> {
            self.request(Method::POST, format!("{}{}/{}{}", ADDRESS, "/player", self.token, path).as_str())
        }
    // 

    pub fn game_create(&mut self) -> Result<Option<GameInfo>, crate::Error> {
        match self.post_req_game::<GameInfo>("/create") {
            Ok(res) =>  { 
                Ok(Some(res))
            },
            Err(err) => {
                if err.to_string().starts_with("You already own a running game") {
                    Ok(None)
                } else {
                    Err(err)
                }
            }
        }
    }
    
    pub fn game_close(&mut self) -> Result<Option<GameInfo>, crate::Error> {
        match self.post_req_game::<GameInfo>("/close") {
            Ok(res) => {
                Ok(Some(res))
            },
            Err(err) => {
                if err.to_string().starts_with("There is no game which could be closed") {
                    Ok(None)
                } else {
                    Err(err)
                }
            }
        }
    }

    pub fn game_status(&mut self) -> Result<GameInfo, crate::Error> {
        let status = self.get_req_game::<GameInfo>("/status")?; 

        Ok(status)
    }

    // Movement
        pub fn player_move(&self, dir : Direction) -> Result<MoveInfo, crate::Error> {
            self.post_req_player(format!("/move/{}", dir.to_string()).as_str())
        }

        pub fn player_dash(&self, dir : Direction) -> Result<DashInfo, crate::Error> {
            self.post_req_player(format!("/dash/{}", dir.to_string()).as_str())
        }

        pub fn player_teleport(&self, pos : crate::RelPos) -> Result<TeleportInfo, crate::Error> {
            self.get_req_player(format!("/teleport/{}/{}", pos.0, pos.1).as_str())
        }
    // 
    
    // Detection
        pub fn player_radar(&self) -> Result<RadarInfo, crate::Error> {
            self.get_req_player("/radar")
        }

        pub fn player_scan(&self) -> Result<ScanInfo, crate::Error> {
            self.get_req_player("/scan")
        }

        pub fn player_peak(&self, dir : Direction) -> Result<PeakInfo, crate::Error> {
            self.get_req_player(format!("/peak/{}", dir.to_string()).as_str())
        }
    //

    // Attack
        pub fn player_hit(&self, dir : Direction) -> Result<HitInfo, crate::Error> {
            self.post_req_player(format!("/hit/{}", dir.to_string()).as_str())
        }

        pub fn player_shoot(&self, dir : Direction) -> Result<ShootInfo, crate::Error> {
            self.post_req_player(format!("/shoot/{}", dir.to_string()).as_str())
        }

        pub fn player_ult(&self) -> Result<UltInfo, crate::Error> {
            self.post_req_player("/specialattack")
        }
    // 

    pub fn player_stats(&self) -> Result<StatsInfo, crate::Error> {
        self.get_req_player("/stats")
    }
}