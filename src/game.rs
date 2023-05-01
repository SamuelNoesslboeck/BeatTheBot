use core::time::Duration;
use std::sync::{Arc, Mutex};
use std::time::Instant;

use serde::{Serialize, Deserialize};

mod logging;
pub use logging::*;

mod map;
pub use map::*;

mod units; 
pub use units::*;

use crate::Api;

pub type LogFunc<L> = Option<fn (&mut L, &mut GameStats)>;

pub struct Events<L : Logger> {
    pub on_kill : LogFunc<L>,
    pub on_death : LogFunc<L>,

    pub on_new_level : LogFunc<L>,

    pub on_start : LogFunc<L>,
    pub on_finish : LogFunc<L>
}

impl<L : Logger> Default for Events<L> {
    fn default() -> Self {
        Self {
            on_kill: None,
            on_death: None,

            on_new_level: None,

            on_start: None, 
            on_finish: None
        }
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct GameStats {
    pub kills : u32, 
    pub deaths : u32,

    #[serde(skip)]
    pub map : Map,

    #[serde(skip)]
    pub stamps : Vec<(Self, Instant)>
}

impl GameStats {
    pub fn clone_nostamp(&self) -> Self {
        Self {
            kills: self.kills,
            deaths: self.deaths,

            ..Default::default()
        }
    }

    pub fn create_stamp(&mut self) {
        self.stamps.push(
            (self.clone_nostamp(), Instant::now())
        );
    }

    pub fn kd(&self) -> f32 {
        self.kills as f32 / self.deaths as f32
    }
}

/// Main class managing, map, stats and bot
pub struct Game<L : Logger> {
    pub stats : Arc<Mutex<GameStats>>,
    pub logger : Arc<Mutex<L>>, 

    // Data
    token : String, 
    events : Arc<Mutex<Events<L>>>,

    // Acting
    api : Api,
    stats_loop : Option<std::thread::JoinHandle<()>>
}

impl<L : Logger + Send + 'static> Game<L> {
    pub fn new(token : String, logger : L) -> Self {
        Self {
            stats: Default::default(),
            logger: Arc::new(Mutex::new(logger)), 

            token: token.clone(), 
            events: Arc::new(Mutex::new(Events::default())), 

            api: Api::new(token),
            stats_loop: None
        }
    }

    // Actions
        pub fn start(&mut self) -> Result<(), crate::Error> {
            self.api.game_create()?;

            let ev = self.events.lock().unwrap(); 
            let mut logger= self.logger.lock().unwrap();
            let mut stats = self.stats.lock().unwrap();

            if let Some(on_start) = ev.on_start {
                on_start(&mut logger, &mut stats);
            }

            Ok(())
        }
    // 

    // Events
        #[inline]
        pub fn on_kill(&mut self, on_kill : LogFunc<L>) {
            let mut ev = self.events.lock().unwrap(); 
            ev.on_kill = on_kill;
        }

        #[inline]
        pub fn on_death(&mut self, on_death : LogFunc<L>) {
            let mut ev = self.events.lock().unwrap(); 
            ev.on_death = on_death;
        }

        #[inline]
        pub fn on_new_level(&mut self, on_new_level : LogFunc<L>) {
            let mut ev = self.events.lock().unwrap(); 
            ev.on_new_level = on_new_level;
        }

        #[inline]
        pub fn on_start(&mut self, on_start : LogFunc<L>) {
            let mut ev = self.events.lock().unwrap(); 
            ev.on_start = on_start;        
        }

        #[inline]
        pub fn on_finish(&mut self, on_finish : LogFunc<L>) {
            let mut ev = self.events.lock().unwrap(); 
            ev.on_finish = on_finish;        
        }
    //

    // Loops
        pub fn start_stats_loop(&mut self) {
            self.stats_loop = Some(start_stats_loop(self.token.clone(), self.events.clone(), self.logger.clone(), self.stats.clone()));
        }
    // 
}

fn start_stats_loop<L : Logger + Send + 'static>(token : String, events_mut : Arc<Mutex<Events<L>>>, logger_mut : Arc<Mutex<L>>, gstats_mut : Arc<Mutex<GameStats>>) -> std::thread::JoinHandle<()> {
    std::thread::spawn(move || { 
        let func = 
        |token : String, events_mut : Arc<Mutex<Events<L>>>, logger_mut : Arc<Mutex<L>>, stats_mut : Arc<Mutex<GameStats>>| -> Result<(), crate::Error> {
            let mut api= crate::Api::new(token);

            // Returns an error if the game is not running yet
            api.game_status()?;

            let mut stats_prev = crate::api::StatsInfo::default();

            loop {
                let stats = api.player_stats()?;

                let mut logger = logger_mut.lock().unwrap();
                let events = events_mut.lock().unwrap();
                let mut gstats = stats_mut.lock().unwrap();

                if stats.perf.kills > stats_prev.perf.kills {
                    for _ in 0 .. (stats.perf.kills - stats_prev.perf.kills) {
                        gstats.kills += 1;

                        if let Some(on_kill) = events.on_kill {
                            on_kill(&mut logger, &mut gstats);
                        }
                    }
                }

                if stats.perf.deaths > stats_prev.perf.deaths {
                    for _ in 0 .. (stats.perf.deaths - stats_prev.perf.deaths) {
                        gstats.deaths += 1;

                        if let Some(on_death) = events.on_death {
                            on_death(&mut logger, &mut gstats);
                        }
                    }
                }

                if stats_prev.level.name != stats.level.name {
                    if let Some(on_new_level) = events.on_new_level {
                        on_new_level(&mut logger, &mut gstats);
                    }
                }

                stats_prev = stats;

                drop(logger);
                drop(events);
                drop(gstats);

                std::thread::sleep(Duration::from_millis(80));
            }
        }; 

        loop {
            if let Err(err) = func(token.clone(), events_mut.clone(), logger_mut.clone(), gstats_mut.clone()) {
                if err.to_string().starts_with("No running game") {
                    let events = events_mut.lock().unwrap(); 
                    let mut logger = logger_mut.lock().unwrap();
                    let mut gstats = gstats_mut.lock().unwrap();
                
                    if let Some(on_finish) = events.on_finish {
                        on_finish(&mut logger, &mut gstats);
                    }

                    break;
                } else {

                }

                println!(" => Error in status thread! {}", err);
                break;
            }
        }
    })
}