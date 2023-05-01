use std::time::Instant;

use doof_bot::{Game, FileLogger, Logger};
use doof_bot::api::{ActionInfo, Api, Direction};

fn main() -> Result<(), doof_bot::Error> {
    let api = Api::new(include_str!("../assets/token.key").to_owned());

    let mut game = Game::new(
        include_str!("../assets/token.key").to_owned(), 
        FileLogger::new("log/map_wall.log".to_owned())
    );

    game.on_start(Some(|log, _| {
        log.logln("[Wall-Mapper]");
    }));

    game.on_death(Some(|log, _| {
        log.logln(" => Player died");
    }));

    game.start()?;
    game.start_stats_loop();
    
    let mut dir = Direction::North; 

    let mut length = 0;
    let mut inst = Instant::now();
    let mut comp;

    loop {
        comp = Instant::now();
        let m_info = api.player_move(dir)?;

        length += 1;

        if !m_info.executed() {
            println!("Move failed!");
        }

        if !m_info.moved {
            println!(" -> Switching directions! Fields: {}, Time per Field: {}", length, inst.elapsed().as_secs_f32() / length as f32);
            length = 0;
            inst = Instant::now();

            if dir == Direction::North {
                dir = Direction::South;
            } else {
                dir = Direction::North;
            }
        }

        std::thread::sleep(std::time::Duration::from_millis(250) - comp.elapsed());
    }
}