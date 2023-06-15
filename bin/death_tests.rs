use core::time::Duration;
use doof_bot::{Game, FileLogger, Logger, RelPos};


fn main() -> Result<(), doof_bot::Error> {
    let mut game = Game::new(
        include_str!("../assets/token.key"),
        FileLogger::new("log/bot.log")
    );

    game.on_start(Some(|log, _| {
        log.logln("[death-test]");
    }));

    game.on_death(Some(|log, _| {
        log.logln(" => Player died!");
    }));

    game.start_min()?;
    game.start_stats_loop();

    dbg!(game.api.player_teleport(RelPos(50, 50)));
    dbg!(game.api.player_ult());

    std::thread::sleep(Duration::from_secs_f32(0.5));
    dbg!(game.api.player_teleport(RelPos(50, 50)));
    dbg!(game.api.player_ult());

    game.close()?;

    Ok(())
}