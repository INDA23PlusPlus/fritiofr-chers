use std::collections::HashSet;

use crate::{error::FromFenError, Board, Color, Game, PieceType};

impl Game {
    /// Creates a new game from a FEN string
    ///
    /// # Arguments
    /// * `fen` - A string that holds the FEN string
    ///
    /// # Returns
    /// * `Result<game, FromFenError>` - A result that holds the game if the fen string is valid
    /// or an error if the FEN string is invalid
    ///
    /// # Examples
    /// ```
    /// use fr_chess::Game;
    ///
    /// // Starting position
    /// let game = Game::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq -");
    /// ```
    pub fn from_fen(fen: &str) -> Result<Game, FromFenError> {
        let fen_parts = fen.split(' ').collect::<Vec<&str>>();

        let fen_part_pieces = fen_parts[0];
        let fen_part_turn = fen_parts[1];
        let fen_part_castling = fen_parts[2];
        let fen_part_en_passant = fen_parts[3];

        if fen_parts.len() != 4 {
            return Err(FromFenError::IncorrectAmountOfParts);
        }

        let board = Board::from_fen(fen_part_pieces)?;

        let turn = match fen_part_turn {
            "w" => Color::White,
            "b" => Color::Black,
            _ => return Err(FromFenError::UnknownTurn),
        };

        let castling = castling_part(fen_part_castling)?;

        let en_passant = en_passant(fen_part_en_passant)?;

        let en_passant = if let Some((ep_x, ep_y)) = en_passant {
            // Because i store en passant as the tile of the pawn that can be captured,
            let ep_y = if turn == Color::White {
                ep_y + 1
            } else {
                ep_y - 1
            };

            let ocp_piece = board.get_tile(ep_x, ep_y);

            if let Some(piece) = ocp_piece {
                if piece.piece_type != PieceType::Pawn || piece.color == turn {
                    return Err(FromFenError::InvalidEnPassant);
                }
            } else {
                return Err(FromFenError::InvalidEnPassant);
            }

            Some((ep_x, ep_y))
        } else {
            None
        };

        Ok(Game {
            board,
            turn,
            en_passant,
            white_kingside_castle: castling[0],
            white_queenside_castle: castling[1],
            black_kingside_castle: castling[2],
            black_queenside_castle: castling[3],
        })
    }
}

fn castling_part(fen_part: &str) -> Result<[bool; 4], FromFenError> {
    if fen_part == "-" {
        return Ok([false; 4]);
    }

    let mut castling: [bool; 4] = [false; 4];
    let chars = fen_part.chars().collect::<Vec<char>>();

    if chars.len() > 4 {
        return Err(FromFenError::IncorrectLength);
    }

    if chars.len() != chars.iter().collect::<HashSet<&char>>().len() {
        return Err(FromFenError::RepeatingCharactersInCastlingPart);
    }

    for c in chars {
        match c {
            'K' => castling[0] = true,
            'Q' => castling[1] = true,
            'k' => castling[2] = true,
            'q' => castling[3] = true,
            _ => return Err(FromFenError::UnknownCharacter),
        }
    }

    Ok(castling)
}

fn en_passant(fen_part: &str) -> Result<Option<(usize, usize)>, FromFenError> {
    if fen_part == "-" {
        return Ok(None);
    }

    let chars = fen_part.chars().collect::<Vec<char>>();

    if chars.len() != 2 {
        return Err(FromFenError::IncorrectAmountOfTiles);
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
        _ => return Err(FromFenError::UnknownCharacter),
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
        _ => return Err(FromFenError::UnknownCharacter),
    };

    Ok(Some((file, rank)))
}
