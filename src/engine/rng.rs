use crate::constants::{CHANCE_GROW_MAX, CHANCE_GROW_MIN, MULTIPLIERS_LENGTH};

pub trait RandomProvider {
    fn percent(&self) -> u8;

    fn multiplier_index(&self) -> usize;

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
