//! # BeatTheBot
//! 
//! Bot for the internal "BeatTheBot" programming competition
#![warn(missing_docs)]

/// Wrapper functions to communicate with the API of the game
mod api;

#[cfg(test)]
mod test;

/// General error type for crate
pub type Error = Box<dyn std::error::Error>;

fn main() -> Result<(), Error> {
    let mut a = api::Api::new("f2b4f1a2-07ba-4a41-bb42-37a8a7206970".to_owned());

    dbg!(a.game_create()?);

    dbg!(a.player_move(api::Direction::North)?);
    dbg!(a.player_move(api::Direction::South)?);

    dbg!(a.player_radar()?);
    dbg!(a.player_radar()?);

    dbg!(a.game_status()?);
    dbg!(a.game_close()?);

    Ok(())
}
