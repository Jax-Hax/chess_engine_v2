mod move_generation;

use move_generation::generate_human_legal_moves;

use crate::move_generation::generate_legal_moves;
use std::{io, ops::Not};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Color {
    White,
    Black,
}

impl Not for Color {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PieceType {
    Bishop,
    Rook,
    Knight,
    Queen,
    King,
    Pawn(bool, i16), // been_moved, en_passant_possible
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Piece {
    color: Color,
    piece_type: PieceType,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Tile {
    piece: Option<Piece>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Move {
    from: usize,
    to: usize,
    capture_type: CaptureType,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum CaptureType {
    Normal,
    EnPassant(usize),
    WhiteQueensideCastle,
    WhiteKingsideCastle,
    BlackQueensideCastle,
    BlackKingsideCastle,
    Doublestep,
}
#[derive(Clone, Debug)]
pub struct Board {
    tiles: Vec<Tile>,
    current_turn: Color,
    num_moves: u16,
    can_castle_white_queenside: bool,
    can_castle_white_kingside: bool,
    can_castle_black_queenside: bool,
    can_castle_black_kingside: bool,
}

impl Board {
    // Parse FEN string to create a Board
    fn from_fen(fen: &str) -> Self {
        let mut sections = fen.split_whitespace();
        let board_section = sections.next().unwrap();
        let turn_section = sections.next().unwrap();
        let castling_section = sections.next().unwrap();

        // Parse the board into Vec<Tile>
        let mut board = Vec::with_capacity(64);
        for row in board_section.split('/') {
            for ch in row.chars() {
                match ch {
                    '1'..='8' => {
                        let empty_tiles = ch.to_digit(10).unwrap();
                        for _ in 0..empty_tiles {
                            board.push(Tile { piece: None });
                        }
                    }
                    'p' => board.push(Tile {
                        piece: Some(Piece {
                            color: Color::Black,
                            piece_type: PieceType::Pawn(true, -1),
                        }),
                    }),
                    'r' => board.push(Tile {
                        piece: Some(Piece {
                            color: Color::Black,
                            piece_type: PieceType::Rook,
                        }),
                    }),
                    'n' => board.push(Tile {
                        piece: Some(Piece {
                            color: Color::Black,
                            piece_type: PieceType::Knight,
                        }),
                    }),
                    'b' => board.push(Tile {
                        piece: Some(Piece {
                            color: Color::Black,
                            piece_type: PieceType::Bishop,
                        }),
                    }),
                    'q' => board.push(Tile {
                        piece: Some(Piece {
                            color: Color::Black,
                            piece_type: PieceType::Queen,
                        }),
                    }),
                    'k' => board.push(Tile {
                        piece: Some(Piece {
                            color: Color::Black,
                            piece_type: PieceType::King,
                        }),
                    }),
                    'P' => board.push(Tile {
                        piece: Some(Piece {
                            color: Color::White,
                            piece_type: PieceType::Pawn(true, -1),
                        }),
                    }),
                    'R' => board.push(Tile {
                        piece: Some(Piece {
                            color: Color::White,
                            piece_type: PieceType::Rook,
                        }),
                    }),
                    'N' => board.push(Tile {
                        piece: Some(Piece {
                            color: Color::White,
                            piece_type: PieceType::Knight,
                        }),
                    }),
                    'B' => board.push(Tile {
                        piece: Some(Piece {
                            color: Color::White,
                            piece_type: PieceType::Bishop,
                        }),
                    }),
                    'Q' => board.push(Tile {
                        piece: Some(Piece {
                            color: Color::White,
                            piece_type: PieceType::Queen,
                        }),
                    }),
                    'K' => board.push(Tile {
                        piece: Some(Piece {
                            color: Color::White,
                            piece_type: PieceType::King,
                        }),
                    }),
                    _ => panic!("Invalid FEN character: {}", ch),
                }
            }
        }

        // Parse current turn
        let current_turn = match turn_section {
            "w" => Color::White,
            "b" => Color::Black,
            _ => panic!("Invalid FEN turn: {}", turn_section),
        };

        // Parse castling rights
        let can_castle_white_kingside = castling_section.contains('K');
        let can_castle_white_queenside = castling_section.contains('Q');
        let can_castle_black_kingside = castling_section.contains('k');
        let can_castle_black_queenside = castling_section.contains('q');

        Board {
            tiles: board,
            current_turn,
            num_moves: 0,
            can_castle_white_queenside,
            can_castle_white_kingside,
            can_castle_black_queenside,
            can_castle_black_kingside,
        }
    }
    fn print_board(&self) {
        println!("  a b c d e f g h");
        print!("8 ");
        for (i, tile) in self.tiles.iter().enumerate() {
            if let Some(piece) = tile.piece {
                let piece_char = match piece {
                    Piece {
                        color: Color::White,
                        piece_type: PieceType::King,
                    } => '♚',
                    Piece {
                        color: Color::White,
                        piece_type: PieceType::Queen,
                    } => '♛',
                    Piece {
                        color: Color::White,
                        piece_type: PieceType::Rook,
                    } => '♜',
                    Piece {
                        color: Color::White,
                        piece_type: PieceType::Bishop,
                    } => '♝',
                    Piece {
                        color: Color::White,
                        piece_type: PieceType::Knight,
                    } => '♞',
                    Piece {
                        color: Color::White,
                        piece_type: PieceType::Pawn(_, _),
                    } => '♟',
                    Piece {
                        color: Color::Black,
                        piece_type: PieceType::King,
                    } => '♔',
                    Piece {
                        color: Color::Black,
                        piece_type: PieceType::Queen,
                    } => '♕',
                    Piece {
                        color: Color::Black,
                        piece_type: PieceType::Rook,
                    } => '♖',
                    Piece {
                        color: Color::Black,
                        piece_type: PieceType::Bishop,
                    } => '♗',
                    Piece {
                        color: Color::Black,
                        piece_type: PieceType::Knight,
                    } => '♘',
                    Piece {
                        color: Color::Black,
                        piece_type: PieceType::Pawn(_, _),
                    } => '♙',
                };
                print!("{}", piece_char);
            } else {
                print!(".");
            }
            print!(" ");
            // Newline after every 8 tiles (since the board is 8x8)
            if (i + 1) % 8 == 0 && (64 - i) / 8 != 0 {
                println!();
                print!("{} ", (64 - i) / 8);
            }
        }
        println!();
        println!("  a b c d e f g h")
    }
    // Move a piece from one tile to another if valid
    fn make_move(&mut self, move_played: &Move, for_humans: bool) -> bool {
        let moving_piece = self.tiles[move_played.from].piece;
        if moving_piece.is_none() {
            if for_humans {
                println!("No piece at the source position.");
            }
            return false;
        }

        if for_humans {
            // Capture if there's an enemy piece on the destination tile
            if let Some(dest_piece) = self.tiles[move_played.to].piece {
                if dest_piece.color != moving_piece.unwrap().color {
                    println!("Captured a piece!");
                }
            }
        }

        // Move the piece
        self.tiles[move_played.to].piece = moving_piece;
        self.tiles[move_played.from].piece = None;

        match move_played.capture_type {
            CaptureType::WhiteKingsideCastle => {
                self.tiles[move_played.to].piece = moving_piece;
                self.tiles[move_played.from].piece = None;
                // Move the rook
                self.tiles[61].piece = self.tiles[63].piece;
                self.tiles[63].piece = None;
            }
            CaptureType::WhiteQueensideCastle => {
                self.tiles[move_played.to].piece = moving_piece;
                self.tiles[move_played.from].piece = None;
                // Move the rook
                self.tiles[59].piece = self.tiles[56].piece;
                self.tiles[56].piece = None;
            }
            CaptureType::BlackKingsideCastle => {
                self.tiles[move_played.to].piece = moving_piece;
                self.tiles[move_played.from].piece = None;
                // Move the rook
                self.tiles[5].piece = self.tiles[7].piece;
                self.tiles[7].piece = None;
            }
            CaptureType::BlackQueensideCastle => {
                self.tiles[move_played.to].piece = moving_piece;
                self.tiles[move_played.from].piece = None;
                // Move the rook
                self.tiles[3].piece = self.tiles[0].piece;
                self.tiles[0].piece = None;
            }
            CaptureType::Doublestep => {
                self.tiles[move_played.to].piece = Some(Piece {
                    color: self.current_turn,
                    piece_type: PieceType::Pawn(false, self.num_moves as i16), // Set en passant
                });
            }
            _ => {
                // Regular movement logic
            }
        }
        if let Some(piece) = moving_piece {
            self.update_castling_rights(move_played.from, piece);
        }
        //if en passanted
        if let CaptureType::EnPassant(target) = move_played.capture_type {
            self.tiles[target].piece = None;
        }

        // Switch turns
        self.current_turn = match self.current_turn {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };

        true
    }
    fn reset_en_passant(&mut self) {
        for tile in &mut self.tiles {
            if let Some(piece) = tile.piece {
                if let PieceType::Pawn(_, en_passant) = piece.piece_type {
                    if en_passant != -1 && en_passant > self.num_moves as i16 + 2 {
                        tile.piece = Some(Piece {
                            color: piece.color,
                            piece_type: PieceType::Pawn(false, -1), // Reset en passant
                        });
                    }
                }
            }
        }
    }
    fn update_castling_rights(&mut self, from: usize, piece: Piece) {
        match piece.piece_type {
            PieceType::King => match piece.color {
                Color::White => {
                    self.can_castle_white_kingside = false;
                    self.can_castle_white_queenside = false;
                }
                Color::Black => {
                    self.can_castle_black_kingside = false;
                    self.can_castle_black_queenside = false;
                }
            },
            PieceType::Rook => match (piece.color, from) {
                (Color::White, 63) => self.can_castle_white_kingside = false,
                (Color::White, 56) => self.can_castle_white_queenside = false,
                (Color::Black, 7) => self.can_castle_black_kingside = false,
                (Color::Black, 0) => self.can_castle_black_queenside = false,
                _ => {}
            },
            _ => {}
        }
    }
    fn can_castle_kingside(&self, color: Color) -> bool {
        match color {
            Color::White => self.can_castle_white_kingside,
            Color::Black => self.can_castle_black_kingside,
        }
    }

    fn can_castle_queenside(&self, color: Color) -> bool {
        match color {
            Color::White => self.can_castle_white_queenside,
            Color::Black => self.can_castle_black_queenside,
        }
    }
    fn unmake_move(&mut self, piece_move: Move) {

    }
    // Basic game loop
    fn game_loop(&mut self) {
        loop {
            self.reset_en_passant();
            self.print_board();
            println!("Current turn: {:?}", self.current_turn);

            // Generate all valid moves for the current turn
            let (valid_moves, all_moves) = generate_human_legal_moves(self);
            if valid_moves.is_empty() {
                println!("No valid moves available. Game over!");
                break;
            }

            let mut from;
            let mut to;
            let mut move_is_valid = false;
            while !move_is_valid {
                from = String::new();
                to = String::new();
                println!("Enter the 'from' position (e.g., e2): ");
                io::stdin()
                    .read_line(&mut from)
                    .expect("Failed to read input");
                let from = from.trim();

                println!("Enter the 'to' position (e.g., e4): ");
                io::stdin()
                    .read_line(&mut to)
                    .expect("Failed to read input");
                let to = to.trim();

                // Convert chess notation to board index
                let from_idx = chess_notation_to_index(from);
                let to_idx = chess_notation_to_index(to);

                match (from_idx, to_idx) {
                    (Some(from), Some(to)) => {
                        let move_played = valid_moves.iter().find(|e| e.from == from && e.to == to);
                        match move_played {
                            Some(move_exists) => {
                                if self.make_move(move_exists, true) {
                                    move_is_valid = true;
                                    self.num_moves += 1;
                                }
                            }
                            _ => {
                                println!("Move was invalid. Try again.");
                            }
                        }
                    }
                    _ => println!("Invalid chess notation. Try again."),
                }
            }
        }
    }
}

fn main() {
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let mut board = Board::from_fen(fen);
    board.game_loop();
}

fn chess_notation_to_index(pos: &str) -> Option<usize> {
    if pos.len() != 2 {
        return None;
    }

    let file = pos.chars().nth(0).unwrap();
    let rank = pos.chars().nth(1).unwrap();

    // Convert file (a-h) to 0-7
    let file_index = match file {
        'a' => 0,
        'b' => 1,
        'c' => 2,
        'd' => 3,
        'e' => 4,
        'f' => 5,
        'g' => 6,
        'h' => 7,
        _ => return None, // Invalid file
    };

    // Convert rank (1-8) to 0-7 (reversed)
    let rank_index = match rank {
        '1' => 7,
        '2' => 6,
        '3' => 5,
        '4' => 4,
        '5' => 3,
        '6' => 2,
        '7' => 1,
        '8' => 0,
        _ => return None, // Invalid rank
    };

    // Calculate the index in the 0-63 range
    Some(rank_index * 8 + file_index)
}
