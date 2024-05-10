use krusty::{board::Board, move_generator::MoveGenerator};

const START_POSITION_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
const OPERA_GAME_FEN: &str = "1n1Rkb1r/p4ppp/4q3/4p1B1/4P3/8/PPP2PPP/2K5 b k - 1 17";
const PAWN_MOVES_FEN: &str = "rnbqkb1r/pp1p1pPp/8/2p1pP2/1P1P4/3P3P/P1P1P3/RNBQKBNR w KQkq e6 0 1";

fn main() -> anyhow::Result<()> {
    let mut board = Board::default();
    board.parse_fen(PAWN_MOVES_FEN)?;
    println!("{}", board);

    let mg = MoveGenerator {};

    mg.generate_pawn_moves(&board)?;

    Ok(())
}
