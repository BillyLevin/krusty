use std::io::{self, BufRead, Write};

use colored::Colorize;

use crate::{
    board::Board,
    perft::{perft, run_perft_tests},
    transposition_table::TranspositionTable,
};

const START_POSITION_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

pub struct CLI {
    board: Board,
    transposition_table: TranspositionTable,
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

        Self::print_commands();
    }

    fn print_commands() {
        println!();

        println!("Commands:");
        println!("- {}: run full perft suite", "perft [<depth>]".cyan());
        println!("- {}: load FEN", "fen <FEN> | startpos".cyan());
        println!("- {}: print current position", "print".cyan());
        println!("- {}: print this command list", "help".cyan());

        println!();
    }

    fn handle_input(&mut self, input: &str) {
        let input = input.trim();
        let (command, args) = match input.split_once(' ') {
            Some((command, args)) => (command, args),
            None => (input, ""),
        };

        let args = args.trim();

        match command {
            "perft" => self.handle_perft_command(args),
            "fen" => self.handle_fen_command(args),
            "print" => println!("{}", self.board),
            "help" => Self::print_commands(),
            _ => println!("Invalid command"),
        };
    }

    fn handle_perft_command(&mut self, args: &str) {
        if args.is_empty() {
            run_perft_tests(include_str!("../perft.epd"), None);
            return;
        }

        let depth: u8 = match args.parse() {
            Ok(value) => value,
            Err(_) => {
                println!("Depth must be an integer");
                return;
            }
        };

        let nodes = perft(&mut self.board, depth, &mut self.transposition_table).unwrap();
        println!("nodes: {}", nodes);
    }

    fn handle_fen_command(&mut self, args: &str) {
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

impl Default for CLI {
    fn default() -> Self {
        Self {
            board: Board::default(),
            transposition_table: TranspositionTable::new(64),
        }
    }
}
