use std::error::Error;

use crate::{Color, Move, Piece, PieceType};

#[derive(Copy, Clone)]
pub struct Board {
    tiles: [Option<Piece>; 64],
    /// Stores a pawn that can be captured en passant
    en_passant: Option<(usize, usize)>,
    white_kingside_castle: bool,
    white_queenside_castle: bool,
    black_kingside_castle: bool,
    black_queenside_castle: bool,
}

impl Board {
    pub fn new() -> Board {
        Board {
            tiles: [None; 64],
            en_passant: None,
            white_kingside_castle: false,
            white_queenside_castle: false,
            black_kingside_castle: false,
            black_queenside_castle: false,
        }
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

    pub fn remove_piece(&mut self, x: usize, y: usize) {
        if x > 7 || y > 7 {
            panic!("x and y must be between 0 and 7");
        }

        let index = y * 8 + x;

        self.tiles[index] = None;
    }

    pub fn apply_move(&mut self, mv: Move) -> Result<(), Box<dyn Error>> {
        match mv {
            Move::Quiet { from, to } => {
                if let Some(piece) = self.get_piece(from.0, from.1) {
                    self.remove_piece(from.0, from.1);
                    self.set_piece(to.0, to.1, piece);
                } else {
                    return Err(Box::new(BoardApplyMoveError::InvalidMove));
                }
            }
            Move::Capture { from, to, capture } => {
                if let Some(piece) = self.get_piece(from.0, from.1) {
                    self.remove_piece(capture.0, capture.1);
                    self.remove_piece(from.0, from.1);
                    self.set_piece(to.0, to.1, piece);
                } else {
                    return Err(Box::new(BoardApplyMoveError::InvalidMove));
                }
            }
            Move::Castle {
                from,
                to,
                rook_from,
                rook_to,
                side,
            } => todo!(),
            Move::QuietPromotion {
                from,
                to,
                promotion,
            } => todo!(),
            Move::CapturePromotion {
                from,
                to,
                capture,
                promotion,
            } => todo!(),
            _ => todo!(),
        };

        Ok(())
    }

    fn gen_tile_moves_ign_check(
        &self,
        x: usize,
        y: usize,
    ) -> Result<Option<Vec<Move>>, Box<dyn Error>> {
        let piece = self.get_piece(x, y);

        if piece.is_none() {
            return Ok(None);
        }

        let piece = piece.expect("Already checked that this is not none");

        let moves = match piece.piece_type {
            PieceType::Pawn => {
                todo!()
            }
            PieceType::Rook | PieceType::Bishop | PieceType::Queen | PieceType::Knight => {
                // The idea is to loop around and spread out to find all the moves
                let mut moves = Vec::new();

                let (dirs, depth) = match piece.piece_type {
                    PieceType::Rook => (vec![(-1, 0), (1, 0), (0, -1), (0, 1)], 8),
                    PieceType::Bishop => (vec![(-1, -1), (-1, 1), (1, -1), (1, 1)], 8),
                    PieceType::Queen => (
                        vec![
                            (-1, 0),
                            (1, 0),
                            (0, -1),
                            (0, 1),
                            (-1, -1),
                            (-1, 1),
                            (1, -1),
                            (1, 1),
                        ],
                        8,
                    ),
                    PieceType::Knight => (
                        vec![
                            (-2, -1),
                            (-2, 1),
                            (-1, -2),
                            (-1, 2),
                            (1, -2),
                            (1, 2),
                            (2, -1),
                            (2, 1),
                        ],
                        1,
                    ),
                    _ => unreachable!(),
                };

                for dir in dirs {
                    for i in 0..depth {
                        let c_x = dir.0 * i + x as i32;
                        let c_y = dir.1 * i + y as i32;

                        if c_x < 0 || c_x > 7 || c_y < 0 || c_y > 7 {
                            break;
                        }

                        let c_x = c_x as usize;
                        let c_y = c_y as usize;

                        let oc_piece = self.get_piece(c_x, c_y);

                        match oc_piece {
                            Some(oc_piece) => {
                                if oc_piece.color != piece.color {
                                    moves.push(Move::Capture {
                                        from: (x, y),
                                        to: (c_x, c_y),
                                        capture: (c_x, c_y),
                                    });
                                }

                                break;
                            }
                            None => {
                                moves.push(Move::Quiet {
                                    from: (x, y),
                                    to: (c_x, c_y),
                                });
                            }
                        }
                    }
                }

                moves
            }
            _ => todo!(),
        };

        Ok(Some(moves))
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
