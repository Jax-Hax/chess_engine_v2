#[derive(Clone, Copy, Debug)]
enum Color {
    White,
    Black,
}

#[derive(Clone, Copy, Debug)]
enum PieceType {
    Bishop,
    Rook,
    Knight,
    Queen,
    King,
    Pawn(u16), // Use a u16 for promotion purposes or additional metadata
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
                    },
                    'p' => board.push(Tile { piece: Some(Piece { color: Color::Black, piece_type: PieceType::Pawn(0) }) }),
                    'r' => board.push(Tile { piece: Some(Piece { color: Color::Black, piece_type: PieceType::Rook }) }),
                    'n' => board.push(Tile { piece: Some(Piece { color: Color::Black, piece_type: PieceType::Knight }) }),
                    'b' => board.push(Tile { piece: Some(Piece { color: Color::Black, piece_type: PieceType::Bishop }) }),
                    'q' => board.push(Tile { piece: Some(Piece { color: Color::Black, piece_type: PieceType::Queen }) }),
                    'k' => board.push(Tile { piece: Some(Piece { color: Color::Black, piece_type: PieceType::King }) }),
                    'P' => board.push(Tile { piece: Some(Piece { color: Color::White, piece_type: PieceType::Pawn(0) }) }),
                    'R' => board.push(Tile { piece: Some(Piece { color: Color::White, piece_type: PieceType::Rook }) }),
                    'N' => board.push(Tile { piece: Some(Piece { color: Color::White, piece_type: PieceType::Knight }) }),
                    'B' => board.push(Tile { piece: Some(Piece { color: Color::White, piece_type: PieceType::Bishop }) }),
                    'Q' => board.push(Tile { piece: Some(Piece { color: Color::White, piece_type: PieceType::Queen }) }),
                    'K' => board.push(Tile { piece: Some(Piece { color: Color::White, piece_type: PieceType::King }) }),
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
            can_castle_white_queenside,
            can_castle_white_kingside,
            can_castle_black_queenside,
            can_castle_black_kingside,
        }
    }
}

fn main() {
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let board = Board::from_fen(fen);
    // Now `board` holds the parsed board state from the FEN string
    println!("{:#?}", board)
}