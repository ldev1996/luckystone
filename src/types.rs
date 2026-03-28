use crate::constants::*;
use crate::random;
use colored::Colorize;

#[derive(Copy, Clone)]
pub enum Event {
    Jackpot,
    LuckBreak,
    Normal,
}

impl Event {
    pub fn multiplier(self) -> i8 {
        let i = random::random_multiplier();

        match self {
            Event::Jackpot => GOOD_MULTIPLIERS[i],
            Event::LuckBreak => BAD_MULTIPLIERS[i],
            Event::Normal => NORMAL_MULTIPLIERS[i],
        }
    }

    pub fn print(self, multiplier: i8) {
        println!(
            "{}",
            match self {
                Event::Jackpot => " JACKPOT! ".black().on_green(),
                Event::LuckBreak => " LUCK BREAK! ".black().on_red(),
                Event::Normal =>
                    if multiplier > 0 {
                        "GOOD LUCK!".green()
                    } else {
                        "BAD LUCK!".red()
                    },
            }
        );
    }
}

#[derive(Copy, Clone)]
pub struct Odds {
    pub good: u8,
    pub bad: u8,
}

impl Odds {
    pub fn update(&mut self, event: Event) {
        self.good = (self.good + random::random_growth()).min(CHANCE_CAP);
        self.bad = (self.bad + random::random_growth()).min(CHANCE_CAP);
        match event {
            Event::Jackpot => self.good = 0,
            Event::LuckBreak => self.bad = 0,
            Event::Normal => (),
        };
    }
}

#[derive(Copy, Clone)]
pub struct Game {
    pub credits: i32,
    pub highest_score: i32,
    pub odds: Odds,
}

impl Game {
    pub fn new() -> Self {
        Self {
            credits: INITIAL_CREDITS,
            highest_score: INITIAL_CREDITS,
            odds: Odds {
                good: INITIAL_ODDS,
                bad: INITIAL_ODDS,
            },
        }
    }

    pub fn roll_event(self) -> Event {
        let random_chance = random::random_percent();
        if random_chance <= self.odds.good {
            Event::Jackpot
        } else if random_chance > 100 - self.odds.bad {
            Event::LuckBreak
        } else {
            Event::Normal
        }
    }

    pub fn print_turn(self) {
        println!(
            ">> You have {} Credits. Your luck is {}/{}. How much do you want to gamble?",
            format!("{}", self.credits).yellow(),
            format!("{}%", self.odds.good).green(),
            format!("{}%", self.odds.bad).red()
        )
    }

    pub fn is_valid_bet(self, value: i32) -> bool {
        value > 0 && value <= self.credits
    }

    pub fn has_won(self) -> bool {
        self.credits >= WIN_VALUE
    }

    pub fn gamble(&mut self, amount: i32) {
        let event = self.roll_event();
        let multiplier = event.multiplier();
        event.print(multiplier);
        self.odds.update(event);

        self.credits += amount * multiplier as i32;
        self.highest_score = self.highest_score.max(self.credits);
    }
}
