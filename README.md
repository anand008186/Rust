# Rust
## Prerequisites
Before running the examples and the guessing game, make sure you have Rust and Cargo installed on your local machine. If you haven't installed them yet, follow the steps below:

1. Visit the official Rust website at [https://www.rust-lang.org/](https://www.rust-lang.org/).
2. Click on the "Install" button to download the Rust installation package for your operating system.
3. Run the installation package and follow the instructions provided by the installer.
4. Once the installation is complete, open a new terminal window and verify that Rust is installed by running the following command:
    ```shell
    rustc --version
    ```
    You should see the version number of Rust printed on the screen.
5. Next, verify that Cargo is installed by running the following command:
    ```shell
    cargo --version
    ```
    You should see the version number of Cargo printed on the screen.

This repository contains basic examples and a guessing game implemented in Rust.

## Folder Structure

- [Basics](./basics): This folder contains examples of variables, functions, data types, and control flow in Rust.

- [Guessing Game](./guessing-game): This folder contains a guessing game implemented in Rust. In this game, you have to guess a number between 1 and 100 until you guess the correct number.

## Basics

The `basics` folder contains examples that cover the following topics:

- Variables: How to declare and use variables in Rust.

- Functions: How to define and call functions in Rust.

- Data Types: An overview of the basic data types available in Rust.

- Control Flow: How to use if-else statements, loops, and match expressions in Rust.

## Guessing Game

The `guessing-game` folder contains a simple guessing game implemented in Rust. The game uses the `rand` crate to generate a random number between 1 and 100. You have to guess the number until you guess the correct one.

To run the guessing game, navigate to the `guessing-game` folder and execute the following command:
cargo run
