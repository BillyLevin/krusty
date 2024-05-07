use krusty::{
    board::Board,
    square::{Color, Piece, PieceKind, Square},
};

fn main() {
    let mut board = Board::default();

    board.add_piece(Piece::new(Color::White, PieceKind::Pawn), Square::A2);
    board.add_piece(Piece::new(Color::White, PieceKind::Pawn), Square::B2);
    board.add_piece(Piece::new(Color::White, PieceKind::Pawn), Square::C2);
    board.add_piece(Piece::new(Color::White, PieceKind::Pawn), Square::D2);
    board.add_piece(Piece::new(Color::White, PieceKind::Pawn), Square::E2);
    board.add_piece(Piece::new(Color::White, PieceKind::Pawn), Square::F2);
    board.add_piece(Piece::new(Color::White, PieceKind::Pawn), Square::G2);
    board.add_piece(Piece::new(Color::White, PieceKind::Pawn), Square::H2);

    board.add_piece(Piece::new(Color::White, PieceKind::Rook), Square::A1);
    board.add_piece(Piece::new(Color::White, PieceKind::Knight), Square::B1);
    board.add_piece(Piece::new(Color::White, PieceKind::Bishop), Square::C1);
    board.add_piece(Piece::new(Color::White, PieceKind::Queen), Square::D1);
    board.add_piece(Piece::new(Color::White, PieceKind::King), Square::E1);
    board.add_piece(Piece::new(Color::White, PieceKind::Bishop), Square::F1);
    board.add_piece(Piece::new(Color::White, PieceKind::Knight), Square::G1);
    board.add_piece(Piece::new(Color::White, PieceKind::Rook), Square::H1);

    board.add_piece(Piece::new(Color::Black, PieceKind::Pawn), Square::A7);
    board.add_piece(Piece::new(Color::Black, PieceKind::Pawn), Square::B7);
    board.add_piece(Piece::new(Color::Black, PieceKind::Pawn), Square::C7);
    board.add_piece(Piece::new(Color::Black, PieceKind::Pawn), Square::D7);
    board.add_piece(Piece::new(Color::Black, PieceKind::Pawn), Square::E7);
    board.add_piece(Piece::new(Color::Black, PieceKind::Pawn), Square::F7);
    board.add_piece(Piece::new(Color::Black, PieceKind::Pawn), Square::G7);
    board.add_piece(Piece::new(Color::Black, PieceKind::Pawn), Square::H7);

    board.add_piece(Piece::new(Color::Black, PieceKind::Rook), Square::A8);
    board.add_piece(Piece::new(Color::Black, PieceKind::Knight), Square::B8);
    board.add_piece(Piece::new(Color::Black, PieceKind::Bishop), Square::C8);
    board.add_piece(Piece::new(Color::Black, PieceKind::Queen), Square::D8);
    board.add_piece(Piece::new(Color::Black, PieceKind::King), Square::E8);
    board.add_piece(Piece::new(Color::Black, PieceKind::Bishop), Square::F8);
    board.add_piece(Piece::new(Color::Black, PieceKind::Knight), Square::G8);
    board.add_piece(Piece::new(Color::Black, PieceKind::Rook), Square::H8);

    println!("{}", board);
}
