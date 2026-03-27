mod config;
mod random;
mod types;

use crate::config::*;
use crate::types::*;
use colored::Colorize;
use std::io::stdin;

fn stop(msg: &str) {
    println!("{}", msg);
    let mut buffer = String::new();
    stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line.");
}

fn print_turn(game: &Game) {
    println!(
        ">> You have {} Credits. Your luck is {}/{}. How much do you want to gamble?",
        format!("{}", game.credits).yellow(),
        format!("{}%", game.odds.good).green(),
        format!("{}%", game.odds.bad).red()
    )
}

fn print_event(event: &Event, multiplier: i8) {
    println!(
        "{}",
        match event {
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

fn read_number_input() -> i32 {
    let mut input = String::new();

    stdin()
        .read_line(&mut input)
        .expect("Failed to read user input!");
    let input = input.trim();

    input.parse().unwrap_or_default() // TODO fazer algo decente aqui (Result)
}

fn roll_event(game: &Game) -> Event {
    let random_chance = random::random_percent();
    if random_chance <= game.odds.good {
        Event::Jackpot
    } else if random_chance > 100 - game.odds.bad {
        Event::LuckBreak
    } else {
        Event::Normal
    }
}

fn roll_multiplier(event: &Event) -> i8 {
    let random_multiplier = random::random_multiplier();
    match event {
        Event::Jackpot => GOOD_MULTIPLIERS[random_multiplier],
        Event::LuckBreak => BAD_MULTIPLIERS[random_multiplier],
        Event::Normal => NORMAL_MULTIPLIERS[random_multiplier],
    }
}

fn update_odds(game: &mut Game, event: &Event) {
    game.odds.good = (game.odds.good + random::random_growth()).min(CHANCE_CAP);
    game.odds.bad = (game.odds.bad + random::random_growth()).min(CHANCE_CAP);
    match event {
        Event::Jackpot => {
            game.odds.good = 0;
        }
        Event::LuckBreak => {
            game.odds.bad = 0;
        }
        Event::Normal => (),
    };
}

fn is_valid_bet(game: &Game, value: i32) -> bool {
    value > 0 && value <= game.credits
}

impl Game {
    fn gamble(&mut self, amount: i32) {
        let event = roll_event(self);
        let multiplier = roll_multiplier(&event);
        print_event(&event, multiplier);
        update_odds(self, &event);

        self.credits += amount * multiplier as i32;
        self.highest_score = self.highest_score.max(self.credits);
    }
}

fn main() {
    println!("{:^82}", " Lucky Stone ".black().on_yellow());
    let mut game = Game {
        credits: INITIAL_VALUE,
        highest_score: INITIAL_VALUE,
        odds: Odds {
            good: INITIAL_CHANCE,
            bad: INITIAL_CHANCE,
        },
    };

    while game.credits > 0 {
        print_turn(&game);
        let gamble_value = read_number_input();
        if !is_valid_bet(&game, gamble_value) {
            println!("{}", "Invalid amount!".red());
            continue;
        }
        game.gamble(gamble_value);
        if game.credits >= WIN_VALUE {
            println!(
                "{} {}",
                ">> You won! Credits: ".yellow(),
                game.highest_score.to_string().yellow()
            );
            stop("Press Enter to exit...");
            return;
        }
    }
    println!(
        "{} {}",
        ">> You lost! Highest Credits: ".red(),
        game.highest_score.to_string().red()
    );
    stop("Press Enter to exit...");
}
