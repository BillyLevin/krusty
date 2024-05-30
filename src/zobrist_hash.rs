use crate::{
    board::{Board, CastlingRights, Side},
    prng::Prng,
    square::{Piece, PieceKind, Square},
};

const SIDE_OFFSET: usize = 768; // 12 * 64 pieces before it
const CASTLE_OFFSET: usize = 769; // + 1 bit for side
const EN_PASSANT_OFFSET: usize = 785; // + 16 bits for castling

const ZOBRIST_NUMBERS_SIZE: usize = 794; // + 9 bits for en passant files

const fn init_zobrist_en_passant_files() -> [usize; 65] {
    let mut files = [0; 65];

    let mut square = 0;

    while square < 65 {
        let rank = square / 8;

        if rank == 2 || rank == 5 {
            let file = square % 8;
            files[square] = file;
        } else {
            files[square] = 8;
        }

        square += 1;
    }

    files
}

const ZOBRIST_EN_PASSANT_FILES: [usize; 65] = init_zobrist_en_passant_files();

pub struct ZobristHasher {
    numbers: [u64; ZOBRIST_NUMBERS_SIZE],
}

pub enum ZobristKey {
    Piece(Piece, Square),
    Side,
    Castling(CastlingRights),
    EnPassantFile(Square),
}

impl ZobristHasher {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn hash_position(&self, board: &Board) -> u64 {
        let mut hash = 0;

        for (square, piece) in board.pieces().iter().enumerate() {
            if piece.kind != PieceKind::NoPiece {
                hash ^= self.get_key_part(ZobristKey::Piece(*piece, square.into()));
            }
        }

        if board.side_to_move() == Side::Black {
            hash ^= self.get_key_part(ZobristKey::Side);
        }

        hash ^= self.get_key_part(ZobristKey::Castling(board.castling_rights()));

        hash ^= self.get_key_part(ZobristKey::EnPassantFile(board.en_passant_square()));

        hash
    }

    pub fn get_key_part(&self, key: ZobristKey) -> u64 {
        match key {
            ZobristKey::Piece(piece, square) => self.index_piece(piece, square),
            ZobristKey::Side => self.numbers[SIDE_OFFSET],
            ZobristKey::Castling(castling_rights) => {
                self.numbers[CASTLE_OFFSET + (castling_rights as usize)]
            }
            ZobristKey::EnPassantFile(square) => {
                self.numbers[EN_PASSANT_OFFSET + ZOBRIST_EN_PASSANT_FILES[square as usize]]
            }
        }
    }

    fn index_piece(&self, piece: Piece, square: Square) -> u64 {
        assert!(piece.kind != PieceKind::NoPiece);

        let piece_offset = 6 * (piece.color as usize) + (piece.kind as usize);
        self.numbers[piece_offset + square.index()]
    }
}

impl Default for ZobristHasher {
    fn default() -> Self {
        let mut prng = Prng::new(123);

        Self {
            numbers: [(); ZOBRIST_NUMBERS_SIZE].map(|_| prng.random_u64()),
        }
    }
}
