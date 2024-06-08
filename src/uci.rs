use std::io::{self, BufRead};

use crate::{
    board::START_POSITION_FEN,
    engine_details::{ENGINE_AUTHOR, ENGINE_NAME, ENGINE_VERSION},
    search::Search,
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
        };
    }
}
