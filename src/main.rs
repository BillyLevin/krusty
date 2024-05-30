use krusty::{board::Board, perft::run_perft_tests, zobrist_hash::ZobristHasher};

fn main() -> anyhow::Result<()> {
    let hasher = ZobristHasher::new();

    let mut board = Board::default();

    board.parse_fen("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1")?;

    dbg!(hasher.hash_position(&board));

    // let perft_contents = include_str!("../perft.epd");
    // run_perft_tests(perft_contents);

    Ok(())
}
