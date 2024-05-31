use std::io::{self, BufRead, Write};

use colored::Colorize;

use crate::{board::Board, perft::run_perft_tests};

const START_POSITION_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

#[derive(Default)]
pub struct CLI {
    board: Board,
}

impl CLI {
    pub fn start_loop(&mut self) {
        Self::print_greeting();

        let mut input_buffer = String::new();

        loop {
            print!("{} ", "krusty>".color("orange"));
            io::stdout().flush().unwrap();

            io::stdin().lock().read_line(&mut input_buffer).unwrap();
            self.handle_input(&input_buffer);
            input_buffer.clear();
        }
    }

    fn print_greeting() {
        println!("\n\nWelcome to {}", "Krusty!".color("orange").bold());
        println!("A chess engine written in Rust \u{1F980}");

        println!("\nCommands:");
        println!("- {}: run full perft suite", "perft".cyan());
        println!("- {}: load FEN", "fen [<FEN> | startpos>]".cyan());

        println!();
    }

    fn handle_input(&mut self, input: &str) {
        let input = input.trim();
        let (command, args) = match input.split_once(' ') {
            Some((command, args)) => (command, args),
            None => (input, ""),
        };

        match command {
            "perft" => run_perft_tests(include_str!("../perft.epd")),
            "fen" => self.handle_fen_command(args),
            _ => println!("Invalid command"),
        };
    }

    fn handle_fen_command(&mut self, args: &str) {
        let args = args.trim();
        if args.is_empty() {
            println!("Invalid FEN");
            return;
        }

        let fen = match args {
            "startpos" => START_POSITION_FEN,
            _ => args,
        };

        if self.board.parse_fen(fen).is_err() {
            println!("Invalid FEN");
        }
    }
}
