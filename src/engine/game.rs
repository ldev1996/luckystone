use crate::{
    constants::{INITIAL_CREDITS, INITIAL_ODDS, WIN_VALUE},
    engine::{Event, GameError, Odds, rng::RandomProvider},
};

#[derive(Clone, Copy)]
pub struct GambleOutcome {
    event: Event,
    multiplier: i8,
}

impl GambleOutcome {
    pub fn event(&self) -> Event {
        self.event
    }

    pub fn is_positive(&self) -> bool {
        self.multiplier > 0
    }
}

pub struct Game<R: RandomProvider> {
    credits: i32,
    highest_score: i32,
    odds: Odds,
    rng: R,
}

impl<R: RandomProvider> Game<R> {
    pub fn new(rng: R) -> Self {
        Self {
            credits: INITIAL_CREDITS,
            highest_score: INITIAL_CREDITS,
            odds: Odds::new(INITIAL_ODDS, INITIAL_ODDS),
            rng,
        }
    }

    pub fn credits(&self) -> i32 {
        self.credits
    }

    pub fn highest_score(&self) -> i32 {
        self.highest_score
    }

    pub fn odds(&self) -> &Odds {
        &self.odds
    }

    pub fn roll_event(&self) -> Event {
        self.odds.select_event(self.rng.percent())
    }

    pub fn has_won(&self) -> bool {
        self.credits >= WIN_VALUE
    }

    pub fn has_lost(&self) -> bool {
        self.credits <= 0
    }

    pub fn validate_bet(&self, amount: i32) -> Result<i32, GameError> {
        if amount <= 0 {
            return Err(GameError::NonPositiveBet);
        }
        if amount > self.credits {
            return Err(GameError::CreditOverflow);
        }
        Ok(amount)
    }

    pub fn gamble(&mut self, amount: i32) -> GambleOutcome {
        let event = self.roll_event();
        let multiplier = event.multiplier(self.rng.multiplier_index());
        self.odds.grow(self.rng.growth(), self.rng.growth(), event);

        self.credits += amount * multiplier as i32;
        self.highest_score = self.highest_score.max(self.credits);

        GambleOutcome { event, multiplier }
    }
}
