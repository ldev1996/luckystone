use std::io::stdin;

pub fn read_number_input() -> i32 {
    let mut input = String::new();

    stdin()
        .read_line(&mut input)
        .expect("Failed to read user input!");
    let input = input.trim();

    input.parse().unwrap_or_default() // TODO fazer algo decente aqui (Result)
}

pub fn stop(msg: &str) {
    println!("{}", msg);
    let mut buffer = String::new();
    stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line.");
}
