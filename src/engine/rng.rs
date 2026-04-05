use crate::constants::{CHANCE_GROW_MAX, CHANCE_GROW_MIN, MULTIPLIERS_LENGTH};

/// Abstraction over the random number generator used by the game engine.
///
/// This trait allows the game logic to remain deterministic and testable by
/// injecting different random providers (for example, deterministic RNGs in tests).
///
/// Implementations must respect the value ranges described by each method.
/// The game logic assumes these invariants and does not perform additional checks.
pub trait RandomProvider {
    /// Returns a pseudo-random percentage value in the range `1..=100`.
    ///
    /// This value is used to determine which event occurs during a turn.
    fn percent(&self) -> u8;

    /// Returns a pseudo-random index into the multiplier tables.
    ///
    /// The returned value is guaranteed to be within
    /// `0..MULTIPLIERS_LENGTH`, making it safe to index
    /// `NORMAL_MULTIPLIERS`, `GOOD_MULTIPLIERS`, and `BAD_MULTIPLIERS`.
    fn multiplier_index(&self) -> usize;

    /// Returns a pseudo-random growth value used to increase event probabilities.
    ///
    /// The value is guaranteed to be in the range
    /// `CHANCE_GROW_MIN..=CHANCE_GROW_MAX`.
    fn growth(&self) -> u8;
}

pub struct FastRandom;

impl RandomProvider for FastRandom {
    fn percent(&self) -> u8 {
        fastrand::u8(1..=100)
    }

    fn multiplier_index(&self) -> usize {
        fastrand::usize(..MULTIPLIERS_LENGTH)
    }

    fn growth(&self) -> u8 {
        fastrand::u8(CHANCE_GROW_MIN..=CHANCE_GROW_MAX)
    }
}
