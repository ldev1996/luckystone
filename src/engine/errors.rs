use std::fmt::Display;

/// Errors that can occur during normal gameplay.
///
/// These represent invalid player actions or input.  
/// Errors caused by programming mistakes or invalid internal state
/// are not represented here.
#[derive(Debug)]
pub enum GameError {
    /// The player attempted to gamble zero or a negative amount.
    NonPositiveBet,
    /// The player attempted to gamble more credits than they current balance.
    InsufficientCredits,
    /// The provided input could not be parsed as a number.
    InvalidInput,
}

impl Display for GameError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                GameError::NonPositiveBet => "⚠ You must gamble more than zero!",
                GameError::InsufficientCredits => "⚠ Insufficient credits!",
                GameError::InvalidInput => "⚠ Please type a number!",
            }
        )
    }
}
