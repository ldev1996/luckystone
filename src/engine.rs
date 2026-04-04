use crate::constants::{
    BAD_MULTIPLIERS, CHANCE_CAP, GOOD_MULTIPLIERS, INITIAL_CREDITS, INITIAL_ODDS,
    NORMAL_MULTIPLIERS, WIN_VALUE,
};
use crate::random::RandomProvider;

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

#[derive(Copy, Clone)]
pub struct Odds {
    good: u8,
    bad: u8,
}

impl Odds {
    pub fn good(&self) -> u8 {
        self.good
    }

    pub fn bad(&self) -> u8 {
        self.bad
    }

    pub fn update(&mut self, good: u8, bad: u8, event: Event) {
        self.good = (self.good + good).min(CHANCE_CAP);
        self.bad = (self.bad + bad).min(CHANCE_CAP);
        match event {
            Event::Jackpot => self.good = 0,
            Event::LuckBreak => self.bad = 0,
            Event::Normal => (),
        };
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
            odds: Odds {
                good: INITIAL_ODDS,
                bad: INITIAL_ODDS,
            },
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
        if random_chance <= self.odds.good {
            Event::Jackpot
        } else if random_chance > 100 - self.odds.bad {
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
