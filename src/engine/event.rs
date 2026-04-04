use crate::constants::{BAD_MULTIPLIERS, GOOD_MULTIPLIERS, NORMAL_MULTIPLIERS};

#[derive(Copy, Clone)]
pub enum Event {
    Jackpot,
    LuckBreak,
    Normal,
}

impl Event {
    pub fn multiplier(self, i: usize) -> i8 {
        match self {
            Event::Jackpot => GOOD_MULTIPLIERS[i],
            Event::LuckBreak => BAD_MULTIPLIERS[i],
            Event::Normal => NORMAL_MULTIPLIERS[i],
        }
    }
}
