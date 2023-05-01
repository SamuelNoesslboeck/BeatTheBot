use doof_bot::{Game, FileLogger, Logger};
use doof_bot::api::{ActionInfo, Api};

fn main() -> Result<(), doof_bot::Error> {
    let mut api = Api::new(include_str!("../assets/token.key").to_owned());

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

    game.on_new_level(Some(|log, stats| {
        log.logln("");
        log.logln("========================");
        log.logln(" => Reached next level!");
        log.logln("========================");
        log.logln(format!("| Kills: {}", stats.kills));
        log.logln(format!("| Deaths: {}", stats.deaths));
        log.logln("");
    })); 

    game.on_finish(Some(|log, stats| {
        let j_str = serde_json::to_string_pretty(&stats).unwrap();
        std::fs::write("log/bot.json", &j_str).unwrap(); 
        log.logln(j_str);
    }));

    game.start()?;
    game.start_stats_loop();
    
    for _ in 0 .. 30 {              
        for _ in 0 .. 10 {          // About 1 minute
            let info =  api.player_ult()?;

            if info.executed() {
            } else {    
                println!(" -> Not executed!"); 
            }

            for _ in 0 .. 20 {
                let r_info = api.player_radar()?;

                if r_info.executed() {
                    let dir = r_info.best_move_dir(); 

                    let m_info = api.player_move(dir)?;
                    let h_info  = api.player_hit(dir)?;

                    if !m_info.executed() {
                        println!(" -> Move not executed!");
                    }

                    if !m_info.moved {
                        println!(" -> Hit a wall! {:?}", dir);
                    }

                    if h_info.executed() {
                        if h_info.hit > 0 {
                            // println!("Hit a target with normal attack! {}", h_info.hit);
                        }
                    } else {
                        println!(" -> Hit failed");
                    }
                } else {
                    println!(" -> Radar not executed!");
                }

                std::thread::sleep(std::time::Duration::from_millis(250));
            }
        }

        // println!(" => Enimies hit with ult {}", hits);
        // dbg!(api.player_stats()?);
    }

    api.game_close()?;

    Ok(())
}