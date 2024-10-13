use structs::Board;

mod board;
mod structs;
mod play;
mod fen;

fn main() {
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let mut board = Board::from_fen(fen.to_string()).unwrap();
    println!("{}", perft(&mut board, 6));
}
fn eval(board: &mut Board, depth: usize) -> usize {
    if depth == 0 {
        return 1;
    }

    let mut count = 0;

    for r#move in board.get_moves() {
        let castling_rights = board.castling_rights.clone();
        let enpassant_square = board.enpassant_square;
        let halfmove_clock = board.halfmove_clock;

        board.execute(r#move);

        let perft = eval(board, depth - 1);
        count += perft;

        board.undo(castling_rights, enpassant_square, halfmove_clock);
    }

    count
}
fn perft(board: &mut Board, depth: usize) -> usize {
    if depth == 0 {
        return 1;
    }

    let mut count = 0;

    for r#move in board.get_moves() {
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