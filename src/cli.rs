use std::io::{self, BufRead, Write};

use colored::Colorize;

use crate::perft::run_perft_tests;

pub struct CLI;

impl CLI {
    pub fn start_loop() {
        Self::print_greeting();

        let mut input_buffer = String::new();

        loop {
            print!("{} ", "krusty>".color("orange").bold());
            io::stdout().flush().unwrap();

            io::stdin().lock().read_line(&mut input_buffer).unwrap();
            Self::handle_input(&input_buffer);
            input_buffer.clear();
        }
    }

    fn print_greeting() {
        println!("\n\nWelcome to {}", "Krusty!".color("orange").bold());
        println!("A chess engine written in Rust \u{1F980}");

        println!("\nCommands:");
        println!("- {}: run full perft suite", "perft".cyan().bold());

        println!();
    }

    fn handle_input(input: &str) {
        let parts: Vec<_> = input.trim().split(' ').collect();

        if *parts.first().unwrap() == "perft" {
            run_perft_tests(include_str!("../perft.epd"));
        } else {
            println!("Invalid command")
        };
    }
}
