use serde::{Deserialize, Serialize};
use indexmap::IndexMap;
pub use Color::*;
#[derive(Eq, PartialEq)]
pub struct Board {
    pub history: Vec<Move>,
    pub pieces: IndexMap<Square, Piece>,
    pub attack_lines: IndexMap<Square, Vec<Vec<Square>>>,
    pub kings: IndexMap<Color, Square>,

    pub turn: Color,
    pub castling_rights: IndexMap<Color, CastlingRights>,
    pub enpassant_square: Option<Square>,
    pub halfmove_clock: u32,
    pub fullmove_number: u32,
}
#[derive(Clone, Deserialize, Eq, PartialEq, Serialize)]
pub struct Move {
    pub from: Square,
    pub to: Square,
    pub r#type: MoveType,
    pub captured: Option<Piece>,
    pub promotion: Option<PieceType>,
}
#[derive(Clone, Copy, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Square {
    pub file: File,
    pub rank: Rank,
}
#[derive(Clone, Deserialize, Eq, PartialEq, Serialize)]
pub struct Piece {
    pub id: u8,
    pub r#type: PieceType,
    pub color: Color,
}
#[derive(Copy, Clone, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum Color {
    White,
    Black,
}
impl Color {
    pub fn opposite(self) -> Color {
        match self {
            White => Black,
            Black => White,
        }
    }

    pub fn get_multiplier(self) -> i8 {
        match self {
            White => 1,
            Black => -1,
        }
    }

    pub fn get_piece_rank(self) -> Rank {
        match self {
            White => Rank::_1,
            Black => Rank::_8,
        }
    }

    pub fn get_pawn_rank(self) -> Rank {
        match self {
            White => Rank::_2,
            Black => Rank::_7,
        }
    }

    pub fn get_enpassant_rank(self) -> Rank {
        match self {
            White => Rank::_3,
            Black => Rank::_6,
        }
    }

    pub fn get_center_rank(self) -> Rank {
        match self {
            White => Rank::_4,
            Black => Rank::_5,
        }
    }
}
#[derive(Clone, Copy, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub enum File {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}
#[derive(Clone, Copy, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub enum Rank {
    _1,
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
    _8,
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CastlingRights {
    pub kingside: bool,
    pub queenside: bool,
}
#[derive(Clone, Copy, Deserialize, Eq, PartialEq, Serialize)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}
#[derive(Clone, Eq, Deserialize, PartialEq, Serialize)]
pub enum MoveType {
    Normal,
    Capture,
    Promotion,
    PromotionCapture,
    PawnJump,
    Enpassant,
    Castle,
}