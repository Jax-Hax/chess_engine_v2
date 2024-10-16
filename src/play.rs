use chess_engine::square;
use indexmap::IndexMap;

use crate::structs::*;

impl Board {
    pub fn execute(&mut self, r#move: Move) {
        self.halfmove_clock += 1;
        self.fullmove_number += 1;

        match r#move.r#type {
            Normal | PawnJump => {
                let piece = self.pieces.swap_remove(&r#move.from).unwrap();
                let enpassant_rank = piece.color.get_enpassant_rank();
                let r#type = piece.r#type;
                self.attack_lines.swap_remove(&r#move.from);
                self.attack_lines
                    .insert(r#move.to, piece.get_attack_lines(r#move.to));
                self.pieces.insert(r#move.to, piece);

                if r#type == Pawn {
                    self.halfmove_clock = 0;
                }

                if r#move.r#type == PawnJump {
                    self.enpassant_square = Some(square!(r#move.from.file enpassant_rank));
                }
            }
            Capture => {
                let piece = self.pieces.swap_remove(&r#move.from).unwrap();
                self.attack_lines.swap_remove(&r#move.from);
                self.attack_lines
                    .insert(r#move.to, piece.get_attack_lines(r#move.to));
                self.pieces.insert(r#move.to, piece);

                self.halfmove_clock = 0;
            }
            Promotion => {
                let mut piece = self.pieces.swap_remove(&r#move.from).unwrap();
                piece.r#type = r#move.promotion.unwrap();
                self.attack_lines.swap_remove(&r#move.from);
                self.attack_lines
                    .insert(r#move.to, piece.get_attack_lines(r#move.to));
                self.pieces.insert(r#move.to, piece);
            }
            PromotionCapture => {
                let mut piece = self.pieces.swap_remove(&r#move.from).unwrap();
                piece.r#type = r#move.promotion.unwrap();
                self.attack_lines.swap_remove(&r#move.from);
                self.attack_lines
                    .insert(r#move.to, piece.get_attack_lines(r#move.to));
                self.pieces.insert(r#move.to, piece);

                self.halfmove_clock = 0;
            }
            Enpassant => {
                let piece = self.pieces.swap_remove(&r#move.from).unwrap();
                self.attack_lines.swap_remove(&r#move.from);
                self.attack_lines
                    .insert(r#move.to, piece.get_attack_lines(r#move.to));
                self.pieces.insert(r#move.to, piece);

                let captured_square = &r#move
                    .from
                    .offset(
                        Into::<i8>::into(r#move.to.file) - Into::<i8>::into(r#move.from.file),
                        0,
                    )
                    .unwrap();
                self.pieces.swap_remove(captured_square);
                self.attack_lines.swap_remove(captured_square);

                self.halfmove_clock = 0;
            }
            Castle => {
                let king = self.pieces.swap_remove(&r#move.from).unwrap();
                self.attack_lines.swap_remove(&r#move.from);
                self.attack_lines
                    .insert(r#move.to, king.get_attack_lines(r#move.to));
                self.pieces.insert(r#move.to, king);

                let (rook_square_from, rook_square_to) = if r#move.to.file == File::C {
                    (square!(A r#move.to.rank), square!(D r#move.to.rank))
                } else {
                    (square!(H r#move.to.rank), square!(F r#move.to.rank))
                };
                let rook = self.pieces.swap_remove(&rook_square_from).unwrap();
                self.attack_lines.swap_remove(&rook_square_from);
                self.attack_lines
                    .insert(rook_square_to, rook.get_attack_lines(rook_square_to));
                self.pieces.insert(rook_square_to, rook);

                self.castling_rights
                    .insert(self.turn, CastlingRights::new(false, false));
            }
        }

        if let King = self.pieces.get(&r#move.to).unwrap().r#type {
            self.kings.insert(self.turn, r#move.to);
        }

        if r#move.r#type != PawnJump {
            self.enpassant_square = None;
        }

        self.history.push(r#move);
        self.turn = self.turn.opposite();

        for color in [White, Black].iter() {
            let CastlingRights {
                kingside,
                queenside,
            } = *self.castling_rights.get(color).unwrap();
            let piece_rank = color.get_piece_rank();

            if kingside {
                let king = self.pieces.get(&square!(E piece_rank));
                let rook = self.pieces.get(&square!(H piece_rank));

                if king.is_none()
                    || rook.is_none()
                    || king.unwrap().r#type != King
                    || rook.unwrap().r#type != Rook
                    || king.unwrap().color != rook.unwrap().color
                {
                    self.castling_rights
                        .insert(*color, CastlingRights::new(false, queenside));
                }
            }

            if queenside {
                let king = self.pieces.get(&square!(E piece_rank));
                let rook = self.pieces.get(&square!(A piece_rank));

                if king.is_none()
                    || rook.is_none()
                    || king.unwrap().r#type != King
                    || rook.unwrap().r#type != Rook
                    || king.unwrap().color != rook.unwrap().color
                {
                    self.castling_rights
                        .insert(*color, CastlingRights::new(kingside, false));
                }
            }
        }
    }
    pub fn undo(
        &mut self,
        castling_rights: IndexMap<Color, CastlingRights>,
        enpassant_square: Option<Square>,
        halfmove_clock: u32,
    ) -> Option<()> {
        let r#move = self.history.pop()?;
        self.turn = self.turn.opposite();
		self.castling_rights = castling_rights;
		self.enpassant_square = enpassant_square;
		self.halfmove_clock = halfmove_clock;
		self.fullmove_number -= 1;

        match r#move.r#type {
            Normal | PawnJump => {
                let piece = self.pieces.swap_remove(&r#move.to).unwrap();
                self.attack_lines.swap_remove(&r#move.to);
                self.attack_lines
                    .insert(r#move.from, piece.get_attack_lines(r#move.from));
                self.pieces.insert(r#move.from, piece);
            }
            Capture => {
                let piece = self.pieces.swap_remove(&r#move.to).unwrap();
                self.attack_lines.swap_remove(&r#move.to);
                self.attack_lines
                    .insert(r#move.from, piece.get_attack_lines(r#move.from));
                self.pieces.insert(r#move.from, piece);

                let captured = r#move.captured.unwrap();
                self.attack_lines
                    .insert(r#move.to, captured.get_attack_lines(r#move.to));
                self.pieces.insert(r#move.to, captured);
            }
            Promotion => {
                let mut piece = self.pieces.swap_remove(&r#move.to).unwrap();
                piece.r#type = Pawn;
                self.attack_lines.swap_remove(&r#move.to);
                self.attack_lines
                    .insert(r#move.from, piece.get_attack_lines(r#move.from));
                self.pieces.insert(r#move.from, piece);
            }
            PromotionCapture => {
                let mut piece = self.pieces.swap_remove(&r#move.to).unwrap();
                piece.r#type = Pawn;
                self.attack_lines.swap_remove(&r#move.to);
                self.attack_lines
                    .insert(r#move.from, piece.get_attack_lines(r#move.from));
                self.pieces.insert(r#move.from, piece);

                let captured = r#move.captured.unwrap();
                self.attack_lines
                    .insert(r#move.to, captured.get_attack_lines(r#move.to));
                self.pieces.insert(r#move.to, captured);
            }
            Enpassant => {
                let piece = self.pieces.swap_remove(&r#move.to).unwrap();
                self.attack_lines.swap_remove(&r#move.to);
                self.attack_lines
                    .insert(r#move.from, piece.get_attack_lines(r#move.from));
                self.pieces.insert(r#move.from, piece);

                let captured = r#move.captured.unwrap();
                let captured_square = r#move
                    .from
                    .offset(
                        Into::<i8>::into(r#move.to.file) - Into::<i8>::into(r#move.from.file),
                        0,
                    )
                    .unwrap();
                self.attack_lines
                    .insert(captured_square, captured.get_attack_lines(captured_square));
                self.pieces.insert(captured_square, captured);
            }
            Castle => {
                let king = self.pieces.swap_remove(&r#move.to).unwrap();
                self.attack_lines.swap_remove(&r#move.to);
                self.attack_lines
                    .insert(r#move.from, king.get_attack_lines(r#move.from));
                self.pieces.insert(r#move.from, king);

                let (rook_square_from, rook_square_to) = if r#move.to.file == File::C {
                    (square!(A r#move.to.rank), square!(D r#move.to.rank))
                } else {
                    (square!(H r#move.to.rank), square!(F r#move.to.rank))
                };
                let rook = self.pieces.swap_remove(&rook_square_to).unwrap();
                self.attack_lines.swap_remove(&rook_square_to);
                self.attack_lines
                    .insert(rook_square_from, rook.get_attack_lines(rook_square_from));
                self.pieces.insert(rook_square_from, rook);
            }
        }

        if let King = self.pieces.get(&r#move.from).unwrap().r#type {
            self.kings.insert(self.turn, r#move.from);
        }

        Some(())
    }
}
