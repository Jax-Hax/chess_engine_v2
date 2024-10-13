use crate::structs::*;


pub fn search(board: &mut Board, depth: usize, mut alpha: i32, beta: i32) -> (i32, Option<Move>) {
    if depth == 0 {
        return (eval(&board), None);
    }
    let (moves, in_check) = board.get_moves();
    let mut best_move = moves[0].clone();
    if moves.len() == 0 {
        if in_check {
            return (i32::MIN, None)
        }
        return (0, None);
    }
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
    const PAWN_VALUE: i32 = 100;
    const KNIGHT_VALUE: i32 = 300;
    const BISHOP_VALUE: i32 = 300;
    const ROOK_VALUE: i32 = 500;
    const QUEEN_VALUE: i32 = 900;
    let mut material = 0;
    for (_, piece) in &board.pieces {
        if piece.color != color {
            continue;
        }
        match piece.r#type {
            Pawn => material += PAWN_VALUE,
            Knight => material += KNIGHT_VALUE,
            Bishop => material += BISHOP_VALUE,
            Rook => material += ROOK_VALUE,
            Queen => material += QUEEN_VALUE,
            King => (),
        }
    }
    material
}