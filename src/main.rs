use krusty::{
    board::Board,
    move_generator::{Move, MoveFlag, MoveKind},
    perft::run_perft_tests,
    square::Square,
};

fn main() -> anyhow::Result<()> {
    let mut board = Board::default();
    board.parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")?;

    let initial_hash = board.hash();

    // println!("{}", board);
    dbg!(board.hash());

    let mv = Move::new(Square::E2, Square::E4, MoveKind::Quiet, MoveFlag::None);

    board.make_move(mv)?;
    dbg!(board.hash());

    board.unmake_move(mv)?;

    assert_eq!(board.hash(), initial_hash);

    let perft_contents = include_str!("../perft.epd");
    run_perft_tests(perft_contents);

    Ok(())
}
