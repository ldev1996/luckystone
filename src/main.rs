mod constants;
mod engine;
mod random;
mod ui;

use crate::{
    constants::WIN_VALUE,
    engine::Game,
    random::FastRandom,
    ui::{print_event, print_turn, read_valid_gamble, stop},
};
use colored::Colorize;

fn main() {
    println!("{:^82}", "Lucky Stone".black().on_yellow());
    let mut game = Game::new(FastRandom);

    while !game.has_lost() {
        print_turn(game.credits(), game.odds().good(), game.odds().bad());
        let (event, multiplier) = game.gamble(read_valid_gamble(game.credits()));
        print_event(event, multiplier);
        if game.has_won() {
            println!(
                ">> Congratulations, you won! You reached {} credits!",
                WIN_VALUE
            );
            stop("Press Enter to exit...");
            return;
        }
    }
    println!(
        "{} {}",
        ">> You lost! Highest Credits: ".red(),
        game.highest_score().to_string().red()
    );
    stop("Press Enter to exit...");
}
