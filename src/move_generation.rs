use crate::{Board, CaptureType, Color, Move, MoveAttempt, Piece, PieceType};

// Function to generate all valid moves for the current player
fn generate_all_moves(board: &Board) -> (Vec<Move>, usize) {
    let mut moves = Vec::new();
    let mut king_space = 0;

    for (i, tile) in board.tiles.iter().enumerate() {
        if let Some(piece) = tile.piece {
            if piece.color == board.current_turn {
                // Generate moves based on the piece type
                match piece.piece_type {
                    PieceType::Pawn(_, _) => moves.extend(generate_pawn_moves(board, i, piece)),
                    PieceType::Knight => moves.extend(generate_knight_moves(board, i)),
                    PieceType::Bishop => {
                        moves.extend(generate_sliding_piece_moves(board, i, &[9, 7]))
                    }
                    PieceType::Rook => {
                        moves.extend(generate_sliding_piece_moves(board, i, &[8, 1]))
                    }
                    PieceType::Queen => {
                        moves.extend(generate_sliding_piece_moves(board, i, &[9, 7, 8, 1]))
                    }
                    PieceType::King => {
                        moves.extend(generate_king_moves(board, i));
                        king_space = i;
                    }
                }
            }
        }
    }
    (moves, king_space)
}

fn generate_pawn_moves(board: &Board, from: usize, piece: Piece) -> Vec<Move> {
    let mut moves = Vec::new();
    let direction = match piece.color {
        Color::White => -8, // White pawns move up
        Color::Black => 8,  // Black pawns move down
    };

    // Single step forward
    let to = (from as isize + direction) as usize;
    if to < 64 && board.tiles[to].piece.is_none() {
        moves.push(Move {
            from,
            to,
            capture_type: CaptureType::Normal,
        });
    }

    // Double step forward if first move
    if let PieceType::Pawn(true, _) = piece.piece_type {
        let double_step = (from as isize + 2 * direction) as usize;
        if board.tiles[to].piece.is_none() && board.tiles[double_step].piece.is_none() {
            moves.push(Move {
                from,
                to: double_step,
                capture_type: CaptureType::Doublestep,
            });
        }
    }

    // Diagonal capture
    let diagonals = [direction + 1, direction - 1];
    for &diag in &diagonals {
        let to = (from as isize + diag) as usize;
        if to < 64 {
            if let Some(captured_piece) = board.tiles[to].piece {
                if captured_piece.color != piece.color {
                    moves.push(Move {
                        from,
                        to,
                        capture_type: CaptureType::Normal,
                    });
                }
            } else {
                // En passant capture
                let adj_tile = (from as isize + (diag - direction)) as usize;
                if let Some(adj_pawn) = board.tiles[adj_tile].piece {
                    if adj_pawn.color != piece.color {
                        if let PieceType::Pawn(_, en_passant_move) = adj_pawn.piece_type {
                            if en_passant_move != -1 {
                                moves.push(Move {
                                    from,
                                    to,
                                    capture_type: CaptureType::EnPassant(adj_tile),
                                });
                            }
                        }
                    }
                }
            }
        }
    }

    moves
}

fn generate_knight_moves(board: &Board, from: usize) -> Vec<Move> {
    let mut moves = Vec::new();
    let knight_moves = [-17, -15, -10, -6, 6, 10, 15, 17];
    for &offset in &knight_moves {
        let to = (from as isize + offset) as usize;
        if to < 64 && is_valid_destination(board, from, to) {
            moves.push(Move {
                from,
                to,
                capture_type: CaptureType::Normal,
            });
        }
    }
    moves
}

fn generate_sliding_piece_moves(board: &Board, from: usize, offsets: &[isize]) -> Vec<Move> {
    let mut moves = Vec::new();
    for &offset in offsets {
        let mut pos = from as isize;
        while pos >= 0 && pos < 64 {
            pos += offset;
            if pos < 0 || pos >= 64 {
                break;
            }
            let to = pos as usize;
            if is_valid_destination(board, from, to) {
                moves.push(Move {
                    from,
                    to,
                    capture_type: CaptureType::Normal,
                });
                if board.tiles[to].piece.is_some() {
                    break; // Stop sliding if we hit a piece
                }
            } else {
                break;
            }
        }
    }
    moves
}

fn generate_king_moves(board: &Board, from: usize) -> Vec<Move> {
    let mut moves = Vec::new();
    let king_moves = [-1, 1, -8, 8, -9, 9, -7, 7];

    for &offset in &king_moves {
        let to = (from as isize + offset) as usize;
        if to < 64 && is_valid_destination(board, from, to) {
            moves.push(Move {
                from,
                to,
                capture_type: CaptureType::Normal,
            });
        }
    }

    // Add castling moves
    let (rank, kingside_rook, queenside_rook) = match board.current_turn {
        Color::White => (7, 63, 56), // Rank 7 for white
        Color::Black => (0, 7, 0),   // Rank 0 for black
    };

    // Kingside castling
    if board.can_castle_kingside(board.current_turn) {
        let empty_squares = [from + 1, from + 2];
        if empty_squares
            .iter()
            .all(|&i| board.tiles[i].piece.is_none())
        {
            moves.push(Move {
                from,
                to: from + 2,
                capture_type: if board.current_turn == Color::White {
                    CaptureType::WhiteKingsideCastle
                } else {
                    CaptureType::BlackKingsideCastle
                },
            });
        }
    }

    // Queenside castling
    if board.can_castle_queenside(board.current_turn) {
        let empty_squares = [from - 1, from - 2, from - 3];
        if empty_squares
            .iter()
            .all(|&i| board.tiles[i].piece.is_none())
        {
            moves.push(Move {
                from,
                to: from - 2,
                capture_type: if board.current_turn == Color::White {
                    CaptureType::WhiteQueensideCastle
                } else {
                    CaptureType::BlackQueensideCastle
                },
            });
        }
    }

    moves
}

// Helper function to check if a destination is valid (empty or enemy piece)
fn is_valid_destination(board: &Board, from: usize, to: usize) -> bool {
    if let Some(dest_piece) = board.tiles[to].piece {
        return dest_piece.color != board.tiles[from].piece.unwrap().color;
    }
    true
}

pub fn generate_legal_moves(board: &mut Board) -> Vec<Move> {
    let (pseudo_legal_moves, king_space) = generate_all_moves(board);
    let mut legal_moves = Vec::new();
    for pseudo_legal_move in pseudo_legal_moves {
        board.make_move(&pseudo_legal_move, false);
        let new_moves = generate_all_moves(board).0;
        if let Some(_) = new_moves.iter().find(|&&x| x.to == king_space) {
        } else {
            legal_moves.push(pseudo_legal_move);
        }
    }
    legal_moves
}

pub fn generate_human_legal_moves(board: &mut Board) -> (Vec<Move>, Vec<Move>) {
    let (pseudo_legal_moves, king_space) = generate_all_moves(board);
    let mut legal_moves = Vec::new();
    for pseudo_legal_move in &pseudo_legal_moves {
        board.make_move(pseudo_legal_move, false);
        let move_played = MoveAttempt::new(pseudo_legal_move, &board);
        let new_moves = generate_all_moves(board).0;
        if let Some(_) = new_moves.iter().find(|&&x| x.to == king_space) {
        } else {
            legal_moves.push(*pseudo_legal_move);
        }
        board.unmake_move(move_played);
    }
    (legal_moves, pseudo_legal_moves)
}
