use std::error::Error;

use crate::{Move, Piece};

#[derive(Copy, Clone)]
pub struct Board {
    tiles: [Option<Piece>; 64],
}

impl Board {
    pub fn new() -> Board {
        Board { tiles: [None; 64] }
    }

    /// Creates a new board from a fen string
    ///
    /// # Arguments
    /// * `fen` - A string that holds the fen string
    ///
    /// # Returns
    /// * `Result<Board, Box<dyn Error>>` - A result that holds the board if the fen string is valid
    /// or an error if the fen string is invalid
    ///
    /// # Examples
    /// ```
    /// // Starting position
    /// let board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");
    /// ```
    pub fn from_fen(fen: &str) -> Result<Board, Box<dyn Error>> {
        let rows = fen.split('/');

        let mut i = 0;

        let mut board = Board::new();

        for (row_index, row) in rows.enumerate() {
            for c in row.chars() {
                let parsed_value = c.to_string().parse::<usize>();

                if i >= row_index * 8 + 8 {
                    return Err(Box::new(BoardFromFenError::TooManyTiles));
                }

                if let Ok(n) = parsed_value {
                    i += n;
                } else {
                    let piece = Piece::try_from(c)?;

                    board.tiles[i] = Some(piece);

                    i += 1;
                }
            }
        }

        if i != 64 {
            return Err(Box::new(BoardFromFenError::TooManyTiles));
        }

        Ok(board)
    }

    pub fn start_pos() -> Board {
        Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR")
            .expect("This fen string is valid")
    }

    pub fn get_piece(&self, x: usize, y: usize) -> Option<Piece> {
        if x > 7 || y > 7 {
            panic!("x and y must be between 0 and 7");
        }

        let index = y * 8 + x;

        self.tiles[index]
    }

    pub fn set_piece(&mut self, x: usize, y: usize, piece: Piece) {
        if x > 7 || y > 7 {
            panic!("x and y must be between 0 and 7");
        }

        let index = y * 8 + x;

        self.tiles[index] = Some(piece);
    }

    pub fn apply_mv(&mut self, mv: Move) -> Result<(), BoardApplyMoveError> {
        if mv.board_before_move != *self {
            return Err(BoardApplyMoveError::InvalidMove);
        }

        self.tiles = mv.board_after_move.tiles;

        Ok(())
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
        let mut board_string = String::new();

        for (i, tile) in self.tiles.iter().enumerate() {
            if i % 8 == 0 && i != 0 {
                board_string.push_str("\n");
            }

            if let Some(piece) = tile {
                let piece_char: char = (*piece).into();

                board_string.push(piece_char);
            } else {
                board_string.push('-');
            }
        }

        write!(f, "{}", board_string)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum BoardFromFenError {
    #[error("FEN string has too many slashes")]
    TooManySlash,
    #[error("Unknown character in fen string")]
    UnknownCharacter,
    #[error("FEN string has too many tiles")]
    TooManyTiles,
}

#[derive(thiserror::Error, Debug)]
pub enum BoardApplyMoveError {
    #[error("The move is not valid for this board")]
    InvalidMove,
}
