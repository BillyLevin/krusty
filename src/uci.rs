use std::io::{self, BufRead};

use crate::{
    engine_details::{ENGINE_AUTHOR, ENGINE_NAME, ENGINE_VERSION},
    search::Search,
};

#[derive(Default)]
pub struct Uci {
    search: Search,
}

impl Uci {
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
        todo!()
    }

    fn handle_uci_command() {
        println!("id name {} v{}", ENGINE_NAME, ENGINE_VERSION);
        println!("id author {}", ENGINE_AUTHOR);
        println!("uciok");
    }
}
