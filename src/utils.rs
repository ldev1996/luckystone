use crate::constants::{CHANCE_GROW_MAX, CHANCE_GROW_MIN, MULTIPLIERS_LENGTH};

pub fn random_percent() -> u8 {
    fastrand::u8(1..=100)
}

pub fn random_multiplier() -> usize {
    fastrand::usize(..MULTIPLIERS_LENGTH)
}

pub fn random_growth() -> u8 {
    fastrand::u8(CHANCE_GROW_MIN..=CHANCE_GROW_MAX)
}
