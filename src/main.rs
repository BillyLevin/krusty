use krusty::{
    board::Board,
    move_generator::{MoveGenerator, MoveList},
};

const START_POSITION_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
const OPERA_GAME_FEN: &str = "1n1Rkb1r/p4ppp/4q3/4p1B1/4P3/8/PPP2PPP/2K5 b k - 1 17";
const PAWN_MOVES_FEN: &str = "rnbqkb1r/pp1p1pPp/8/2p1pP2/1P1P4/3P3P/P1P1P3/RNBQKBNR w KQkq e6 0 1";
const KING_MOVES_FEN: &str = "8/2k5/8/4Pn2/3BK3/8/8/8 w - - 0 1";
const ROOK_MOVES_FEN: &str = "6k1/8/5r1p/8/1nR5/5N2/8/6K1 b - - 0 1";
const BISHOP_MOVES_FEN: &str = "6k1/1b6/4n2P/8/1n4B1/1B3N2/1N6/2b2K1 b - - 0 1";
const QUEEN_MOVES_FEN: &str = "6k1/7P/4nq2/8/1nQ5/5N2/1N6/6K1 b - - 0 1";

fn main() -> anyhow::Result<()> {
    let mut board = Board::default();
    board.parse_fen(START_POSITION_FEN)?;
    println!("{}", board);

    let mg = MoveGenerator::default();
    let mut move_list = MoveList::default();

    mg.generate_all_moves(&board, &mut move_list)?;

    dbg!(move_list);

    board.make_move(move_list.get(0))?;

    println!("{}", board);

    Ok(())
}
