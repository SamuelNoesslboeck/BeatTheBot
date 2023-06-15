use core::time::Duration;
use std::sync::{Arc, Mutex};
use std::time::Instant;

use serde::{Serialize, Deserialize};

use crate::Api;
use crate::api::ActionInfo;
use crate::player::Player;

mod logging;
pub use logging::*;

mod map;
pub use map::*;

mod units; 
pub use units::*;

pub type LoopFn<L> = fn (&Api, &Arc<Mutex<Player>>, &Arc<Mutex<GameStats>>, &Arc<Mutex<Events<L>>>, &Arc<Mutex<L>>) -> Result<(), crate::Error>;
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

    pub hits : u32,
    pub criticals : u32,
    pub shots : u32, 

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

    pub fn last_stamp_elapsed(&self) -> Option<Duration> {
        if let Some(stamp) = self.stamps.last() {
            return Some(stamp.1.elapsed());
        } else {
            None
        }
    }

    pub fn last_kills(&self) -> Option<u32> {
        if let Some(stamp) = self.stamps.last() {
            return Some(self.kills - stamp.0.kills);
        } else {
            None
        }
    }

    pub fn last_deaths(&self) -> Option<u32> {
        if let Some(stamp) = self.stamps.last() {
            return Some(self.deaths - stamp.0.deaths);
        } else {
            None
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

    pub fn kills_per_min(&self) -> Option<f32> {
        if let Some(stamp) = self.stamps.first() {
            let mins = stamp.1.elapsed().as_secs_f32() / 60.0;
            Some(self.kills as f32 / mins)
        } else {
            None
        }
    }

    pub fn deaths_per_min(&self) -> Option<f32> {
        if let Some(stamp) = self.stamps.first() {
            let mins = stamp.1.elapsed().as_secs_f32() / 60.0;
            Some(self.deaths as f32 / mins)
        } else {
            None
        }
    }
}

/// Main class managing, map, stats and bot
pub struct Game<L : Logger> {
    pub stats : Arc<Mutex<GameStats>>,
    pub logger : Arc<Mutex<L>>, 

    // Data
    pub api : Arc<Api>,
    events : Arc<Mutex<Events<L>>>,
    pub player : Arc<Mutex<Player>>,

    loops : Vec<std::thread::JoinHandle<()>>
}

impl<L : Logger + Send + 'static> Game<L> {
    pub fn new<S : Into<String>>(token : S, logger : L) -> Self {
        Self {
            stats: Default::default(),
            logger: Arc::new(Mutex::new(logger)), 

            api: Arc::new(Api::new(token.into())), 
            events: Arc::new(Mutex::new(Events::default())), 
            player: Arc::new(Mutex::new(Player::new())), 

            loops: Vec::new()
        }
    }

    // Actions
        pub fn start_min(&mut self) -> Result<(), crate::Error> {
            self.api.game_create()?;

            let ev = self.events.lock().unwrap(); 
            let mut logger= self.logger.lock().unwrap();
            let mut stats = self.stats.lock().unwrap();

            if let Some(on_start) = ev.on_start {
                on_start(&mut logger, &mut stats);
            }

            Ok(())
        }

        pub fn start(&mut self) -> Result<(), crate::Error> {
            self.start_min()?; 

            self.start_stats_loop();
            self.start_radar_loop();
            self.start_move_loop();
            self.start_hit_loop();
            self.start_teleport_loop();

            Ok(())
        }

        pub fn close(&mut self) -> Result<(), crate::Error> {
            self.api.game_close()?;
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
        pub fn append_loop(&mut self, loop_func : LoopFn<L>) {
            let api = self.api.clone();
            let player = self.player.clone();
            let stats = self.stats.clone(); 
            let events = self.events.clone();
            let logger = self.logger.clone();
    
            self.loops.push(std::thread::spawn(move || {
                loop {
                    if let Err(err) = loop_func(&api, &player, &stats, &events, &logger) {
                        if err.to_string().starts_with("No running game") {
                            break;
                        } 
        
                        println!(" => Error in status thread! {}", err);
                    }
                }
            })); 
        }
        
        pub fn start_stats_loop(&mut self) {
            self.append_loop(|api, player_mut, gstats_mut, events_mut, logger_mut| -> Result<(), crate::Error> {
                    // Returns an error if the game is not running yet
                    api.game_status()?;
        
                    let mut stats_prev = crate::api::StatsInfo::default();
        
                    loop {
                        let stats = api.player_stats()?;
        
                        let events = events_mut.lock().unwrap();
                        let mut logger = logger_mut.lock().unwrap();
                        let mut gstats = gstats_mut.lock().unwrap();
                        let mut player = player_mut.lock().unwrap();
        
        
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
                                player.reset_pos();
        
                                if let Some(on_death) = events.on_death {
                                    on_death(&mut logger, &mut gstats);
                                }
                            }
                        }
        
                        if stats_prev.level.name != stats.level.name {
                            player.reset_pos();

                            if gstats.map.kills_goal.is_some() {
                                gstats.kills += 1;
        
                                if let Some(on_kill) = events.on_kill {
                                    on_kill(&mut logger, &mut gstats);
                                }
                            }
        
                            if let Some(on_new_level) = events.on_new_level {
                                on_new_level(&mut logger, &mut gstats);
                            }
        
                            gstats.kills += 1;
        
                            if let Some(on_kill) = events.on_kill {
                                on_kill(&mut logger, &mut gstats);
                            }
                        }
        
                        stats_prev = stats;
        
                        drop(logger);
                        drop(events);
                        drop(gstats);
                        drop(player);
        
                        std::thread::sleep(Duration::from_millis(50));
                    }
                });
        }

        pub fn start_move_loop(&mut self) {
            self.append_loop(|api, player_mut, gstats_mut, _, _| -> Result<(), crate::Error> {
                loop {
                    let opt_dir = player_mut.lock().unwrap().get_dir(); 
                    let inst = Instant::now();
    
                    if let Some(dir) = opt_dir {
                        let m_info = api.player_move(dir)?;
                        let stats = gstats_mut.lock().unwrap();
                        let mut player = player_mut.lock().unwrap();
        
                        if !m_info.executed() {
                            // TODO: Proper error
                            println!("Move not executed!");
                            continue;
                        }
        
                        if !m_info.moved {
                            player.set_dir(Some(dir.opposite()));
                            player.map_wall(dir, &stats.map);
                        }
        
                        drop(player);
                        drop(stats);
                    }
    
                    let elapsed = inst.elapsed().as_secs_f32() * 0.3;  // Safety factor of 0.3
    
                    if elapsed < 0.25 {
                        std::thread::sleep(Duration::from_secs_f32(0.25 - elapsed));
                    }
                }
            });
        }

        pub fn start_radar_loop(&mut self) {
            self.append_loop(|api, player_mut, _, _, _| -> Result<(), crate::Error> {
                let mut last_dash = Instant::now(); 
                loop {
                    let inst = Instant::now();
                    let r_info = api.player_radar()?;
                    // let stats = gstats_mut.lock().unwrap();
                    let mut player = player_mut.lock().unwrap();
    
                    if !r_info.executed() {
                        // TODO: Proper error
                        println!("Radar not executed!");
                        continue;
                    }
                    
                    if r_info.results.sameblock == 0 {
                        if let Some(dir) = r_info.best_move_dir() {
                            player.set_dir(Some(dir));
        
                            if last_dash.elapsed().as_secs_f32() > 5.0 {
                                let d_info = api.player_dash(dir)?;
        
                                if d_info.executed() {
                                    println!(" => Dashed!");
                                }
        
                                last_dash = Instant::now();
                            }
                        }   
                    } else {
                        player.set_dir(None);
                    }
    
                    drop(player);
    
                    let elapsed = inst.elapsed().as_secs_f32() * 0.3;  // Safety factor of 0.75
    
                    if elapsed < 0.25 {
                        std::thread::sleep(Duration::from_secs_f32(0.25 - elapsed));
                    }
                }
            });
        }

        pub fn start_hit_loop(&mut self) {
            self.append_loop(|api, player_mut, gstats_mut, _, _| -> Result<(), crate::Error> {
                loop {
                    let dir = player_mut.lock().unwrap().get_dir(); 
                    let inst = Instant::now();
                    let h_info = api.player_hit(dir.unwrap_or(crate::api::Direction::North))?;
                    let mut stats = gstats_mut.lock().unwrap();
    
                    if !h_info.executed() {
                        // TODO: Proper error
                        println!("Hit not executed!");
                        continue;
                    }
    
                    stats.hits += h_info.hit;
                    stats.criticals += h_info.critical;
    
                    drop(stats);
    
                    let elapsed = inst.elapsed().as_secs_f32() * 0.3;  // Safety factor of 0.75
    
                    if elapsed < 0.25 {
                        std::thread::sleep(Duration::from_secs_f32(0.25 - elapsed));
                    }
                }
            });
        }

        pub fn start_teleport_loop(&mut self) {
            self.append_loop(|api, _, _, _, _| -> Result<(), crate::Error> {
                loop {
                    let pos; 

                    loop {
                        let s_info = api.player_scan()?;

                        if s_info.executed() {
                            if let Some(new_pos) = s_info.nearest.to_optpos().try_conv() {
                                pos = new_pos;
                                break;
                            }
                        }

                        std::thread::sleep(Duration::from_secs_f32(2.0));
                    }

                    let t_info = api.player_teleport(pos)?;
    
                    if !t_info.executed() {
                        // TODO: Proper error
                        println!("Hit not executed!");
                        continue;
                    } else {
                        println!(" => Teleported!"); 
                    }

                    std::thread::sleep(Duration::from_secs_f32(20.0));
                }
            });
        }
    // 
}