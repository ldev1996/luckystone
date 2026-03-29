use colored::Colorize;
use std::io::stdin;

pub fn read_valid_bet(max: i32) -> i32 {
    loop {
        let mut input = String::new();

        if stdin().read_line(&mut input).is_err() {
            println!("Failed to read user input, please try again!");
            continue;
        };

        match input.trim().parse::<i32>() {
            Ok(value) if value < 0 => {
                println!(
                    "{}",
                    "⚠ Your bet must be higher than zero!"
                        .to_string()
                        .bright_magenta()
                )
            }
            Ok(value) if value > max => println!(
                "{}",
                format!("⚠ Insufficient credits! You only have {}", max).bright_magenta()
            ),
            Ok(value) => return value,
            Err(_) => println!("{}", "⚠ Please, type a number".bright_magenta()),
        }
    }
}

pub fn stop(msg: &str) {
    println!("{}", msg);
    let mut buffer = String::new();
    stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line.");
}
