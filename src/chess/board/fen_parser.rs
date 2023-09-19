use std::{collections::HashSet, error::Error};

use crate::{Board, Color, Piece, PieceType};

#[derive(thiserror::Error, Debug)]
pub enum BoardFromFenError {
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

/// Internal function to split out code from the `Board` struct.
/// This function will parse a FEN string and return a `Board` struct.
pub fn fen_parser(fen: &str) -> Result<Board, Box<dyn Error>> {
    let fen_parts = fen.split(' ').collect::<Vec<&str>>();

    let fen_part_pieces = fen_parts[0];
    let fen_part_turn = fen_parts[1];
    let fen_part_castling = fen_parts[2];
    let fen_part_en_passant = fen_parts[3];
    let fen_part_halfmove = fen_parts[4];
    let fen_part_fullmove = fen_parts[5];

    if fen_parts.len() != 6 {
        return Err(Box::new(BoardFromFenError::IncorrectAmountOfParts));
    }

    let tiles = piece_placement_part(fen_part_pieces)?;

    let turn = match fen_part_turn {
        "w" => Color::White,
        "b" => Color::Black,
        _ => return Err(Box::new(BoardFromFenError::UnknownTurn)),
    };

    let castling = castling_part(fen_part_castling)?;

    let en_passant = en_passant(fen_part_en_passant)?;

    let halfmove = fen_part_halfmove.parse::<usize>()?;
    let fullmove = fen_part_fullmove.parse::<usize>()?;

    if let Some((ep_x, ep_y)) = en_passant {
        // Because i store en passant as the tile of the pawn that can be captured,
        let ep_y = if turn == Color::White {
            ep_y + 1
        } else {
            ep_y - 1
        };

        let ocp_piece = tiles[ep_x + ep_y * 8];

        if let Some(piece) = ocp_piece {
            if piece.piece_type != PieceType::Pawn || piece.color == turn {
                return Err(Box::new(BoardFromFenError::InvalidEnPassant));
            }
        } else {
            return Err(Box::new(BoardFromFenError::InvalidEnPassant));
        }
    }

    Ok(Board {
        tiles,
        turn,
        halfmove,
        fullmove,
        en_passant,
        white_kingside_castle: castling[0],
        white_queenside_castle: castling[1],
        black_kingside_castle: castling[2],
        black_queenside_castle: castling[3],
    })
}

/// Parses the first part of a FEN string, which is the piece placement.
fn piece_placement_part(fen_part: &str) -> Result<[Option<Piece>; 64], Box<dyn Error>> {
    let mut tiles: [Option<Piece>; 64] = [None; 64];

    let rows = fen_part.split('/').collect::<Vec<&str>>();

    if rows.len() != 8 {
        return Err(Box::new(BoardFromFenError::IncorrectAmountOfSlash));
    }

    let mut i = 0;
    for (row_index, row) in rows.iter().enumerate() {
        for c in row.chars() {
            let parsed_value = c.to_string().parse::<usize>();

            if i >= row_index * 8 + 8 {
                return Err(Box::new(BoardFromFenError::IncorrectAmountOfTiles));
            }

            if let Ok(n) = parsed_value {
                i += n;
            } else {
                let piece = Piece::try_from(c)
                    .map_err(|_| Box::new(BoardFromFenError::UnknownCharacter))?;

                tiles[i] = Some(piece);

                i += 1;
            }
        }
    }

    if i != 64 {
        return Err(Box::new(BoardFromFenError::IncorrectAmountOfTiles));
    }

    Ok(tiles)
}

fn castling_part(fen_part: &str) -> Result<[bool; 4], Box<dyn Error>> {
    if fen_part == "-" {
        return Ok([false; 4]);
    }

    let mut castling: [bool; 4] = [false; 4];
    let chars = fen_part.chars().collect::<Vec<char>>();

    if chars.len() > 4 {
        return Err(Box::new(BoardFromFenError::IncorrectLength));
    }

    if chars.len() != chars.iter().collect::<HashSet<&char>>().len() {
        return Err(Box::new(
            BoardFromFenError::RepeatingCharactersInCastlingPart,
        ));
    }

    for c in chars {
        match c {
            'K' => castling[0] = true,
            'Q' => castling[1] = true,
            'k' => castling[2] = true,
            'q' => castling[3] = true,
            _ => return Err(Box::new(BoardFromFenError::UnknownCharacter)),
        }
    }

    Ok(castling)
}

fn en_passant(fen_part: &str) -> Result<Option<(usize, usize)>, Box<dyn Error>> {
    if fen_part == "-" {
        return Ok(None);
    }

    let chars = fen_part.chars().collect::<Vec<char>>();

    if chars.len() != 2 {
        return Err(Box::new(BoardFromFenError::IncorrectAmountOfTiles));
    }

    let file = match chars[0] {
        'a' => 0,
        'b' => 1,
        'c' => 2,
        'd' => 3,
        'e' => 4,
        'f' => 5,
        'g' => 6,
        'h' => 7,
        _ => return Err(Box::new(BoardFromFenError::UnknownCharacter)),
    };

    // Yes, this is super odd, but i accidentally made the board uppside down, oops...
    let rank = match chars[1] {
        '1' => 7,
        '2' => 6,
        '3' => 5,
        '4' => 4,
        '5' => 3,
        '6' => 2,
        '7' => 1,
        '8' => 0,
        _ => return Err(Box::new(BoardFromFenError::UnknownCharacter)),
    };

    Ok(Some((file, rank)))
}
