mod constants;
mod engine;
mod ui;

use crate::{
    constants::WIN_VALUE,
    engine::{Game, rng::FastRandom},
    ui::*,
};

fn main() {
    print_title();
    let mut game = Game::new(FastRandom);

    while !game.has_lost() {
        print_turn(
            game.credits(),
            game.odds().jackpot(),
            game.odds().luck_break(),
        );

        let amount = loop {
            match read_gamble().and_then(|v| game.validate_bet(v)) {
                Ok(v) => break v,
                Err(msg) => print_error(msg),
            }
        };

        print_event(game.gamble(amount));

        if game.has_won() {
            print_won(WIN_VALUE);
            stop();
            return;
        }
    }
    print_loss(game.highest_score());
    stop();
}
