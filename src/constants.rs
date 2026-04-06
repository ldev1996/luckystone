/// The shared length of all the multiplier tables. With a shared length,
/// the method [`RandomProvider::multiplier_index`] can be much more simple, by using this
/// constant as the maximum generated value.
pub const MULTIPLIERS_LENGTH: usize = 8;

/// The multiplier table used in [`Event::Normal`](crate::engine::Event::Normal).
pub const NORMAL_MULTIPLIERS: [i8; MULTIPLIERS_LENGTH] = [-3, -2, -2, -1, 1, 2, 2, 3];

/// The multiplier table used in [`Event::Jackpot`](crate::engine::Event::Jackpot).
pub const GOOD_MULTIPLIERS: [i8; MULTIPLIERS_LENGTH] = [2, 2, 2, 2, 3, 3, 4, 5];

/// The multiplier table used in [`Event::LuckBreak`](crate::engine::Event::LuckBreak).
pub const BAD_MULTIPLIERS: [i8; MULTIPLIERS_LENGTH] = [-2, -2, -2, -2, -3, -3, -4, -5];

/// The initial value for both `Jackpot` and `Luck Break` odds in the [`Odds`](crate::engine::Odds)
/// struct.
pub const INITIAL_ODDS: u8 = 1;

/// The initial amount of credits a player has in a new [`Game`](crate::engine::Game) instance.
pub const INITIAL_CREDITS: i32 = 20;

/// The value used to check whether the game has been won.
///
/// **Safety constraint:**  
/// To avoid integer overflow when updating `credits`, ensure that:
///
/// ```rust
/// WIN_VALUE * (largest multiplier between all tables) + 1 <= i32::MAX
/// ```
///
/// In this code, the largest multiplier is 5 (from `GOOD_MULTIPLIERS`).
/// This ensures that the addition performed in [`Game::gamble`](crate::engine::Game::gamble) using `+=`
/// will not overflow `i32`.
pub const WIN_VALUE: i32 = 1_000_000;

/// The minimum value by which [`Odds::grow`](crate::engine::Odds::grow)
/// can increase the odds each turn.
pub const CHANCE_GROW_MIN: u8 = 3;

/// The maximum value by which [`Odds::grow`](crate::engine::Odds::grow)
/// can increase the odds each turn.
pub const CHANCE_GROW_MAX: u8 = 7;

/// Determines the maximum value for `Jackpot` or `Luck Break` odds in the
/// [`Odds`](crate::engine::Odds) struct. It can't be
/// higher than 50 or the odds may overlap each other.
pub const CHANCE_CAP: u8 = 50;
