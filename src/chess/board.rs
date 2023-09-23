use crate::{error::FromFenError, Color, Piece, PieceType};

/// A chess board
#[derive(Debug, Copy, Clone)]
pub struct Board {
    pub(crate) tiles: [Option<Piece>; 64],
}

impl Board {
    /// Parses the board part of a FEN string
    ///
    /// # Arguments
    /// * `fen` - The board part of a FEN string
    ///
    /// # Examples
    /// ```
    /// use fritiofr_chess::Board;
    ///
    /// // The starting position
    /// Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");
    /// ```
    pub fn from_fen(fen: &str) -> Result<Board, FromFenError> {
        let mut tiles: [Option<Piece>; 64] = [None; 64];

        let rows = fen.split('/').collect::<Vec<&str>>();

        if rows.len() != 8 {
            return Err(FromFenError::IncorrectAmountOfSlash);
        }

        let mut i = 0;
        for (row_index, row) in rows.iter().enumerate() {
            for c in row.chars() {
                let parsed_value = c.to_string().parse::<usize>();

                if i >= row_index * 8 + 8 {
                    return Err(FromFenError::IncorrectAmountOfTiles);
                }

                if let Ok(n) = parsed_value {
                    i += n;
                } else {
                    let piece = Piece::try_from(c).map_err(|_| FromFenError::UnknownCharacter)?;

                    tiles[i] = Some(piece);

                    i += 1;
                }
            }
        }

        if i != 64 {
            return Err(FromFenError::IncorrectAmountOfTiles);
        }

        Ok(Board { tiles })
    }

    /// Returns the position of the king of a color
    pub fn get_king_pos(&self, color: Color) -> Option<(usize, usize)> {
        (0..8)
            .map(|x| (0..8).map(move |y| (x, y)))
            .flatten()
            .find(|(x, y)| {
                if let Some(piece) = self.get_tile(*x, *y) {
                    return piece.color == color && piece.piece_type == PieceType::King;
                }
                false
            })
    }

    /// Returns a piece on the board
    pub fn get_tile(&self, x: usize, y: usize) -> Option<Piece> {
        if x > 7 || y > 7 {
            panic!("x and y must be between 0 and 7");
        }

        let index = y * 8 + x;

        self.tiles[index]
    }

    /// Sets a tile on the board
    pub fn set_tile(&mut self, x: usize, y: usize, piece: Piece) {
        if x > 7 || y > 7 {
            panic!("x and y must be between 0 and 7");
        }

        let index = y * 8 + x;

        self.tiles[index] = Some(piece);
    }

    /// Removes a tile from the board
    pub fn remove_tile(&mut self, x: usize, y: usize) {
        if x > 7 || y > 7 {
            panic!("x and y must be between 0 and 7");
        }

        let index = y * 8 + x;

        self.tiles[index] = None;
    }

    /// Returns the board as a FEN string
    pub fn fen(&self) -> String {
        let mut fen = String::new();

        let mut empty_tiles = 0;

        for (i, tile) in self.tiles.into_iter().enumerate() {
            if i % 8 == 0 && i != 0 {
                if empty_tiles != 0 {
                    fen.push_str(&empty_tiles.to_string());
                    empty_tiles = 0;
                }

                fen.push('/');
            }

            if let Some(piece) = tile {
                if empty_tiles != 0 {
                    fen.push_str(&empty_tiles.to_string());
                    empty_tiles = 0;
                }

                let piece_char: char = piece.into();

                fen.push(piece_char);
            } else {
                empty_tiles += 1;
            }
        }

        if empty_tiles != 0 {
            fen.push_str(&empty_tiles.to_string());
        }

        fen
    }
}

impl Eq for Board {}
impl PartialEq for Board {
    fn eq(&self, other: &Self) -> bool {
        self.tiles
            .into_iter()
            .zip(other.tiles.into_iter())
            .all(|(a, b)| a == b)
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut game_string = String::new();

        for (i, tile) in self.tiles.iter().enumerate() {
            if i % 8 == 0 && i != 0 {
                game_string.push_str("\n");
            }

            if let Some(piece) = tile {
                let piece_char: char = (*piece).into();

                game_string.push(piece_char);
            } else {
                game_string.push('-');
            }
        }

        write!(f, "{}", game_string)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn fen_should_be_same_as_from_fen() {
        let fens_to_test = vec![
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR",
            "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1",
            "5bnr/pp1ppppp/nbrp4/1k3QN1/2B1q3/6N1/PPPRPPPP/R1B1K3",
        ];

        for fen in fens_to_test {
            let board = Board::from_fen(fen).unwrap();
            assert_eq!(board.fen(), fen);
        }
    }
}
