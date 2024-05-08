use krusty::board::Board;

const START_POSITION_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
const OPERA_GAME_FEN: &str = "1n1Rkb1r/p4ppp/4q3/4p1B1/4P3/8/PPP2PPP/2K5 b k - 1 17";

fn main() -> anyhow::Result<()> {
    let mut board = Board::default();
    board.parse_fen(OPERA_GAME_FEN)?;
    println!("{}", board);
    Ok(())
}
