use std::time::Duration;
use std::thread::sleep;

/// Checks if the API function "game_create" is functioning
#[test]
fn game_create() -> Result<(), crate::Error> {
    let mut a = crate::api::Api::new(include_str!("../assets/token.key").to_owned());

    // Make sure the game is closed
    a.game_close()?;

dbg!(a.game_create()?);
    sleep(Duration::from_secs_f32(0.5));

    a.game_close()?;



    Ok(())
}

/// Checks if the API function "game_close" is functioning
#[test]
fn game_close() -> Result<(), crate::Error> {
    let mut a = crate::api::Api::new(include_str!("../assets/token.key").to_owned());

    // Make sure the game is open
    a.game_create()?;

    // Close the game
    dbg!(a.game_close()?);

    Ok(())
}