use std::io::{self, BufRead, Write};

use colored::Colorize;

use crate::{
    board::START_POSITION_FEN,
    evaluate::evaluate,
    perft::{perft, run_perft_tests},
    search::Search,
    transposition_table::{PerftTableEntry, TranspositionTable},
    uci::Uci,
};

pub struct Cli {
    transposition_table: TranspositionTable<PerftTableEntry>,
    search: Search,
}

impl Cli {
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
        println!(
            "- {}: make moves on board",
            "moves <move1> <move2> ... ".cyan()
        );
        println!(
            "- {}: alias for `moves` command",
            "mv <move1> <move2> ... ".cyan()
        );
        println!(
            "- {}: print evaluation of position relative to current side",
            "eval".cyan()
        );
        println!(
            "- {}: print best move after searching at given depth",
            "search <depth>".cyan()
        );
        println!("- {}: print current position", "print".cyan());
        println!("- {}: start UCI protocol", "uci".cyan());
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
            "moves" | "mv" => self.handle_moves_command(args),
            "eval" => self.handle_eval_command(),
            "search" => self.handle_search_command(args),
            "print" => println!("{}", self.search.board),
            "uci" => self.handle_uci_command(),
            "help" => Self::print_commands(),
            _ => println!("Invalid command"),
        };
    }

    fn handle_perft_command(&mut self, args: &str) {
        if args.is_empty() {
            run_perft_tests(include_str!("../perft.epd"), &mut self.transposition_table);
            return;
        }

        let depth: u8 = match args.parse() {
            Ok(value) => value,
            Err(_) => {
                println!("Depth must be an integer");
                return;
            }
        };

        let nodes = perft(&mut self.search.board, depth, &mut self.transposition_table).unwrap();
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

        if self.search.board.parse_fen(fen).is_err() {
            println!("Invalid FEN");
        }
    }

    fn handle_moves_command(&mut self, args: &str) {
        if args.is_empty() {
            println!("No moves provided");
            return;
        }

        let moves: Result<Vec<_>, _> = args
            .split_whitespace()
            .map(|move_str| self.search.board.get_move_metadata(move_str))
            .collect();

        let moves = match moves {
            Ok(m) => m,
            Err(error) => {
                println!("Invalid move: {}", error);
                return;
            }
        };

        for mv in moves {
            let found_move = self.search.board.find_matching_move(mv);

            match found_move {
                Some(mv) => {
                    let is_legal = self.search.board.make_move(mv).unwrap();
                    if !is_legal {
                        println!("Move `{:?}` is not legal in this position", mv);
                        self.search.board.unmake_move(mv).unwrap();
                        return;
                    }
                }
                None => {
                    println!("Move `{:?}` is not legal in this position", mv);
                    return;
                }
            }
        }
    }

    fn handle_eval_command(&self) {
        println!("{}", evaluate(&self.search.board))
    }

    fn handle_search_command(&mut self, args: &str) {
        if args.is_empty() {
            println!("Please provide a search depth");
            return;
        }

        let depth: u8 = match args.parse() {
            Ok(value) => value,
            Err(_) => {
                println!("Depth must be an integer");
                return;
            }
        };

        self.search.max_depth = depth;

        let best_move = self.search.search_position().unwrap();

        println!("{}", best_move);
    }

    fn handle_uci_command(&mut self) {
        let mut uci = Uci::new(&mut self.search);
        uci.start_loop();
    }
}

impl Default for Cli {
    fn default() -> Self {
        Self {
            transposition_table: TranspositionTable::new(256),
            search: Search::default(),
        }
    }
}
