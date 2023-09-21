use std::str::FromStr;

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: Color,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Copy, Debug, Clone, PartialEq, Eq)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub fn opposite(&self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

impl Into<char> for Piece {
    fn into(self) -> char {
        let piece_char = match self.piece_type {
            PieceType::Pawn => 'p',
            PieceType::Knight => 'n',
            PieceType::Bishop => 'b',
            PieceType::Rook => 'r',
            PieceType::Queen => 'q',
            PieceType::King => 'k',
        };

        if self.color == Color::White {
            piece_char.to_ascii_uppercase()
        } else {
            piece_char
        }
    }
}

impl TryFrom<char> for Piece {
    type Error = ParsePieceError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let piece_type = match value.to_ascii_lowercase() {
            'p' => PieceType::Pawn,
            'n' => PieceType::Knight,
            'b' => PieceType::Bishop,
            'r' => PieceType::Rook,
            'q' => PieceType::Queen,
            'k' => PieceType::King,
            _ => return Err(ParsePieceError::UnknownCharacterPiece),
        };

        let color = if value.is_ascii_uppercase() {
            Color::White
        } else {
            Color::Black
        };

        Ok(Piece { piece_type, color })
    }
}

impl FromStr for Piece {
    type Err = ParsePieceError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() > 1 {
            return Err(ParsePieceError::StringTooLong);
        }

        let piece = s
            .chars()
            .next()
            .ok_or(ParsePieceError::StringEmpty)
            .and_then(|c| Piece::try_from(c));

        piece
    }
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
