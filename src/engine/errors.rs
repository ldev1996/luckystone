use std::fmt::Display;

pub enum GameError {
    NonPositiveBet,
    CreditOverflow,
    InvalidInput,
}

impl Display for GameError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                GameError::NonPositiveBet => "⚠ You must gamble more than zero!",
                GameError::CreditOverflow => "⚠ Insufficient credits!",
                GameError::InvalidInput => "⚠ Please type a number",
            }
        )
    }
}
