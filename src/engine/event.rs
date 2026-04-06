use crate::constants::{BAD_MULTIPLIERS, GOOD_MULTIPLIERS, MULTIPLIERS_LENGTH, NORMAL_MULTIPLIERS};

/// Represents the possible outcomes of a gambling turn.
///
/// Each event determines which multiplier table will be used
/// to compute the final result of the player's bet.
#[derive(Copy, Clone)]
pub enum Event {
    /// A positive event that uses the high-reward multiplier table.
    Jackpot,
    /// A negative event that uses the penalty multiplier table.
    LuckBreak,
    /// The default outcome when neither special event occurs.
    Normal,
}

impl Event {
    /// Returns the multiplier associated with this event.
    ///
    /// The multiplier is selected from one of the predefined
    /// multiplier tables depending on the event type.
    ///
    /// The provided index must be within the bounds of the
    /// multiplier tables.
    pub fn multiplier(self, i: usize) -> i8 {
        debug_assert!(i < MULTIPLIERS_LENGTH);

        match self {
            Event::Jackpot => GOOD_MULTIPLIERS[i],
            Event::LuckBreak => BAD_MULTIPLIERS[i],
            Event::Normal => NORMAL_MULTIPLIERS[i],
        }
    }
}
