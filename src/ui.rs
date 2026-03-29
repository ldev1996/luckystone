use colored::Colorize;
use std::io::stdin;

use crate::types::{Event, Game};

pub fn read_valid_gamble(max: i32) -> i32 {
    loop {
        let mut input = String::new();

        if stdin().read_line(&mut input).is_err() {
            println!("Failed to read user input, please try again!");
            continue;
        };

        match input.trim().parse::<i32>() {
            Ok(value) if value <= 0 => {
                println!("{}", "⚠ You must gamble more than zero!".bright_magenta())
            }
            Ok(value) if value > max => {
                println!("{}", "⚠ Insufficient credits!".bright_magenta())
            }
            Ok(value) => return value,
            Err(_) => println!("{}", "⚠ Please, type a number".bright_magenta()),
        }
    }
}

pub fn print_event(event: Event, multiplier: i8) {
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

pub fn print_turn(game: &Game) {
    println!(
        ">> You have {} Credits. Your luck is {}/{}. How much do you want to gamble?",
        format!("{}", game.credits()).yellow(),
        format!("{}%", game.odds().good()).green(),
        format!("{}%", game.odds().bad()).red()
    )
}

pub fn stop(msg: &str) {
    println!("{}", msg);
    let mut buffer = String::new();
    stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line!");
}
