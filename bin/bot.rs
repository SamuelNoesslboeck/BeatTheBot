use doof_bot::{Game, FileLogger, Logger};
use doof_bot::api::{ActionInfo, Api};

fn main() -> Result<(), doof_bot::Error> {
    let api = Api::new(include_str!("../assets/token.key").to_owned());

    let mut game = Game::new(
        include_str!("../assets/token.key").to_owned(), 
        FileLogger::new("log/bot.log".to_owned())
    );

    game.on_start(Some(|log, _| {
        log.logln("  [BeatTheBot]  ");
        log.logln("================");
        log.logln(" => Name: doof_bot");
        log.logln(" => Version: 0.0.1");
        log.logln("");
    }));

    game.on_kill(Some(|log, stats| {
        log.logln(format!(" => Made a kill! {}, {}", stats.kills, stats.kd()));
    }));

    game.on_death(Some(|log, stats| {
        log.logln(format!(" => Player died! {}", stats.deaths));
    }));

    game.on_new_level(Some(|log, stats| {
        log.logln("");
        log.logln("========================");
        log.logln(" => Reached next level!");
        log.logln("========================");
        log.logln(format!("{:?}", stats));
        log.logln("");

        if let Some(elapsed) = stats.last_stamp_elapsed() {
            log.logln(format!("Time elapsed: {}", elapsed.as_secs_f32())); 
        }

        if let Some(kpm) = stats.kills_per_min() {
            log.logln(format!("Kills per minute: {}", kpm));
        }

        if let Some(dpm) = stats.deaths_per_min() {
            log.logln(format!("Deaths per minute: {}", dpm));
        }

        stats.create_stamp();
    })); 

    game.on_finish(Some(|log, stats| {
        let j_str = serde_json::to_string_pretty(&stats).unwrap();
        std::fs::write("log/bot.json", &j_str).unwrap(); 
        log.logln(j_str);
    }));

    game.start()?;
    
    for _ in 0 .. 1000 {              
        let info =  api.player_ult()?;

        if !info.executed() {
            println!(" -> Not executed!"); 
        }
        
        for _ in 0 .. 5 {
            let s_info = api.player_shoot(game.player.lock().unwrap().get_dir().unwrap_or(doof_bot::api::Direction::North))?;

            if !s_info.executed() {
                println!(" -> Shoot not executed!");
            } else {
                if s_info.hit {
                    println!(" -> Hit a target!");
                }
            }

            std::thread::sleep(std::time::Duration::from_millis(1000));
        }

        // println!(" => Enimies hit with ult {}", hits);
        // dbg!(api.player_stats()?);
    }

    api.game_close()?;

    Ok(())
}