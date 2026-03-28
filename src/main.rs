mod constants;
mod random;
mod types;
mod ui;

use crate::types::*;
use colored::Colorize;

fn main() {
    println!("{:^82}", "Lucky Stone".black().on_yellow());
    let mut game = Game::new();

    while game.credits > 0 {
        game.print_turn();
        let gamble_value = ui::read_number_input();
        if !game.is_valid_bet(gamble_value) {
            println!("{}", "Invalid amount!".red());
            continue;
        }
        game.gamble(gamble_value);
        if game.has_won() {
            println!(
                "{} {}",
                ">> You won! Credits: ".yellow(),
                game.highest_score.to_string().yellow()
            );
            ui::stop("Press Enter to exit...");
            return;
        }
    }
    println!(
        "{} {}",
        ">> You lost! Highest Credits: ".red(),
        game.highest_score.to_string().red()
    );
    ui::stop("Press Enter to exit...");
}
