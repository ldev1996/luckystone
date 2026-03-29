use crate::constants::{
    BAD_MULTIPLIERS, CHANCE_CAP, GOOD_MULTIPLIERS, INITIAL_CREDITS, INITIAL_ODDS,
    NORMAL_MULTIPLIERS, WIN_VALUE,
};
use crate::utils::{random_growth, random_multiplier, random_percent};

#[derive(Copy, Clone)]
pub enum Event {
    Jackpot,
    LuckBreak,
    Normal,
}

impl Event {
    pub fn multiplier(self) -> i8 {
        let i = random_multiplier();

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

    pub fn update(&mut self, event: Event) {
        self.good = (self.good + random_growth()).min(CHANCE_CAP);
        self.bad = (self.bad + random_growth()).min(CHANCE_CAP);
        match event {
            Event::Jackpot => self.good = 0,
            Event::LuckBreak => self.bad = 0,
            Event::Normal => (),
        };
    }
}

pub struct Game {
    credits: i32,
    highest_score: i32,
    odds: Odds,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            credits: INITIAL_CREDITS,
            highest_score: INITIAL_CREDITS,
            odds: Odds {
                good: INITIAL_ODDS,
                bad: INITIAL_ODDS,
            },
        }
    }
}

impl Game {
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
        let random_chance = random_percent();
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
        let multiplier = event.multiplier();
        self.odds.update(event);

        self.credits += amount * multiplier as i32;
        self.highest_score = self.highest_score.max(self.credits);

        (event, multiplier)
    }
}
