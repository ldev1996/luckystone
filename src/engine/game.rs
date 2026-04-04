use crate::{
    constants::{INITIAL_CREDITS, INITIAL_ODDS, WIN_VALUE},
    engine::{Event, Odds, rng::RandomProvider},
};

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
        let random_chance = self.rng.percent();
        if random_chance <= self.odds.jackpot() {
            Event::Jackpot
        } else if random_chance > 100 - self.odds.luck_break() {
            Event::LuckBreak
        } else {
            Event::Normal
        }
    }

    pub fn has_won(&self) -> bool {
        self.credits >= WIN_VALUE
    }

    pub fn has_lost(&self) -> bool {
        self.credits <= 0
    }

    pub fn gamble(&mut self, amount: i32) -> (Event, i8) {
        let event = self.roll_event();
        let multiplier = event.multiplier(self.rng.multiplier_index());
        self.odds
            .update(self.rng.growth(), self.rng.growth(), event);

        self.credits += amount * multiplier as i32;
        self.highest_score = self.highest_score.max(self.credits);

        (event, multiplier)
    }
}
