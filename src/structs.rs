use chess_engine::square;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
pub use Color::*;
pub use MoveType::*;
pub use PieceType::*;
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

impl Move {
    pub fn from_normal(from: Square, to: Square) -> Move {
        Move {
            from,
            to,
            r#type: Normal,
            captured: None,
            promotion: None,
        }
    }

    pub fn from_pawn_jump(from: Square, to: Square) -> Move {
        assert_eq!((from.rank as i8 - to.rank as i8).abs(), 2);
        Move {
            from,
            to,
            r#type: PawnJump,
            captured: None,
            promotion: None,
        }
    }

    pub fn from_capture(from: Square, to: Square, captured: Piece) -> Move {
        Move {
            from,
            to,
            r#type: Capture,
            captured: Some(captured),
            promotion: None,
        }
    }

    pub fn from_promotion(from: Square, to: Square, promotion: PieceType) -> Move {
        Move {
            from,
            to,
            r#type: Promotion,
            captured: None,
            promotion: Some(promotion),
        }
    }

    pub fn from_promotion_capture(
        from: Square,
        to: Square,
        captured: Piece,
        promotion: PieceType,
    ) -> Move {
        Move {
            from,
            to,
            r#type: PromotionCapture,
            captured: Some(captured),
            promotion: Some(promotion),
        }
    }

    pub fn from_enpassant(from: Square, to: Square, captured: Piece) -> Move {
        Move {
            from,
            to,
            r#type: Enpassant,
            captured: Some(captured),
            promotion: None,
        }
    }

    pub fn from_castle(from: Square, to: Square) -> Move {
        Move {
            from,
            to,
            r#type: Castle,
            captured: None,
            promotion: None,
        }
    }
}

#[derive(Clone, Copy, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Square {
    pub file: File,
    pub rank: Rank,
}
impl Square {
    pub const ALL: [Square; 64] = [
        square!(A8),
        square!(B8),
        square!(C8),
        square!(D8),
        square!(E8),
        square!(F8),
        square!(G8),
        square!(H8),
        square!(A7),
        square!(B7),
        square!(C7),
        square!(D7),
        square!(E7),
        square!(F7),
        square!(G7),
        square!(H7),
        square!(A6),
        square!(B6),
        square!(C6),
        square!(D6),
        square!(E6),
        square!(F6),
        square!(G6),
        square!(H6),
        square!(A5),
        square!(B5),
        square!(C5),
        square!(D5),
        square!(E5),
        square!(F5),
        square!(G5),
        square!(H5),
        square!(A4),
        square!(B4),
        square!(C4),
        square!(D4),
        square!(E4),
        square!(F4),
        square!(G4),
        square!(H4),
        square!(A3),
        square!(B3),
        square!(C3),
        square!(D3),
        square!(E3),
        square!(F3),
        square!(G3),
        square!(H3),
        square!(A2),
        square!(B2),
        square!(C2),
        square!(D2),
        square!(E2),
        square!(F2),
        square!(G2),
        square!(H2),
        square!(A1),
        square!(B1),
        square!(C1),
        square!(D1),
        square!(E1),
        square!(F1),
        square!(G1),
        square!(H1),
    ];

    pub fn offset(&self, file_offset: i8, rank_offset: i8) -> Option<Square> {
        let file = File::try_from((Into::<i8>::into(self.file) + file_offset) as i8);
        let rank = Rank::try_from((Into::<i8>::into(self.rank) + rank_offset) as i8);

        if let Ok(file) = file {
            if let Ok(rank) = rank {
                return Some(square!(file rank));
            }
        }

        None
    }
}

