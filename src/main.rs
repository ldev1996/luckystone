mod constants;
mod random;
mod types;
mod ui;

use crate::{constants::WIN_VALUE, types::Game};
use colored::Colorize;

fn main() {
    println!("{:^82}", "Lucky Stone".black().on_yellow());
    let mut game = Game::default();

    while !game.has_lost() {
        game.print_turn();
        game.gamble(ui::read_valid_bet(game.credits()));
        if game.has_won() {
            println!(
                ">> Congratulations, you won! You reached {} credits!",
                WIN_VALUE
            );
            ui::stop("Press Enter to exit...");
            return;
        }
    }
    println!(
        "{} {}",
        ">> You lost! Highest Credits: ".red(),
        game.highest_score().to_string().red()
    );
    ui::stop("Press Enter to exit...");
}
