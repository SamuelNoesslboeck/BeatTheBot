//! # BeatTheBot
//! 
//! Bot for the internal "BeatTheBot" programming competition
// #![warn(missing_docs)]

/// Wrapper functions to communicate with the API of the game
pub mod api;
pub use api::{Api, ApiError};

/// Functions and structs for rebuilding the actual game
pub mod game;
pub use game::{AbsPos, RelPos, OptPos};

pub mod player;

#[cfg(test)]
mod test;

/// General error type for crate
pub type Error = Box<dyn std::error::Error>;
