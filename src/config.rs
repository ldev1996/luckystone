pub const MULTIPLIERS_LENGTH: usize = 7;
pub const NORMAL_MULTIPLIERS: [i8; MULTIPLIERS_LENGTH] = [-5, -3, -2, 1, 2, 3, 5];
pub const GOOD_MULTIPLIERS: [i8; MULTIPLIERS_LENGTH] = [2, 2, 2, 3, 3, 5, 5];
pub const BAD_MULTIPLIERS: [i8; MULTIPLIERS_LENGTH] = [-2, -2, -2, -3, -3, -5, -5];

pub const INITIAL_CHANCE: u8 = 1;
pub const INITIAL_VALUE: i32 = 20;
pub const WIN_VALUE: i32 = 1_000_000;

pub const CHANCE_GROW_MIN: u8 = 3;
pub const CHANCE_GROW_MAX: u8 = 7;
pub const CHANCE_CAP: u8 = 50;
