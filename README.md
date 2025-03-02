# 🎄 Advent of Code 2024 Solutions in Rust 🦀

Welcome to my Advent of Code 2024 solutions repository! This project contains my solutions to the [Advent of Code 2024](https://adventofcode.com/2024) programming puzzles, implemented in Rust.

## 🎯 Current Progress

- [x] Day 1: Location Pairs Calculator
  - Part 1: ✅ Implemented (Calculates total distance between location pairs)
  - Part 2: 🚧 In Progress

## 🚀 Getting Started

### Prerequisites

- Rust (latest stable version)
- Nix (for development environment)

### Running the Solutions

1. Clone this repository:

   ```bash
   git clone <repository-url>
   cd advent-of-code-2024
   ```

2. Set up the development environment:

   ```bash
   direnv allow  # You need to setup direnv and devenv on your machine to run this.
   ```

3. Run a specific day's solution:

   ```bash
   # For Day 1, Part 1
   cd day-01
   cargo run --bin part_one

   # For Day 1, Part 2
   cargo run --bin part_two
   ```

## 📁 Project Structure Overview

```
advent-of-code-2024/
├── day-01/
│   ├── src/
│   │   └── bin/
│   │       ├── part_one.rs
│   │       └── part_two.rs
│   └── Cargo.toml
├── devenv.nix
└── Cargo.toml
```

## 🛠 Development Environment

This project uses:

- Nix with devenv for reproducible development environments
- Rust's Cargo for package management
- Pre-commit hooks for code quality

## 🤝 Contributing

Feel free to:

- Open issues for questions or problems you encounter
- Submit pull requests for improvements
- Share your thoughts on solution approaches

## 📝 License

This project is open source and available under the MIT License.

---

Happy coding! 🎅 Remember, the goal is to learn and have fun while solving these puzzles! 🌟
