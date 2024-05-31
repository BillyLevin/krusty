use krusty::{
    board::Board,
    perft::run_perft_tests,
    square::{Piece, PieceColor, PieceKind},
    zobrist_hash::ZobristHasher,
};

fn main() -> anyhow::Result<()> {
    let mut board = Board::default();

    board.parse_fen("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1")?;

    dbg!(board.hash);

    board.add_piece_and_hash(
        Piece::new(PieceColor::White, PieceKind::Pawn),
        krusty::square::Square::D4,
    )?;

    dbg!(board.hash);

    board.remove_piece_and_hash(krusty::square::Square::D4)?;

    dbg!(board.hash);

    board.switch_side_and_hash();

    dbg!(board.hash);

    board.switch_side_and_hash();

    dbg!(board.hash);

    let perft_contents = include_str!("../perft.epd");
    run_perft_tests(perft_contents);

    Ok(())
}
