use crate::{
    constants::{INITIAL_CREDITS, INITIAL_ODDS, WIN_VALUE},
    engine::{Event, GameError, Odds, rng::RandomProvider},
};

/// Result produced after a gamble is resolved.
///
/// A `GambleOutcome` contains the event that occurred during the turn
/// and the multiplier applied to the player's bet.
#[derive(Clone, Copy)]
pub struct GambleOutcome {
    event: Event,
    multiplier: i8,
}

impl GambleOutcome {
    /// Returns the event that occurred during the gamble.
    pub fn event(&self) -> Event {
        self.event
    }

    /// Indicates whether the gamble produced a positive credit change.
    pub fn is_positive(&self) -> bool {
        self.multiplier > 0
    }
}

/// Core game state and rules for Lucky Stone.
///
/// `Game` encapsulates all gameplay logic, including:
///
/// - player credits
/// - evolving event probabilities ([`Odds`])
/// - random number generation via [`RandomProvider`]
///
/// The game state is deterministic except for the injected random
/// provider, which allows tests to replace the RNG with predictable
/// implementations.
///
/// A game progresses through repeated calls to [`Game::gamble`].
/// Each turn resolves an [`Event`], applies a multiplier to the bet,
/// and updates the event probabilities.
pub struct Game<R: RandomProvider> {
    credits: i32,
    highest_score: i32,
    odds: Odds,
    rng: R,
}

impl<R: RandomProvider> Game<R> {
    /// Creates a new game instance with the initial configuration.
    ///
    /// The player starts with [`INITIAL_CREDITS`] and the event
    /// probabilities defined by [`INITIAL_ODDS`].
    ///
    /// The random provider determines how events and multipliers
    /// are generated during gameplay.
    pub fn new(rng: R) -> Self {
        Self {
            credits: INITIAL_CREDITS,
            highest_score: INITIAL_CREDITS,
            odds: Odds::new(INITIAL_ODDS, INITIAL_ODDS),
            rng,
        }
    }

    /// Returns the player's current credit balance.
    pub fn credits(&self) -> i32 {
        self.credits
    }

    /// Returns the player's highest credit value reached during the game.
    pub fn highest_score(&self) -> i32 {
        self.highest_score
    }

    /// Returns the current event probability state.
    pub fn odds(&self) -> &Odds {
        &self.odds
    }

    /// Rolls a random event based on the current probabilities.
    ///
    /// The event is selected using a random percentage provided
    /// by the configured [`RandomProvider`].
    ///
    /// The method is used in [`Game::gamble`].
    pub fn roll_event(&self) -> Event {
        self.odds.select_event(self.rng.percent())
    }

    /// Returns `true` if the player has reached the winning condition ([`WIN_VALUE`]).
    pub fn has_won(&self) -> bool {
        self.credits >= WIN_VALUE
    }

    /// Returns `true` if the player has no credits remaining.
    pub fn has_lost(&self) -> bool {
        self.credits <= 0
    }

    /// Validates whether a bet is allowed.
    ///
    /// Errors if the bet is non-positive or exceeds the player's
    /// current credit balance.
    pub fn validate_bet(&self, amount: i32) -> Result<i32, GameError> {
        if amount <= 0 {
            return Err(GameError::NonPositiveBet);
        }
        if amount > self.credits {
            return Err(GameError::InsufficientCredits);
        }
        Ok(amount)
    }

    /// Resolves a gambling turn.
    ///
    /// The method:
    ///
    /// 1. Rolls an [`Event`]
    /// 2. Selects a multiplier associated with that event
    /// 3. Updates the event probabilities
    /// 4. Applies the result to the player's credits
    ///
    /// The outcome of the turn is returned as [`GambleOutcome`].
    pub fn gamble(&mut self, amount: i32) -> GambleOutcome {
        let event = self.roll_event();
        let multiplier = event.multiplier(self.rng.multiplier_index());
        self.odds.grow(self.rng.growth(), self.rng.growth(), event);

        self.credits += amount * multiplier as i32;
        self.highest_score = self.highest_score.max(self.credits);

        GambleOutcome { event, multiplier }
    }
}
