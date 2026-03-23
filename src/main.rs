use colored::Colorize;
use std::io::stdin;

// TODO melhorar validação de input

const MULTIPLIERS_LENGTH: usize = 7;
const NORMAL_MULTIPLIERS: [i8; MULTIPLIERS_LENGTH] = [-5, -3, -2, 1, 2, 3, 5];
const GOOD_MULTIPLIERS: [i8; MULTIPLIERS_LENGTH] = [2, 2, 2, 3, 3, 5, 5];
const BAD_MULTIPLIERS: [i8; MULTIPLIERS_LENGTH] = [-2, -2, -2, -3, -3, -5, -5];

const INITIAL_CHANCE: u8 = 1;
const INITIAL_VALUE: i32 = 20;
const WIN_VALUE: i32 = 1_000_000;

const CHANCE_GROW_MIN: u8 = 3;
const CHANCE_GROW_MAX: u8 = 7;
const CHANCE_CAP: u8 = 50;

enum Event {
    Jackpot,
    LuckBreak,
    Normal,
}

fn stop(msg: &str) {
    println!("{}", msg);
    let mut buffer = String::new();
    stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line.");
}

struct Game {
    credits: i32,
    highest_score: i32,
    good_chance: u8,
    bad_chance: u8,
}

impl Game {
    fn ask_for_gamble(&self) -> i32 {
        println!(
            ">> You have {} Credits. Your luck is {}/{}. How much do you want to gamble?",
            format!("{}", self.credits).yellow(),
            format!("{}%", self.good_chance).green(),
            format!("{}%", self.bad_chance).red()
        );
        let mut input = String::new();

        stdin()
            .read_line(&mut input)
            .expect("Failed to read user input!");
        let input = input.trim();

        input.parse().unwrap_or_default() // TODO fazer algo decente aqui (Result)
    }

    // TODO dividir, talvez
    /// Receives a amount.
    /// 1. Decides the event randomly.
    /// 2. Decides the multiplier based on the event.
    /// 3. Prints the event message.
    /// 4. Updates the odds of each event.
    /// 5. Updates the credits and high score for the game.
    fn gamble(&mut self, amount: i32) {
        let event = {
            let random_chance = fastrand::u8(1..=100);
            if random_chance <= self.good_chance {
                Event::Jackpot
            } else if random_chance > 100 - self.bad_chance {
                Event::LuckBreak
            } else {
                Event::Normal
            }
        };

        let multiplier = {
            let random_multiplier = fastrand::usize(..MULTIPLIERS_LENGTH);
            match event {
                Event::Jackpot => GOOD_MULTIPLIERS[random_multiplier],
                Event::LuckBreak => BAD_MULTIPLIERS[random_multiplier],
                Event::Normal => NORMAL_MULTIPLIERS[random_multiplier],
            }
        };

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

        self.good_chance += fastrand::u8(CHANCE_GROW_MIN..CHANCE_GROW_MAX).min(CHANCE_CAP);
        self.bad_chance += fastrand::u8(CHANCE_GROW_MIN..CHANCE_GROW_MAX).min(CHANCE_CAP);
        match event {
            Event::Jackpot => {
                self.good_chance = 0;
            }
            Event::LuckBreak => {
                self.bad_chance = 0;
            }
            Event::Normal => (),
        };

        self.credits += amount * multiplier as i32;
        self.highest_score = self.highest_score.max(self.credits);
    }
}

fn main() {
    println!("{:^82}", " Lucky Stone ".black().on_yellow());
    let mut game = Game {
        credits: INITIAL_VALUE,
        highest_score: INITIAL_VALUE,
        good_chance: INITIAL_CHANCE,
        bad_chance: INITIAL_CHANCE,
    };

    while game.credits > 0 {
        let gamble_value = game.ask_for_gamble();
        if gamble_value <= 0 || gamble_value > game.credits {
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
