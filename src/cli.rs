use std::io::{self, BufRead, Write};

use anyhow::bail;
use colored::Colorize;

use crate::{
    board::Board,
    evaluate::evaluate,
    move_generator::{Move, MoveKind, MoveList},
    perft::{perft, run_perft_tests},
    square::{PieceKind, Square},
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
            "moves" | "mv" => self.handle_moves_command(args),
            "eval" => self.handle_eval_command(),
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

    fn handle_moves_command(&mut self, args: &str) {
        if args.is_empty() {
            println!("No moves provided");
            return;
        }

        let moves: Result<Vec<_>, _> = args
            .split(' ')
            .map(|move_str| {
                if move_str.len() < 4 {
                    bail!("Move string `{}` is invalid", move_str)
                }

                let mut chars = move_str.chars();
                let from_file = chars.next().unwrap().try_into()?;
                let from_rank = chars.next().unwrap().try_into()?;
                let to_file = chars.next().unwrap().try_into()?;
                let to_rank = chars.next().unwrap().try_into()?;

                let from = Square::new(from_rank, from_file);
                let to = Square::new(to_rank, to_file);

                let promotion: Option<PieceKind> = match chars.next() {
                    Some(piece) => Some(piece.try_into()?),
                    None => None,
                };

                Ok((from, to, promotion))
            })
            .collect();

        let moves = match moves {
            Ok(m) => m,
            Err(error) => {
                println!("Invalid move: {}", error);
                return;
            }
        };

        for mv in moves {
            let found_move = self.find_matching_move(mv);

            match found_move {
                Some(mv) => {
                    let is_legal = self.board.make_move(mv).unwrap();
                    if !is_legal {
                        println!("Move `{:?}` is not legal in this position", mv);
                        self.board.unmake_move(mv).unwrap();
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

    fn find_matching_move(
        &self,
        (from, to, promotion_piece): (Square, Square, Option<PieceKind>),
    ) -> Option<Move> {
        let mut possible_moves = MoveList::default();
        self.board.generate_all_moves(&mut possible_moves).unwrap();

        for possible_move in possible_moves {
            if possible_move.from_square() == from && possible_move.to_square() == to {
                if possible_move.kind() == MoveKind::Promotion {
                    let promotion_piece = promotion_piece.expect("promotion piece missing");
                    if promotion_piece == possible_move.flag().into() {
                        return Some(possible_move);
                    }
                } else {
                    return Some(possible_move);
                }
            }
        }

        None
    }

    fn handle_eval_command(&self) {
        println!("{}", evaluate(&self.board))
    }
}

impl Default for CLI {
    fn default() -> Self {
        let mut board = Board::default();
        board.parse_fen(START_POSITION_FEN).unwrap();

        Self {
            board,
            transposition_table: TranspositionTable::new(64),
        }
    }
}
