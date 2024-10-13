use structs::{Board, Color, PieceType::*};

mod board;
mod structs;
mod play;
mod fen;

fn main() {
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let mut board = Board::from_fen(fen.to_string()).unwrap();
    search(&mut board, 6);
}

fn perft(board: &mut Board, depth: usize) -> usize {
    if depth == 0 {
        return 1;
    }

    let mut count = 0;

    for r#move in board.get_moves().0 {
        let castling_rights = board.castling_rights.clone();
        let enpassant_square = board.enpassant_square;
        let halfmove_clock = board.halfmove_clock;

        board.execute(r#move);

        let perft = perft(board, depth - 1);
        count += perft;

        board.undo(castling_rights, enpassant_square, halfmove_clock);
    }

    count
}
fn search(board: &mut Board, depth: usize, mut alpha: i32, beta: i32) -> i32 {
    if depth == 0 {
        return eval(&board);
    }

    let (moves, in_check) = board.get_moves();
    if moves.len() == 0 {
        if in_check {
            return i32::MIN
        }
        return 0;
    }
    for r#move in moves {
        let castling_rights = board.castling_rights.clone();
        let enpassant_square = board.enpassant_square;
        let halfmove_clock = board.halfmove_clock;

        board.execute(r#move);

        let evaluation = -search(board, depth - 1, -beta, -alpha);

        board.undo(castling_rights, enpassant_square, halfmove_clock);
        if evaluation >= beta {
            return beta;
        }
        alpha = alpha.max(evaluation)
    }

    alpha
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