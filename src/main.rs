use std::io;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Color {
    White,
    Black,
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

#[derive(Clone, Copy, Debug)]
struct Piece {
    color: Color,
    piece_type: PieceType,
}

#[derive(Clone, Copy, Debug)]
struct Tile {
    piece: Option<Piece>,
}
#[derive(Clone, Debug)]
struct Board {
    board: Vec<Tile>,
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
            board,
            current_turn,
            num_moves: 0,
            can_castle_white_queenside,
            can_castle_white_kingside,
            can_castle_black_queenside,
            can_castle_black_kingside,
        }
    }
    fn print_board(&self) {
        println!("  abcdefgh");
        print!("8 ");
        for (i, tile) in self.board.iter().enumerate() {
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

            // Newline after every 8 tiles (since the board is 8x8)
            if (i + 1) % 8 == 0 && (64 - i) / 8 != 0 {
                println!();
                print!("{} ", (64 - i) / 8);
            }
        }
        println!();
        println!("  abcdefgh")
    }

    fn check_validity(&mut self, from: usize, to: usize) -> bool {
        let piece = match self.board[from].piece {
            Some(p) => p,
            None => return false, // No piece to move
        };

        let dest_piece = self.board[to].piece;

        // Cannot capture a piece of the same color
        if let Some(dest) = dest_piece {
            if dest.color == piece.color {
                return false;
            }
        }

        match piece.piece_type {
            PieceType::Pawn(_, _) => self.check_pawn_validity(from, to, piece),
            PieceType::Knight => self.check_knight_validity(from, to),
            PieceType::Bishop => self.check_sliding_piece_validity(from, to, &[9, 7]),
            PieceType::Rook => self.check_sliding_piece_validity(from, to, &[8, 1]),
            PieceType::Queen => self.check_sliding_piece_validity(from, to, &[9, 7, 8, 1]),
            PieceType::King => self.check_king_validity(from, to),
        }
    }

    fn check_pawn_validity(&mut self, from: usize, to: usize, piece: Piece) -> bool {
        let direction = match piece.color {
            Color::White => -8, // White pawns move up
            Color::Black => 8,  // Black pawns move down
        };

        let double_step = direction * 2;
        let valid_move = match to as isize - from as isize {
            // Single step forward
            x if x == direction => self.board[to].piece.is_none(),

            // Double step forward if it's the first move
            x if x == double_step
                && piece.piece_type == PieceType::Pawn(true, -1)
                && self.board[to].piece.is_none() =>
            {
                self.board[(from as isize + direction) as usize]
                    .piece
                    .is_none() // No piece in between
            }

            // Diagonal capture or en passant capture
            x if (x == direction + 1 || x == direction - 1) => {
                if let Some(captured_piece) = self.board[to].piece {
                    captured_piece.color != piece.color // Regular diagonal capture
                } else {
                    // En passant capture
                    let adj_tile = (from as isize + (x - direction)) as usize;
                    if let Some(adj_pawn) = self.board[adj_tile].piece {
                        if let PieceType::Pawn(_, en_passant) = adj_pawn.piece_type {
                            if en_passant != -1 {
                                self.board[adj_tile].piece = None;
                                true
                            } else {
                                false
                            }
                        }
                    } else {
                        false
                    }
                }
            }
            _ => false,
        };

        // If the pawn moved two squares, mark it as eligible for en passant
        if valid_move
            && piece.piece_type == PieceType::Pawn(true, -1)
            && (to as isize - from as isize).abs() == double_step
        {
            self.board[from].piece = Some(Piece {
                color: piece.color,
                piece_type: PieceType::Pawn(false, self.num_moves as i16), // Enable en passant
            });
        } else if valid_move {
            // Mark the pawn as having moved and not eligible for en passant
            self.board[from].piece = Some(Piece {
                color: piece.color,
                piece_type: PieceType::Pawn(false, -1),
            });
        }

        valid_move
    }

    fn check_knight_validity(&self, from: usize, to: usize) -> bool {
        // All possible knight moves in 0-63 range
        let knight_moves = [-17, -15, -10, -6, 6, 10, 15, 17];
        let delta = to as isize - from as isize;

        knight_moves.contains(&delta)
    }

    fn check_sliding_piece_validity(&self, from: usize, to: usize, offsets: &[isize]) -> bool {
        let direction = to as isize - from as isize;

        for &offset in offsets {
            let mut pos = from as isize;

            // Keep sliding in the offset direction
            while (pos >= 0 && pos < 64) && (pos % 8 != 7 && pos % 8 != 0) {
                pos += offset;
                if pos == to as isize {
                    return true;
                }
                if self.board[pos as usize].piece.is_some() {
                    break;
                }
            }
        }

        false
    }

    fn check_king_validity(&self, from: usize, to: usize) -> bool {
        let delta = to as isize - from as isize;
        [-1, 1, -8, 8, -9, 9, -7, 7].contains(&delta)
    }

    // Move a piece from one tile to another if valid
    fn move_piece(&mut self, from: usize, to: usize) -> bool {
        if !self.check_validity(from, to) {
            println!("Invalid move!");
            return false;
        }

        let moving_piece = self.board[from].piece;
        if moving_piece.is_none() {
            println!("No piece at the source position.");
            return false;
        }

        // Capture if there's an enemy piece on the destination tile
        if let Some(dest_piece) = self.board[to].piece {
            if dest_piece.color != moving_piece.unwrap().color {
                println!("Captured a piece!");
            }
        }

        // Move the piece
        self.board[to].piece = moving_piece;
        self.board[from].piece = None;

        // Switch turns
        self.current_turn = match self.current_turn {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };

        true
    }
    fn reset_en_passant(&mut self) {
        for tile in &mut self.board {
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
    // Basic game loop
    fn game_loop(&mut self) {
        loop {
            self.reset_en_passant();
            self.print_board();
            println!("Current turn: {:?}", self.current_turn);

            let mut from = String::new();
            let mut to = String::new();

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
                    if !self.move_piece(from, to) {
                        println!("Move was invalid. Try again.");
                    } else {
                        self.num_moves += 1;
                    }
                }
                _ => println!("Invalid chess notation. Try again."),
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
