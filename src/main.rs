use krusty::{
    board::Board,
    move_generator::{Move, MoveFlag, MoveKind},
    square::Square,
};

const START_POSITION_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
const OPERA_GAME_FEN: &str = "1n1Rkb1r/p4ppp/4q3/4p1B1/4P3/8/PPP2PPP/2K5 b k - 1 17";
const PAWN_MOVES_FEN: &str = "rnbqkb1r/pp1p1pPp/8/2p1pP2/1P1P4/3P3P/P1P1P3/RNBQKBNR w KQkq e6 0 1";
const KING_MOVES_FEN: &str = "8/2k5/8/4Pn2/3BK3/8/8/8 w - - 0 1";
const ROOK_MOVES_FEN: &str = "6k1/8/5r1p/8/1nR5/5N2/8/6K1 b - - 0 1";
const BISHOP_MOVES_FEN: &str = "6k1/1b6/4n2P/8/1n4B1/1B3N2/1N6/2b2K1 b - - 0 1";
const QUEEN_MOVES_FEN: &str = "6k1/7P/4nq2/8/1nQ5/5N2/1N6/6K1 b - - 0 1";
const CASTLING_FEN: &str = "r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1";
const PAWNS_FEN: &str = "rnbqkb1r/pp1p1pPp/8/2p1pP2/1P1P4/3P3P/P1P1P3/RNBQKBNR w KQkq e6 0 1";

fn main() -> anyhow::Result<()> {
    let mut board = Board::default();
    board.parse_fen("rnbqkbnr/ppppppp1/7p/4P3/8/8/PPPP1PPP/RNBQKBNR b KQkq - 0 2")?;
    println!("{}", board);

    dbg!(board.make_move(Move::new(
        Square::D7,
        Square::D5,
        MoveKind::Quiet,
        MoveFlag::None,
    ))?);

    println!("{}", board);

    dbg!(board.make_move(Move::new(
        Square::E5,
        Square::D6,
        MoveKind::Capture,
        MoveFlag::EnPassant,
    ))?);

    println!("{}", board);

    dbg!(board.make_move(Move::new(
        Square::E8,
        Square::D7,
        MoveKind::Quiet,
        MoveFlag::None,
    ))?);

    println!("{}", board);

    dbg!(board.make_move(Move::new(
        Square::D1,
        Square::G4,
        MoveKind::Quiet,
        MoveFlag::None,
    ))?);

    println!("{}", board);

    // king left in check - should return false
    dbg!(board.make_move(Move::new(
        Square::A7,
        Square::A6,
        MoveKind::Quiet,
        MoveFlag::None,
    ))?);

    println!("{}", board);

    Ok(())
}
