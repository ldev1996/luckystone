pub const MULTIPLIERS_LENGTH: usize = 8;
pub const NORMAL_MULTIPLIERS: [i8; MULTIPLIERS_LENGTH] = [-3, -2, -2, -1, 1, 2, 2, 3];
pub const GOOD_MULTIPLIERS: [i8; MULTIPLIERS_LENGTH] = [2, 2, 2, 2, 3, 3, 4, 5];
pub const BAD_MULTIPLIERS: [i8; MULTIPLIERS_LENGTH] = [-2, -2, -2, -2, -3, -3, -4, -5];

pub const INITIAL_ODDS: u8 = 1;
pub const INITIAL_CREDITS: i32 = 20;
pub const WIN_VALUE: i32 = 1_000_000;

pub const CHANCE_GROW_MIN: u8 = 3;
pub const CHANCE_GROW_MAX: u8 = 7;
pub const CHANCE_CAP: u8 = 50;
