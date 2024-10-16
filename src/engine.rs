use crate::structs::*;

const PAWN_VALUE: i32 = 100;
const KNIGHT_VALUE: i32 = 300;
const BISHOP_VALUE: i32 = 300;
const ROOK_VALUE: i32 = 500;
const QUEEN_VALUE: i32 = 900;

pub fn search(board: &mut Board, depth: usize, mut alpha: i32, beta: i32) -> (i32, Option<Move>) {
    if depth == 0 {
        return (search_all_captures(board, alpha, beta), None);
    }
    let (moves, in_check) = board.get_moves(false);
    let mut best_move = moves[0].clone();
    if moves.len() == 0 {
        if in_check {
            return (i32::MIN, None);
        }
        return (0, None);
    }
    let moves = order_moves(board, moves);
    for r#move in moves {
        let castling_rights = board.castling_rights.clone();
        let enpassant_square = board.enpassant_square;
        let halfmove_clock = board.halfmove_clock;

        board.execute(r#move.clone());

        let evaluation = search(board, depth - 1, beta.wrapping_neg(), alpha.wrapping_neg())
            .0
            .wrapping_neg();

        board.undo(castling_rights, enpassant_square, halfmove_clock);
        if evaluation >= beta {
            return (beta, None);
        }
        if evaluation > alpha {
            alpha = evaluation;
            best_move = r#move;
        }
        alpha = alpha.max(evaluation)
    }

    (alpha, Some(best_move))
}
fn eval(board: &Board) -> i32 {
    let perspective = if board.turn == Color::White { 1 } else { -1 };
    let (mut white_eval, white_pawns) = count_material(board, Color::White);
    let (mut black_eval, black_pawns) = count_material(board, Color::Black);
    let kings: Vec<&Square> = board
        .pieces
        .iter()
        .filter_map(|(square, piece)| {
            if piece.r#type == PieceType::King {
                Some(square)
            } else {
                None
            }
        })
        .collect();
    let mut white_king_square = &Square {rank: Rank::_1, file: File::A};
    let mut black_king_square = &Square {rank: Rank::_1, file: File::A};

    for &square in &kings {
        if let Some(piece) = board.pieces.get(square) {
            match piece.color {
                Color::White => white_king_square = square,
                Color::Black => black_king_square = square,
            }
        }
    }
    white_eval += force_king_to_corner_endgame_eval(
        white_king_square,
        black_king_square,
        endgame_phase_weight(white_eval - white_pawns * PAWN_VALUE),
    );
    black_eval += force_king_to_corner_endgame_eval(
        black_king_square,
        white_king_square,
        endgame_phase_weight(black_eval - black_pawns * PAWN_VALUE),
    );
    let evaluation = white_eval - black_eval;
    evaluation * perspective
}
fn endgame_phase_weight(material_without_pawns: i32) -> f32 {
    const ENDGAME_MATERIAL_START: f32 = (ROOK_VALUE * 2 + BISHOP_VALUE + KNIGHT_VALUE) as f32;
    let multiplier = 1.0 / ENDGAME_MATERIAL_START;
    1.0 - 1.0_f32.min(material_without_pawns as f32 * multiplier)
}
fn count_material(board: &Board, color: Color) -> (i32, i32) {
    let mut material = 0;
    let mut num_pawns = 0;
    for (_, piece) in &board.pieces {
        if piece.color != color {
            continue;
        }
        material += get_piece_value(&piece.r#type);
        if let PieceType::Pawn = piece.r#type {
            num_pawns += 1;
        }
    }
    (material, num_pawns)
}
fn force_king_to_corner_endgame_eval(
    friendly_king_square: &Square,
    opponent_king_square: &Square,
    endgame_weight: f32,
) -> i32 {
    let mut evaluation = 0;
    let opponent_king_dist_to_center_file =
        (3 - opponent_king_square.file as i32).max(opponent_king_square.file as i32 - 4);
    let opponent_king_dist_to_center_rank =
        (3 - opponent_king_square.file as i32).max(opponent_king_square.file as i32 - 4);
    let opponent_king_dist_to_center =
        opponent_king_dist_to_center_file + opponent_king_dist_to_center_rank;
    evaluation += opponent_king_dist_to_center;
    let dist_between_kings_files =
        (friendly_king_square.file as i32 - opponent_king_square.file as i32).abs();
    let dist_between_kings_ranks =
        (friendly_king_square.rank as i32 - opponent_king_square.rank as i32).abs();
    let dist_between_kings = dist_between_kings_files + dist_between_kings_ranks;
    evaluation += 14 - dist_between_kings;
    (evaluation as f32 * 10.0 * endgame_weight).round() as i32
}
fn order_moves(board: &Board, moves: Vec<Move>) -> Vec<Move> {
    let mut scores = vec![];
    for r#move in &moves {
        let mut score_guess = 0;
        let move_piece_type = board.pieces.get(&r#move.from);
        let capture_piece_type = board.pieces.get(&r#move.to);
        // prioritise capturing opponent's most valuable pieces with our least valuable pieces
        if let Some(_) = capture_piece_type {
            score_guess = 10 * get_piece_value(&capture_piece_type.unwrap().r#type)
                - get_piece_value(&move_piece_type.unwrap().r#type);
        }
        //promoting a pawn is probably good
        if r#move.promotion.is_some() {
            score_guess += get_piece_value(&r#move.promotion.unwrap());
        }
        scores.push(score_guess);
    }
    sort_moves(moves, scores)
}
fn sort_moves(moves: Vec<Move>, scores: Vec<i32>) -> Vec<Move> {
    let mut zipped: Vec<_> = moves.into_iter().zip(scores).collect();
    zipped.sort_by(|a, b| b.1.cmp(&a.1)); // Sort by score (descending)

    // Unzip the structs back
    zipped.into_iter().map(|(s, _)| s).collect()
}
fn get_piece_value(piece: &PieceType) -> i32 {
    match piece {
        Pawn => PAWN_VALUE,
        Knight => KNIGHT_VALUE,
        Bishop => BISHOP_VALUE,
        Rook => ROOK_VALUE,
        Queen => QUEEN_VALUE,
        King => 0,
    }
}
fn search_all_captures(board: &mut Board, mut alpha: i32, beta: i32) -> i32 {
    let mut evaluation = eval(board);
    if evaluation >= beta {
        return beta;
    }
    alpha = alpha.max(evaluation);
    let capture_moves = board.get_moves(true).0;
    let capture_moves = order_moves(board, capture_moves);
    for r#move in capture_moves {
        let castling_rights = board.castling_rights.clone();
        let enpassant_square = board.enpassant_square;
        let halfmove_clock = board.halfmove_clock;
        board.execute(r#move);
        evaluation =
            search_all_captures(board, beta.wrapping_neg(), alpha.wrapping_neg()).wrapping_neg();
        board.undo(castling_rights, enpassant_square, halfmove_clock);
        if evaluation >= beta {
            return beta;
        }
        alpha = alpha.max(evaluation);
    }
    alpha
}
