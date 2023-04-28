use core::time::Duration;
use std::sync::{Arc, Mutex};

mod logging;
pub use logging::*;

mod units; 
pub use units::*;

#[derive(Clone, Debug)]
pub struct Map {
    pub size : AbsPos
}

pub type LogFunc<L : Logger> = Option<fn (&mut L)>;

#[derive(Default)]
pub struct Events<L : Logger + Default> {
    pub on_kill : LogFunc<L>,
    pub on_death : LogFunc<L>,
    pub on_new_level : LogFunc<L>
    // pub on_update : 
}

#[derive(Clone, Debug, Default)]
pub struct GameStats {
    pub kills : u32, 
    pub deaths : u32
}

/// Main class managing, map, stats and bot
pub struct Game<L : Logger + Default> {
    pub stats : Arc<Mutex<GameStats>>,
    pub logger : Arc<Mutex<L>>, 

    // Data
    token : String, 
    events : Arc<Events<L>>,

    // Loops
    stats_loop : Option<std::thread::JoinHandle<()>>
}

impl<L : Logger + Default + Send> Game<L> {
    pub fn new(token : String, logger : L) -> Self {
        Self {
            stats: Default::default(),
            logger: Arc::new(Mutex::new(logger)), 

            token, 
            events: Default::default(), 

            stats_loop: None
        }
    }

    pub fn start_stats_loop(&mut self) {
        self.stats_loop = Some(std::thread::spawn(move || { 
            let func = 
            |token : String, events : Arc<Events<L>>, logger_mut : Arc<Mutex<L>>| -> Result<(), crate::Error> {
                let mut api= crate::Api::new(token);
    
                // Returns an error if the game is not running yet
                api.game_status()?;
    
                let mut stats_prev = crate::api::StatsInfo::default();
    
                loop {
                    let stats = api.player_stats()?;
                    let mut logger = logger_mut.lock().unwrap();
    
                    if stats.perf.kills > stats_prev.perf.kills {
                        for _ in 0 .. (stats.perf.kills - stats_prev.perf.kills) {
                            if let Some(on_kill) = events.on_kill {
                                
                                on_kill(&mut logger);
                            }
                        }
                    }
    
                    if stats.perf.deaths > stats_prev.perf.deaths {
                        for _ in 0 .. (stats.perf.deaths - stats_prev.perf.deaths) {
                            if let Some(on_death) = events.on_death {
                                on_death(&mut logger);
                            }
                        }
                    }
    
                    stats_prev = stats;
    
                    std::thread::sleep(Duration::from_millis(100));
                }
            }; 
    
            loop {
                if let Err(err) = 
                func(self.token.clone(), self.events.clone(), self.logger.clone()) {
                    println!(" => Error in status thread! {}", err);
                    break;
                }
            }
        }));
    }
}