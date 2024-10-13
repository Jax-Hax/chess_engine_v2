use std::io::stdin;

use engine::search;
use structs::{Board, File, PieceType::*, Rank, Square};

mod board;
mod engine;
mod fen;
mod play;
mod structs;

fn main() {
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let mut board = Board::from_fen(fen.to_string()).unwrap();
    game_loop(&mut board);
}

fn game_loop(board: &mut Board) {
    board.print_board();
    loop {
        println!("Current turn: {:?}", board.fullmove_number);

        // Generate all valid moves for the current turn
        let (moves, in_check) = board.get_moves();
        if moves.len() == 0 {
            if in_check {
                println!("Checkmate! Game over!");
            }
            println!("Stalemate! Game over!");
            return;
        }

        let mut from;
        let mut to;
        let mut move_is_valid = false;
        while !move_is_valid {
            from = String::new();
            to = String::new();
            println!("Enter the 'from' position (e.g., e2): ");
            stdin().read_line(&mut from).expect("Failed to read input");
            let from = from.trim();

            println!("Enter the 'to' position (e.g., e4): ");
            stdin().read_line(&mut to).expect("Failed to read input");
            let to = to.trim();

            // Convert chess notation to board index
            let from_idx = chess_notation_to_square(from);
            let to_idx = chess_notation_to_square(to);

            match (from_idx, to_idx) {
                (Some(from), Some(to)) => {
                    let move_played = moves.iter().find(|e| e.from == from && e.to == to);
                    match move_played {
                        Some(move_exists) => {
                            move_is_valid = true;
                            board.execute(move_exists.clone());
                        }
                        _ => {
                            println!("Cannot play that move. Try again.");
                        }
                    }
                }
                _ => println!("Invalid chess notation. Try again."),
            }
        }
        board.print_board();
        println!();
        println!("The AI is thinking...");
        println!();
        let best_move = search(board, 6, i32::MIN, i32::MAX).1.unwrap();
        board.execute(best_move.clone());
        board.print_board();
        println!("The AI played a move: {} to {}", best_move.from, best_move.to);
    }
}

fn chess_notation_to_square(pos: &str) -> Option<Square> {
    // Ensure the string has exactly 2 characters
    if pos.len() != 2 {
        return None;
    }
    // Extract the file and rank characters
    let file_char = pos.chars().nth(0).unwrap().to_ascii_lowercase();
    let rank_char = pos.chars().nth(1).unwrap();

    // Convert the file character to the File enum
    let file = match file_char {
        'a' => File::A,
        'b' => File::B,
        'c' => File::C,
        'd' => File::D,
        'e' => File::E,
        'f' => File::F,
        'g' => File::G,
        'h' => File::H,
        _ => return None,
    };

    // Convert the rank character to the Rank enum
    let rank = match rank_char {
        '1' => Rank::_1,
        '2' => Rank::_2,
        '3' => Rank::_3,
        '4' => Rank::_4,
        '5' => Rank::_5,
        '6' => Rank::_6,
        '7' => Rank::_7,
        '8' => Rank::_8,
        _ => return None,
    };

    // Return the parsed Square
    Some(Square { file, rank })
}
fn _perft(board: &mut Board, depth: usize) -> usize {
    if depth == 0 {
        return 1;
    }

    let mut count = 0;

    for r#move in board.get_moves().0 {
        let castling_rights = board.castling_rights.clone();
        let enpassant_square = board.enpassant_square;
        let halfmove_clock = board.halfmove_clock;

        board.execute(r#move);

        let perft = _perft(board, depth - 1);
        count += perft;

        board.undo(castling_rights, enpassant_square, halfmove_clock);
    }

    count
}
