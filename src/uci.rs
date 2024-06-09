use std::io::{self, BufRead};

use anyhow::{bail, Context};

use crate::{
    board::{Side, START_POSITION_FEN},
    engine_details::{ENGINE_AUTHOR, ENGINE_NAME, ENGINE_VERSION},
    search::{Search, SearchDepth, SearchInfo, TimeRemaining},
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
            self.handle_input(&input_buffer);
            input_buffer.clear();
        }
    }

    fn handle_input(&mut self, input: &str) {
        let input = input.trim();
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
            _ => (),
        };
    }

    fn handle_uci_command() {
        println!("id name {} v{}", ENGINE_NAME, ENGINE_VERSION);
        println!("id author {}", ENGINE_AUTHOR);
        println!("uciok");
    }

    fn handle_position_command(&mut self, args: &str) {
        let moves_start_index = args.find("moves");

        let fen = match moves_start_index {
            Some(index) => &args[..index],
            None => args,
        }
        .trim();

        let fen = match fen {
            "startpos" => START_POSITION_FEN,
            _ => fen,
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

        let mut search_info = SearchInfo::default();

        while let Some(arg) = args.next() {
            match arg {
                "depth" => {
                    let depth: SearchDepth = match args.next().try_into() {
                        Ok(depth) => depth,
                        Err(error) => {
                            println!("{}", error);
                            return;
                        }
                    };

                    search_info.depth = depth;
                }
                "wtime" => {
                    if self.search.board.side_to_move() == Side::White {
                        let time_remaining = match args.next().try_into() {
                            Ok(time) => time,
                            Err(error) => {
                                println!("{}", error);
                                return;
                            }
                        };

                        search_info.time_remaining = time_remaining;
                    }
                }
                "btime" => {
                    if self.search.board.side_to_move() == Side::Black {
                        let time_remaining = match args.next().try_into() {
                            Ok(time) => time,
                            Err(error) => {
                                println!("{}", error);
                                return;
                            }
                        };

                        search_info.time_remaining = time_remaining;
                    }
                }
                _ => (),
            }
        }

        self.search.set_search_info(search_info);

        let best_move = match self.search.search_position(search_info.depth) {
            Ok(mv) => match mv {
                Some(mv) => mv,
                None => {
                    // TODO: better handling for null moves
                    println!("0000");
                    return;
                }
            },
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

impl TryFrom<Option<&str>> for TimeRemaining {
    type Error = anyhow::Error;

    fn try_from(time: Option<&str>) -> Result<Self, Self::Error> {
        if let Some(time) = time {
            let time = time.parse().context("invalid time provided")?;
            Ok(TimeRemaining::Finite(time))
        } else {
            bail!("no time provided")
        }
    }
}
