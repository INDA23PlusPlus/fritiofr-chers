#[derive(thiserror::Error, Debug)]
pub enum FromFenError {
    #[error("FEN string has too many or too few slashes")]
    IncorrectAmountOfSlash,
    #[error("Unknown character in fen string")]
    UnknownCharacter,
    #[error("FEN string has too many or too few tiles")]
    IncorrectAmountOfTiles,
    #[error("FEN string has too many or too few parts")]
    IncorrectAmountOfParts,
    #[error("Unknown turn")]
    UnknownTurn,
    #[error("Repeating characters in castling part")]
    RepeatingCharactersInCastlingPart,
    #[error("Incorrect length")]
    IncorrectLength,
    #[error("Invalid en passant")]
    InvalidEnPassant,
}

#[derive(thiserror::Error, Debug)]
pub enum GameApplyMoveError {
    #[error("The move is not valid for this game")]
    InvalidMove,
    #[error("The move is not valid for this game")]
    Debug(u8),
}

#[derive(thiserror::Error, Debug)]
pub enum ParsePieceError {
    #[error("Provided string is too long")]
    StringTooLong,
    #[error("Provided string is empty")]
    StringEmpty,
    #[error("Unknown character piece")]
    UnknownCharacterPiece,
}
