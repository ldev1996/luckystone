# Lucky Stone

Lucky Stone is a small command-line game built as a personal learning project in Rust. The goal of the project is to practice language fundamentals, structuring a program, and applying concepts such as enums, pattern matching, and state management.

## 🎮 How the game works

- You start with an initial amount of credits.
- In each round, you choose how many credits to gamble.
- The game randomly determines an event:
  - **Jackpot**: increases your odds of winning in a positive way
  - **Luck Break**: triggers a negative outcome
  - **Normal**: standard outcome
- Each event applies a multiplier that affects your credits.
- The game dynamically adjusts probabilities based on outcomes:
  - Winning events reset their respective chances
  - Other chances gradually increase over time

## 🏁 Win / Lose conditions

- You win if your credits reach the target value.
- You lose if your credits drop to zero.

## 🧠 Purpose of the project

This project was created for learning purposes, with focus on:

- Rust fundamentals
- Ownership and borrowing concepts
- Enums and pattern matching
- Structuring stateful applications
- Handling user input in CLI programs
- Basic game loop design

## 🚧 Status

- Functional prototype
- In development

## ▶️ Running the project

Make sure you have Rust installed, then:

```bash
cargo run
```

## 📌 Notes

This project is not intended to be a production-ready game. It is primarily a sandbox for experimenting with Rust and improving coding practices over time.