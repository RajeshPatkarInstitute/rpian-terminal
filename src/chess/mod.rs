/// Represents chess piece symbols available in Unicode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChessPieceSymbol {
    // White pieces
    WhiteKing,   // ♔ U+2654
    WhiteQueen,  // ♕ U+2655
    WhiteRook,   // ♖ U+2656
    WhiteBishop, // ♗ U+2657
    WhiteKnight, // ♘ U+2658
    WhitePawn,   // ♙ U+2659

    // Black pieces
    BlackKing,   // ♚ U+265A
    BlackQueen,  // ♛ U+265B
    BlackRook,   // ♜ U+265C
    BlackBishop, // ♝ U+265D
    BlackKnight, // ♞ U+265E
    BlackPawn,   // ♟ U+265F
}

/// Converts the ChessPieceSymbol enum variant to its corresponding Unicode character.
pub fn chess_piece_to_char(piece: ChessPieceSymbol) -> char {
    match piece {
        ChessPieceSymbol::WhiteKing => '♔',
        ChessPieceSymbol::WhiteQueen => '♕',
        ChessPieceSymbol::WhiteRook => '♖',
        ChessPieceSymbol::WhiteBishop => '♗',
        ChessPieceSymbol::WhiteKnight => '♘',
        ChessPieceSymbol::WhitePawn => '♙',
        ChessPieceSymbol::BlackKing => '♚',
        ChessPieceSymbol::BlackQueen => '♛',
        ChessPieceSymbol::BlackRook => '♜',
        ChessPieceSymbol::BlackBishop => '♝',
        ChessPieceSymbol::BlackKnight => '♞',
        ChessPieceSymbol::BlackPawn => '♟',
    }
}

/// Returns true if the piece is white, false otherwise.
pub fn is_white_piece(piece: ChessPieceSymbol) -> bool {
    matches!(
        piece,
        ChessPieceSymbol::WhiteKing
            | ChessPieceSymbol::WhiteQueen
            | ChessPieceSymbol::WhiteRook
            | ChessPieceSymbol::WhiteBishop
            | ChessPieceSymbol::WhiteKnight
            | ChessPieceSymbol::WhitePawn
    )
}

/// Returns the corresponding piece of the opposite color.
pub fn opposite_color_piece(piece: ChessPieceSymbol) -> ChessPieceSymbol {
    match piece {
        ChessPieceSymbol::WhiteKing => ChessPieceSymbol::BlackKing,
        ChessPieceSymbol::WhiteQueen => ChessPieceSymbol::BlackQueen,
        ChessPieceSymbol::WhiteRook => ChessPieceSymbol::BlackRook,
        ChessPieceSymbol::WhiteBishop => ChessPieceSymbol::BlackBishop,
        ChessPieceSymbol::WhiteKnight => ChessPieceSymbol::BlackKnight,
        ChessPieceSymbol::WhitePawn => ChessPieceSymbol::BlackPawn,
        ChessPieceSymbol::BlackKing => ChessPieceSymbol::WhiteKing,
        ChessPieceSymbol::BlackQueen => ChessPieceSymbol::WhiteQueen,
        ChessPieceSymbol::BlackRook => ChessPieceSymbol::WhiteRook,
        ChessPieceSymbol::BlackBishop => ChessPieceSymbol::WhiteBishop,
        ChessPieceSymbol::BlackKnight => ChessPieceSymbol::WhiteKnight,
        ChessPieceSymbol::BlackPawn => ChessPieceSymbol::WhitePawn,
    }
}
