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
            return (i32::MIN, None)
        }
        return (0, None);
    }
    let moves = order_moves(board, moves);
    for r#move in moves {
        let castling_rights = board.castling_rights.clone();
        let enpassant_square = board.enpassant_square;
        let halfmove_clock = board.halfmove_clock;

        board.execute(r#move.clone());

        let evaluation = search(board, depth - 1, beta.wrapping_neg(), alpha.wrapping_neg()).0.wrapping_neg();

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
    let perspective = if board.turn == Color::White {1} else {-1};
    let white_eval = count_material(board, Color::White);
    let black_eval = count_material(board, Color::Black);
    let evaluation = white_eval - black_eval;
    evaluation * perspective
}
fn count_material(board: &Board, color: Color) -> i32 {
    let mut material = 0;
    for (_, piece) in &board.pieces {
        if piece.color != color {
            continue;
        }
        material += get_piece_value(&piece.r#type);
    }
    material
}
fn force_king_to_corner_endgame_eval(friendly_king_square: Square, opponent_king_square: Square, endgame_weight: i32) -> i32 {
    let evaluation = 0;
    evaluation
}
fn order_moves(board: &Board, moves: Vec<Move>) -> Vec<Move> {
    let mut scores = vec![];
    for r#move in &moves {
        let mut score_guess = 0;
        let move_piece_type = board.pieces.get(&r#move.from);
        let capture_piece_type = board.pieces.get(&r#move.to);
        // prioritise capturing opponent's most valuable pieces with our least valuable pieces
        if let Some(_) = capture_piece_type {
            score_guess = 10 * get_piece_value(&capture_piece_type.unwrap().r#type) - get_piece_value(&move_piece_type.unwrap().r#type);
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
        evaluation = search_all_captures(board, beta.wrapping_neg(), alpha.wrapping_neg()).wrapping_neg();
        board.undo(castling_rights, enpassant_square, halfmove_clock);
        if evaluation >= beta {
            return beta;
        }
        alpha = alpha.max(evaluation);
    }
    alpha
}