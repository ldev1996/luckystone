# 💎 Lucky Stone

**Lucky Stone** is a small command-line gambling game written in Rust.
The project was created as a learning exercise and has gone through several refactoring iterations while exploring Rust design patterns and program structure.

The game runs entirely in the terminal and revolves around managing credits and betting on random outcomes whose probabilities gradually change over time.

## 🎮 Gameplay

The player starts with a fixed amount of credits and chooses how many to gamble each turn.

For every bet, the game rolls one of three possible events:

| Event	     | Description                                                       |
|:-----------|:------------------------------------------------------------------|
| Jackpot    | Applies a positive multiplier and resets the jackpot probability  |
| Luck Break | Applies a negative multiplier and resets the bad luck probability |
| Normal     | Applies a small positive or negative multiplier                   |

Each event applies a multiplier to the bet:

`credits += bet * multiplier`

After every round:

- The probability of **Jackpot** and **Luck Break** gradually increases
- When one of these events occurs, its probability is reset

This creates a shifting risk/reward dynamic over time.

### 🏁 Win / Lose conditions

- **Win:** reach the target credit value
- **Lose:** credits drop to zero or below

## 🎯 Project Goals

The purpose of the project is educational.
It focuses on practicing:
- basic Rust syntax and idioms
- struct and enum based state modeling
- separating game logic from input/output
- modularizing a small CLI program
- iterative refactoring
- software engineering

The codebase evolved through several rewrites as the program structure improved.

## ▶️ Running the project

Make sure you have Rust installed, then:

```bash
cargo run
```

## 📌 Notes

- This project is not intended to be a production-ready game. It is primarily a sandbox for experimenting with Rust and improving coding practices over time.
- The project is intentionally over-engineered in places because its primary goal is to explore Rust patterns and program structure.