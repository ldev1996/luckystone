use colored::Colorize;
use std::io::stdin;

use crate::engine::{Event, GambleOutcome};

pub fn read_gamble() -> Option<i32> {
    let mut input = String::new();
    if stdin().read_line(&mut input).is_err() {
        return None;
    }

    input.trim().parse::<i32>().ok()
}

pub fn print_title() {
    println!("{:^82}", "Lucky Stone".black().on_yellow());
}

pub fn print_event(outcome: GambleOutcome) {
    println!(
        "{}",
        match outcome.event() {
            Event::Jackpot => " JACKPOT! ".black().on_green(),
            Event::LuckBreak => " LUCK BREAK! ".black().on_red(),
            Event::Normal if outcome.is_positive() => "GOOD LUCK!".green(),
            Event::Normal => "BAD LUCK!".red(),
        }
    );
}

pub fn print_turn(credits: i32, jackpot_odds: u8, luck_break_odds: u8) {
    println!(
        ">> You have {} Credits. Your luck is {}/{}. How much do you want to gamble?",
        credits.to_string().yellow(),
        format!("{}%", jackpot_odds).green(),
        format!("{}%", luck_break_odds).red()
    )
}

pub fn print_won(value: i32) {
    println!(
        "{} {} {}",
        ">> Congratulations, you won! You reached".yellow(),
        value.to_string().yellow(),
        "credits!".yellow()
    );
}

pub fn print_loss(highest_score: i32) {
    println!(
        "{} {}",
        ">> You lost! Highest Credits:".red(),
        highest_score.to_string().red()
    );
}

pub fn print_error(msg: impl std::fmt::Display) {
    println!("{}", msg.to_string().bright_magenta());
}

pub fn stop() {
    println!("Press Enter to exit...");
    let mut _pause = String::new();
    let _ = stdin().read_line(&mut _pause);
}