#[derive(Clone, Deserialize, Eq, PartialEq, Serialize)]
pub struct Piece {
    pub id: u8,
    pub r#type: PieceType,
    pub color: Color,
}
impl Piece {
    pub fn new(id: u8, r#type: PieceType, color: Color) -> Piece {
        Piece { id, r#type, color }
    }
    pub fn get_attack_lines(&self, square: Square) -> Vec<Vec<Square>> {
        let mut attack_lines = vec![];
        match self.r#type {
            Pawn => {
                let team_multiplier = if self.color == White { 1 } else { -1 };
                for (file, rank) in [(-1, team_multiplier), (1, team_multiplier)] {
                    if let Some(target_square) = square.offset(file, rank) {
                        attack_lines.push(vec![target_square]);
                    }
                }
            }
            Knight => {
                for (file, rank) in Directions::KNIGHT {
                    if let Some(target_square) = square.offset(file, rank) {
                        attack_lines.push(vec![target_square]);
                    }
                }
            }
            Bishop => {
                for (file, rank) in Directions::BISHOP {
                    let mut current_square = square;
                    let mut line = vec![];
                    while let Some(target_square) = current_square.offset(file, rank) {
                        line.push(target_square);
                        current_square = target_square;
                    }

                    if !line.is_empty() {
                        attack_lines.push(line);
                    }
                }
            }
            Rook => {
                for (file, rank) in Directions::ROOK {
                    let mut current_square = square;
                    let mut line = vec![];
                    while let Some(target_square) = current_square.offset(file, rank) {
                        line.push(target_square);
                        current_square = target_square;
                    }

                    if !line.is_empty() {
                        attack_lines.push(line);
                    }
                }
            }
            Queen => {
                for (file, rank) in Directions::QUEEN {
                    let mut current_square = square;
                    let mut line = vec![];
                    while let Some(target_square) = current_square.offset(file, rank) {
                        line.push(target_square);
                        current_square = target_square;
                    }

                    if !line.is_empty() {
                        attack_lines.push(line);
                    }
                }
            }
            King => {
                for (file, rank) in Directions::KING {
                    if let Some(target_square) = square.offset(file, rank) {
                        attack_lines.push(vec![target_square]);
                    }
                }
            }
        }

        attack_lines
    }
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

impl From<File> for i8 {
    fn from(file: File) -> i8 {
        match file {
            File::A => 0,
            File::B => 1,
            File::C => 2,
            File::D => 3,
            File::E => 4,
            File::F => 5,
            File::G => 6,
            File::H => 7,
        }
    }
}

impl TryFrom<i8> for File {
    type Error = ();

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(File::A),
            1 => Ok(File::B),
            2 => Ok(File::C),
            3 => Ok(File::D),
            4 => Ok(File::E),
            5 => Ok(File::F),
            6 => Ok(File::G),
            7 => Ok(File::H),
            _ => Err(()),
        }
    }
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
impl From<Rank> for i8 {
    fn from(rank: Rank) -> i8 {
        match rank {
            Rank::_1 => 0,
            Rank::_2 => 1,
            Rank::_3 => 2,
            Rank::_4 => 3,
            Rank::_5 => 4,
            Rank::_6 => 5,
            Rank::_7 => 6,
            Rank::_8 => 7,
        }
    }
}

impl TryFrom<i8> for Rank {
    type Error = ();

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Rank::_1),
            1 => Ok(Rank::_2),
            2 => Ok(Rank::_3),
            3 => Ok(Rank::_4),
            4 => Ok(Rank::_5),
            5 => Ok(Rank::_6),
            6 => Ok(Rank::_7),
            7 => Ok(Rank::_8),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CastlingRights {
    pub kingside: bool,
    pub queenside: bool,
}
impl CastlingRights {
    pub fn new(kingside: bool, queenside: bool) -> Self {
        Self {
            kingside,
            queenside,
        }
    }
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
pub struct Directions {}

impl Directions {
    pub const KNIGHT: [(i8, i8); 8] = [
        (1, 2),
        (2, 1),
        (2, -1),
        (1, -2),
        (-1, -2),
        (-2, -1),
        (-2, 1),
        (-1, 2),
    ];

    pub const BISHOP: [(i8, i8); 4] = [(1, 1), (1, -1), (-1, -1), (-1, 1)];

    pub const ROOK: [(i8, i8); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    pub const QUEEN: [(i8, i8); 8] = [
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
        (-1, -1),
        (-1, 0),
        (-1, 1),
    ];

    pub const KING: [(i8, i8); 8] = Directions::QUEEN;
}
