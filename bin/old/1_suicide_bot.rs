use doof_bot::api::{ActionInfo, Api};

fn main() -> Result<(), doof_bot::Error> {
    let mut api = Api::new(include_str!("../assets/token.key").to_owned());

    let mut hits = 0;

    api.game_create()?;

    for _ in 0 .. 30 {              
        for _ in 0 .. 10 {          // About 1 minute
            let info =  api.player_ult()?;

            if info.executed() {
                println!("Ulting! {}", info.hits);
                hits += info.hits;
            } else {    
                println!(" -> Not executed!"); 
            }

            for _ in 0 .. 20 {
                let r_info = api.player_radar()?;

                if r_info.executed() {
                    let m_info = api.player_move(r_info.best_move_dir())?;
                    let h_info  = api.player_hit(r_info.best_move_dir())?;

                    if !m_info.executed() {
                        println!(" -> Move not executed!");
                    }

                    if h_info.executed() {
                        if h_info.hit > 0 {
                            println!("Hit a target with normal attack! {}", h_info.hit);
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

        println!(" => Enimies hit with ult {}", hits);
        dbg!(api.player_stats()?);
    }

    api.game_close()?;

    Ok(())
}