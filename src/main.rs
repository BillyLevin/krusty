use krusty::{
    bitboard::Bitboard,
    board::Board,
    generate_magics::{
        generate_bishop_attack_mask, generate_bishop_blocker_mask, generate_rook_attack_mask,
        generate_rook_blocker_mask,
    },
    move_generator::{MoveGenerator, MoveList},
    square::{File, Rank, Square},
};

const START_POSITION_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
const OPERA_GAME_FEN: &str = "1n1Rkb1r/p4ppp/4q3/4p1B1/4P3/8/PPP2PPP/2K5 b k - 1 17";
const PAWN_MOVES_FEN: &str = "rnbqkb1r/pp1p1pPp/8/2p1pP2/1P1P4/3P3P/P1P1P3/RNBQKBNR w KQkq e6 0 1";
const KING_MOVES_FEN: &str = "8/2k5/8/4Pn2/3BK3/8/8/8 w - - 0 1";

fn main() -> anyhow::Result<()> {
    let mut board = Board::default();
    board.parse_fen(KING_MOVES_FEN)?;
    println!("{}", board);

    // let mg = MoveGenerator {};
    // let mut move_list = MoveList::default();
    //
    // mg.generate_pawn_moves(&board, &mut move_list)?;
    // mg.generate_knight_moves(&board, &mut move_list)?;
    // mg.generate_king_moves(&board, &mut move_list)?;
    //
    // dbg!(move_list);

    println!("rook masks:");
    println!("{:?}", generate_rook_blocker_mask(Square::D3)?);
    println!(
        "{:?}",
        generate_rook_attack_mask(
            Square::D3,
            Square::D2.bitboard() | Square::C3.bitboard() | Square::D5.bitboard()
        )?
    );

    println!("bishop masks:");
    println!("{:?}", generate_bishop_blocker_mask(Square::D5)?);
    println!(
        "{:?}",
        generate_bishop_attack_mask(Square::D5, Square::C4.bitboard() | Square::F3.bitboard())?
    );

    Ok(())
}
