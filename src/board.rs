use chess_engine::square;
use structs::*;
use crate::structs;

impl Board {
    pub fn print_board(&self) {
        let mut board_rep = [['.'; 8]; 8]; // Initialize the board with empty squares

        // Populate the board with pieces
        for (square, piece) in &self.pieces {
            let rank_idx = 7 - Into::<i8>::into(square.rank) as usize; // Invert rank for display (top-down)
            let file_idx = Into::<i8>::into(square.file) as usize;

            // Get a character for the piece based on its type and color
            let piece_char = match (piece.r#type, piece.color) {
                (PieceType::Pawn, Color::White) => '♟',
                (PieceType::Pawn, Color::Black) => '♙',
                (PieceType::Knight, Color::White) => '♞',
                (PieceType::Knight, Color::Black) => '♘',
                (PieceType::Bishop, Color::White) => '♝',
                (PieceType::Bishop, Color::Black) => '♗',
                (PieceType::Rook, Color::White) => '♜',
                (PieceType::Rook, Color::Black) => '♖',
                (PieceType::Queen, Color::White) => '♛',
                (PieceType::Queen, Color::Black) => '♕',
                (PieceType::King, Color::White) => '♚',
                (PieceType::King, Color::Black) => '♔',
            };

            board_rep[rank_idx][file_idx] = piece_char;
        }

        // Print the board
        for (i, rank) in board_rep.iter().enumerate() {
            print!("{} ", 8 - i); // Print rank numbers
            for square in rank.iter() {
                print!("{} ", square);
            }
            println!();
        }

        // Print file letters at the bottom
        println!("  a b c d e f g h");
    }
    pub fn get_square(&self, piece: &Piece) -> Option<Square> {
        self.pieces
            .iter()
            .find_map(|(s, p)| if p.id == piece.id { Some(*s) } else { None })
    }
    pub fn get_moves(&self, only_captures: bool) -> (Vec<Move>, bool) {
        let mut moves = vec![];
        let opposite_color = self.turn.opposite();

        let piece_rank = self.turn.get_piece_rank();
        let mut in_check = false;
        let mut unchecked_squares = vec![
            square!(C piece_rank),
            square!(D piece_rank),
            square!(F piece_rank),
            square!(G piece_rank),
        ];

        for (square, piece) in &self.pieces {
            if piece.color == opposite_color {
                continue;
            }

            let square = *square;
            match piece.r#type {
                Pawn => {
                    let multiplier = self.turn.get_multiplier();

                    if !only_captures && square.rank == self.turn.get_pawn_rank()
                        && self
                            .pieces
                            .get(&square.offset(0, multiplier).unwrap())
                            .is_none()
                        && self
                            .pieces
                            .get(&square.offset(0, 2 * multiplier).unwrap())
                            .is_none()
                    {
                        moves.push(Move::from_pawn_jump(
                            square,
                            square.offset(0, 2 * multiplier).unwrap(),
                        ));
                    }

                    if square.rank == self.turn.opposite().get_pawn_rank() {
                        if let Some(target_square) = square.offset(-1, multiplier) {
                            if let Some(target_piece) = self.pieces.get(&target_square) {
                                if target_piece.color != self.turn {
                                    for r#type in [Queen, Rook, Bishop, Knight] {
                                        moves.push(Move::from_promotion_capture(
                                            square,
                                            target_square,
                                            target_piece.clone(),
                                            r#type,
                                        ));
                                    }
                                }
                            }
                        }
                        let target_square = square.offset(0, multiplier).unwrap();
                        if !only_captures && self.pieces.get(&target_square).is_none() {
                            for r#type in [Queen, Rook, Bishop, Knight] {
                                moves.push(Move::from_promotion(square, target_square, r#type));
                            }
                        }
                        if let Some(target_square) = square.offset(1, multiplier) {
                            if let Some(target_piece) = self.pieces.get(&target_square) {
                                if target_piece.color != self.turn {
                                    for r#type in [Queen, Rook, Bishop, Knight] {
                                        moves.push(Move::from_promotion_capture(
                                            square,
                                            target_square,
                                            target_piece.clone(),
                                            r#type,
                                        ));
                                    }
                                }
                            }
                        }
                    } else {
                        if let Some(enpassant_square) = self.enpassant_square {
                            if enpassant_square.rank == self.turn.opposite().get_enpassant_rank()
                                && square.rank == self.turn.opposite().get_center_rank()
                            {
                                if let Some(left_target_square) = square.offset(-1, multiplier) {
                                    if left_target_square == enpassant_square {
                                        moves.push(Move::from_enpassant(
                                            square,
                                            left_target_square,
                                            self.pieces
                                                .get(
                                                    &enpassant_square
                                                        .offset(0, -multiplier)
                                                        .unwrap(),
                                                )
                                                .unwrap()
                                                .clone(),
                                        ));
                                    }
                                }
                                if let Some(right_target_square) = square.offset(1, multiplier) {
                                    if right_target_square == enpassant_square {
                                        moves.push(Move::from_enpassant(
                                            square,
                                            right_target_square,
                                            self.pieces
                                                .get(
                                                    &enpassant_square
                                                        .offset(0, -multiplier)
                                                        .unwrap(),
                                                )
                                                .unwrap()
                                                .clone(),
                                        ));
                                    }
                                }
                            }
                        }
                        if let Some(target_square) = square.offset(-1, multiplier) {
                            if let Some(target_piece) = self.pieces.get(&target_square) {
                                if target_piece.color != self.turn {
                                    moves.push(Move::from_capture(
                                        square,
                                        target_square,
                                        target_piece.clone(),
                                    ));
                                }
                            }
                        }
                        let target_square = square.offset(0, multiplier).unwrap();
                        if !only_captures && self.pieces.get(&target_square).is_none() {
                            moves.push(Move::from_normal(square, target_square));
                        }
                        if let Some(target_square) = square.offset(1, multiplier) {
                            if let Some(target_piece) = self.pieces.get(&target_square) {
                                if target_piece.color != self.turn {
                                    moves.push(Move::from_capture(
                                        square,
                                        target_square,
                                        target_piece.clone(),
                                    ));
                                }
                            }
                        }
                    }
                }
                Knight => {
                    for (file, rank) in Directions::KNIGHT {
                        if let Some(target_square) = square.offset(file, rank) {
                            if let Some(target_piece) = self.pieces.get(&target_square) {
                                if target_piece.color != self.turn {
                                    moves.push(Move::from_capture(
                                        square,
                                        target_square,
                                        target_piece.clone(),
                                    ));
                                }
                            } else if !only_captures {
                                moves.push(Move::from_normal(square, target_square));
                            }
                        }
                    }
                }
                Bishop => self.get_straight_moves(&mut moves, piece, &Directions::BISHOP, only_captures),
                Rook => self.get_straight_moves(&mut moves, piece, &Directions::ROOK, only_captures),
                Queen => self.get_straight_moves(&mut moves, piece, &Directions::QUEEN, only_captures),
                King => {
                    for (file, rank) in Directions::KING {
                        if let Some(target_square) = square.offset(file, rank) {
                            if let Some(target_piece) = self.pieces.get(&target_square) {
                                if target_piece.color != self.turn {
                                    moves.push(Move::from_capture(
                                        square,
                                        target_square,
                                        target_piece.clone(),
                                    ));
                                }
                            } else if !only_captures {
                                moves.push(Move::from_normal(square, target_square));
                            }
                        }
                    }
                }
            }
        }

        let king_square = *self.kings.get(&self.turn).unwrap();
        for square in Square::ALL {
            if let Some(piece) = self.pieces.get(&square) {
                if piece.color != self.turn {
                    // This block of code runs where {piece} is every piece
                    // on the enemy team

                    let attack_lines = piece.get_attack_lines(square);
                    if let Some(index) =
                        attack_lines.iter().position(|al| al.contains(&king_square))
                    {
                        let attack_line = attack_lines
                            .get(index)
                            .expect("Invalid lines_with_king index");

                        let mut blocking_pieces = 0;
                        let mut resolving_squares = vec![square];

                        for square in attack_line {
                            let square = *square;

                            if self.pieces.get(&square).is_some() {
                                if square == king_square {
                                    match blocking_pieces {
                                        0 => {
                                            in_check = true;

                                            // Move that checks the King
                                            moves.retain(|m| {
                                                (m.from == king_square
                                                    && !attack_line.contains(&m.to))
                                                    || resolving_squares.contains(&m.to)
													// Allow Enpassant where captured piece is checking the King
                                                    || (m.r#type == Enpassant
                                                        && resolving_squares.get(0).unwrap()
                                                            == &square!(m.to.file m.from.rank))
                                            });
                                        }
                                        1 => {
                                            // Move that pins another piece
                                            moves.retain(|m| {
                                                !resolving_squares.contains(&m.from)
                                                    || resolving_squares.contains(&m.to)
                                            });
                                        }
                                        2 => {
                                            // Enpassant that leaves the king in check
                                            // This takes two pieces off the board
                                            moves.retain(|m| {
                                                m.r#type != Enpassant
                                                    || m.from.rank != king_square.rank
                                            })
                                        }
                                        _ => {}
                                    }

                                    break;
                                } else {
                                    blocking_pieces += 1;
                                }
                            }

                            resolving_squares.push(square);
                        }
                    }

                    // This block of code filter moves that regard our King
                    // moving into a check
                    for attack_line in &attack_lines {
                        moves.retain(|m| {
                            if m.from == king_square {
                                for square in attack_line {
                                    if self.pieces.get(square).is_some() {
                                        return *square != m.to;
                                    } else if *square == m.to {
                                        return false;
                                    }
                                }
                            }

                            true
                        });

                        if unchecked_squares.iter().any(|s| attack_line.contains(s)) {
                            for square in attack_line {
                                if let Some(index) =
                                    unchecked_squares.iter().position(|s| s == square)
                                {
                                    unchecked_squares.remove(index);
                                    break;
                                }

                                if self.pieces.get(square).is_some() {
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }

        let CastlingRights {
            queenside,
            kingside,
        } = *self.castling_rights.get(&self.turn).unwrap();
        if !in_check {
            if kingside
                && [square!(F piece_rank), square!(G piece_rank)]
                    .iter()
                    .all(|s| self.pieces.get(s).is_none())
                && unchecked_squares.contains(&square!(F piece_rank))
                && unchecked_squares.contains(&square!(G piece_rank))
            {
                moves.push(Move::from_castle(
                    square!(E piece_rank),
                    square!(G piece_rank),
                ));
            }

            if queenside
                && [
                    square!(B piece_rank),
                    square!(C piece_rank),
                    square!(D piece_rank),
                ]
                .iter()
                .all(|s| self.pieces.get(s).is_none())
                && unchecked_squares.contains(&square!(C piece_rank))
                && unchecked_squares.contains(&square!(D piece_rank))
            {
                moves.push(Move::from_castle(
                    square!(E piece_rank),
                    square!(C piece_rank),
                ));
            }
        }
        if only_captures {
            moves.retain(|m| m.captured.is_some());
        }
        (moves, in_check)
    }

    fn get_straight_moves(&self, moves: &mut Vec<Move>, piece: &Piece, directions: &[(i8, i8)], only_captures: bool) {
        let square = self.get_square(piece).unwrap();
        for (file, rank) in directions {
            let mut file_offset = *file;
            let mut rank_offset = *rank;
            while let Some(target_square) = square.offset(file_offset, rank_offset) {
                if let Some(target_piece) = self.pieces.get(&target_square) {
                    if target_piece.color != self.turn {
                        moves.push(Move::from_capture(
                            square,
                            target_square,
                            target_piece.clone(),
                        ));
                    }
                    break;
                } else if !only_captures {
                    moves.push(Move::from_normal(square, target_square));
                }
                file_offset += file;
                rank_offset += rank;
            }
        }
    }
}
