use std::io::{self, BufRead};

use crate::{
    engine_details::{ENGINE_AUTHOR, ENGINE_NAME, ENGINE_VERSION},
    search::Search,
};

pub struct Uci<'a> {
    search: &'a Search,
}

impl<'a> Uci<'a> {
    pub fn new(search: &'a Search) -> Self {
        Self { search }
    }

    pub fn start_loop(&self) {
        Self::handle_uci_command();

        let mut input_buffer = String::new();

        loop {
            io::stdin().lock().read_line(&mut input_buffer).unwrap();
            self.handle_input(&input_buffer);
            input_buffer.clear();
        }
    }

    fn handle_input(&self, input: &str) {
        let input = input.trim();
        let (command, args) = match input.split_once(' ') {
            Some((command, args)) => (command, args),
            None => (input, ""),
        };

        let args = args.trim();

        match command {
            "uci" => Self::handle_uci_command(),
            "isready" => println!("readyok"),
            _ => (),
        };
    }

    fn handle_uci_command() {
        println!("id name {} v{}", ENGINE_NAME, ENGINE_VERSION);
        println!("id author {}", ENGINE_AUTHOR);
        println!("uciok");
    }
}
