use reqwest::blocking::Client;
use serde::de::DeserializeOwned;

mod objs;
pub use objs::*;

const ADDRESS : &str = "https://game-dd.countit.at/api";

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
    client : Client,

    running : bool
}

impl Api {
    pub fn new(token : String) -> Self {
        Self {
            token: token,
            client: Client::new(),

            running: false
        }
    }

    // Requests
        /// Sends a general post request to the game-API
        #[inline(always)]
        fn post_req<T : DeserializeOwned>(&self, url : &str) -> Result<T, crate::Error> {
            match self.client.post(url).body("")
                .send() {
                Ok(res) => { 
                    #[cfg(not(feature = "high-dbg"))]
                    let res = Ok(res.json::<T>()?);

                    #[cfg(feature = "high-dbg")]
                    let res = Ok(dbg!(res).json::<T>()?); 

                    res
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

        /// Sends a game-specific post request to the game-API
        #[inline(always)]
        fn game_post_req<T : DeserializeOwned>(&self, path : &str) -> Result<T, crate::Error> {
            self.post_req(format!("{}{}/{}{}", ADDRESS, "/game", self.token, path).as_str())
        }

        /// Sends a game-specific post request to the game-API
        #[inline(always)]
        fn player_post_req<T : DeserializeOwned>(&self, path : &str) -> Result<T, crate::Error> {
            self.check_running();
            self.post_req(format!("{}{}/{}{}", ADDRESS, "/player", self.token, path).as_str())
        }
    // 

    // Errors
        #[inline(always)]
        fn check_running(&self) {
            if !self.running {
                panic!("The game is not running currently!");       // TODO: Replace with errors
            }    
        }
    // 
    
    pub fn game_create(&mut self) -> Result<Option<GameInfo>, crate::Error> {
        match self.game_post_req::<GameInfo>("/create") {
            Ok(val) => {
                self.running = true;
                Ok(Some(val))
            },
            // React "already existing" error
            Err(err) => if err.to_string().contains("missing field `gameid`") {   
                self.running = true;  
                Ok(None)
            } else {
                Err(err)
            }
        }
    }
    
    pub fn game_close(&mut self) -> Result<Option<GameInfo>, crate::Error> {
        match self.game_post_req::<GameInfo>("/close") {
            Ok(val) => {
                self.running = false; 
                Ok(Some(val))
            },
            // React "already closed" error
            Err(err) => if err.to_string().contains("missing field `gameid`") {  
                self.running = false;   
                Ok(None)
            } else {
                Err(err)
            }
        }
    }

    pub fn game_status(&mut self) -> Result<GameInfo, crate::Error> {
        let status = self.game_post_req::<GameInfo>("/close")?; 
        self.running = status.running;

        Ok(status)
    }

    pub fn player_move(&self, dir : Direction) -> Result<MoveInfo, crate::Error> {
        self.player_post_req(format!("/move/{}", dir.to_string()).as_str())
    }

    pub fn player_dash(&self, dir : Direction) -> Result<MoveInfo, crate::Error> {
        self.player_post_req(format!("/dash/{}", dir.to_string()).as_str())
    }

    pub fn player_radar(&self) -> Result<RadarResult, crate::Error> {
        self.player_post_req("/radar")
    }
}