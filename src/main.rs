use krusty::{board::Board, cli::CLI, search::Search};

fn main() -> anyhow::Result<()> {
    let mate_in_2 = "4kb1r/p2n1ppp/4q3/4p1B1/4P3/1Q6/PPP2PPP/2KR4 w k - 0 16";
    let mut board = Board::default();
    board.parse_fen(mate_in_2)?;

    let search = Search {};
    dbg!(search.search_position(&mut board, 4)?);

    // let mut cli = CLI::default();
    // cli.start_loop();

    Ok(())
}
