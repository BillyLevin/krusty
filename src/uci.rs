use std::io::{self, BufRead};

use anyhow::{bail, Context};

use crate::{
    board::{Side, START_POSITION_FEN},
    engine_details::{ENGINE_AUTHOR, ENGINE_NAME, ENGINE_VERSION},
    search::{Search, SearchDepth},
};

pub struct Uci<'a> {
    search: &'a mut Search,
}

impl<'a> Uci<'a> {
    pub fn new(search: &'a mut Search) -> Self {
        Self { search }
    }

    pub fn start_loop(&mut self) {
        Self::handle_uci_command();

        let mut input_buffer = String::new();

        loop {
            io::stdin().lock().read_line(&mut input_buffer).unwrap();

            let input = input_buffer.trim();
            let (command, args) = match input.split_once(' ') {
                Some((command, args)) => (command, args),
                None => (input, ""),
            };

            let args = args.trim();

            match command {
                "uci" => Self::handle_uci_command(),
                "isready" => println!("readyok"),
                "position" => self.handle_position_command(args),
                "go" => self.handle_go_command(args),
                "quit" => {
                    self.search.reset();
                    break;
                }
                _ => (),
            };

            input_buffer.clear();
        }
    }

    fn handle_uci_command() {
        println!("id name {} v{}", ENGINE_NAME, ENGINE_VERSION);
        println!("id author {}", ENGINE_AUTHOR);
        println!("uciok");
    }

    // possible examples:
    // position startpos
    // position fen <fen>
    // position startpos moves e2e4 e7e5 ...
    // position fen <fen> moves e2e4 e7e5 ...
    fn handle_position_command(&mut self, args: &str) {
        let position_kind = match args.split_whitespace().nth(0) {
            Some(kind) => kind,
            None => {
                println!("Invalid `position` command");
                return;
            }
        };

        let moves_start_index = args.find("moves");

        let fen = match position_kind {
            "startpos" => START_POSITION_FEN,
            "fen" => {
                let fen_start_index = "fen".len() + 1;
                match moves_start_index {
                    Some(index) => &args[fen_start_index..(index - 1)],
                    None => &args[fen_start_index..],
                }
            }
            _ => {
                println!("Invalid `position` command");
                return;
            }
        };

        if self.search.board.parse_fen(fen).is_err() {
            println!("Invalid FEN");
            return;
        }

        if let Some(index) = moves_start_index {
            let start_index = index + "moves ".len();
            let moves: Result<Vec<_>, _> = args[start_index..]
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
    }

    fn handle_go_command(&mut self, args: &str) {
        let mut args = args.split_whitespace();

        let mut time_remaining = None;
        let mut increment = 0;
        let mut max_depth = SearchDepth::Infinite;
        let mut moves_to_go = None;

        while let Some(arg) = args.next() {
            match arg {
                "depth" => {
                    max_depth = match args.next().try_into() {
                        Ok(depth) => depth,
                        Err(error) => {
                            println!("{}", error);
                            return;
                        }
                    };
                }
                "wtime" => {
                    if self.search.board.side_to_move() == Side::White {
                        time_remaining = match args.next() {
                            Some(time) => time.parse::<u128>().ok(),
                            None => {
                                println!("missing wtime value");
                                return;
                            }
                        };
                    }
                }
                "btime" => {
                    if self.search.board.side_to_move() == Side::Black {
                        time_remaining = match args.next() {
                            Some(time) => time.parse::<u128>().ok(),
                            None => {
                                println!("missing btime value");
                                return;
                            }
                        };
                    }
                }
                "winc" => {
                    if self.search.board.side_to_move() == Side::White {
                        increment = match args.next() {
                            Some(time) => time.parse().unwrap_or(0),
                            None => {
                                println!("missing winc value");
                                return;
                            }
                        }
                    }
                }
                "binc" => {
                    if self.search.board.side_to_move() == Side::Black {
                        increment = match args.next() {
                            Some(time) => time.parse().unwrap_or(0),
                            None => {
                                println!("missing binc value");
                                return;
                            }
                        }
                    }
                }
                "movestogo" => {
                    moves_to_go = match args.next() {
                        Some(moves) => moves.parse().ok(),
                        None => {
                            println!("missing movestogo values");
                            return;
                        }
                    }
                }
                _ => (),
            }
        }

        self.search.max_depth = max_depth.into();

        self.search
            .timer
            .initialize(time_remaining, increment, moves_to_go);

        self.search.search_info.nodes_searched = 0;

        self.search.timer.start();

        let best_move = match self.search.search_position() {
            Ok(mv) => mv,
            Err(error) => {
                println!("{}", error);
                return;
            }
        };

        println!("bestmove {}", best_move);
    }
}

impl TryFrom<Option<&str>> for SearchDepth {
    type Error = anyhow::Error;

    fn try_from(depth: Option<&str>) -> Result<Self, Self::Error> {
        if let Some(depth) = depth {
            let depth = depth.parse().context("invalid depth provided")?;
            Ok(SearchDepth::Finite(depth))
        } else {
            bail!("no depth provided")
        }
    }
}
