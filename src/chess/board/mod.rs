use std::error::Error;

use crate::{Color, Move, Piece, PieceType};

mod fen_parser;

pub use fen_parser::BoardFromFenError;

use super::mv::DOUBLE_PAWN_PUSH;

#[derive(Copy, Clone)]
pub struct Board {
    tiles: [Option<Piece>; 64],
    pub turn: Color,
    halfmove: usize,
    fullmove: usize,
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
            turn: Color::White,
            halfmove: 0,
            fullmove: 1,
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
    /// let board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    /// ```
    pub fn from_fen(fen: &str) -> Result<Board, Box<dyn Error>> {
        fen_parser::fen_parser(fen)
    }

    pub fn start_pos() -> Board {
        Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
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

    /// Takes a move and applies it on the board
    ///
    /// # Arguments
    ///
    pub fn apply_move(&mut self, mv: Move) -> Result<(), Box<dyn Error>> {
        match mv {
            Move::Quiet { from, to, .. } => {
                if let Some(piece) = self.get_piece(from.0, from.1) {
                    // Check en peasant possibility
                    if mv.is_double_pawn_push() {
                        self.en_passant = Some((to.0, to.1));
                    }

                    self.remove_piece(from.0, from.1);
                    self.set_piece(to.0, to.1, piece);
                } else {
                    return Err(Box::new(BoardApplyMoveError::InvalidMove));
                }
            }
            Move::Capture {
                from, to, capture, ..
            } => {
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
                ..
            } => todo!(),
            Move::QuietPromotion {
                from,
                to,
                promotion,
                ..
            } => todo!(),
            Move::CapturePromotion {
                from,
                to,
                capture,
                promotion,
                ..
            } => todo!(),
            _ => todo!(),
        };

        // Reset en passant since it is only valid for one move
        self.en_passant = None;

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
                let mut moves = Vec::new();

                let final_rank = if piece.color == Color::White { 7 } else { 0 };
                let dir = if piece.color == Color::White { -1 } else { 1 };
                let starting_rank = if piece.color == Color::White { 6 } else { 1 };
                let promotion_pieces = [
                    PieceType::Queen,
                    PieceType::Rook,
                    PieceType::Bishop,
                    PieceType::Knight,
                ];

                // Regular move forwards
                {
                    let c_x = x as i32;
                    let c_y = y as i32 + dir;

                    if c_y >= 0 || c_y <= 7 {
                        let c_x = c_x as usize;
                        let c_y = c_y as usize;

                        let oc_piece = self.get_piece(c_x as usize, c_y as usize);

                        if oc_piece.is_none() {
                            if c_y == final_rank {
                                moves.append(
                                    &mut promotion_pieces
                                        .iter()
                                        .map(|&p| Move::QuietPromotion {
                                            from: (x, y),
                                            to: (c_x, c_y),
                                            promotion: p,
                                            flags: 0,
                                        })
                                        .collect(),
                                );
                            } else {
                                moves.push(Move::Quiet {
                                    from: (x, y),
                                    to: (c_x, c_y),
                                    flags: 0,
                                });
                            }
                        }
                    }
                }

                // Double forwards
                {
                    let c_x = x as i32;
                    let c_y = y as i32 + dir * 2;

                    if c_y >= 0 || c_y <= 7 && y == starting_rank {
                        let c_x = c_x as usize;
                        let c_y = c_y as usize;

                        let oc_piece_further = self.get_piece(c_x as usize, c_y as usize);
                        let oc_piece_close =
                            self.get_piece(c_x as usize, c_y as usize - dir as usize);

                        if oc_piece_further.is_none() && oc_piece_close.is_none() {
                            moves.push(Move::Quiet {
                                from: (x, y),
                                to: (c_x, c_y),
                                flags: DOUBLE_PAWN_PUSH,
                            });
                        }
                    }
                }

                // Capture
                {
                    for x_dir in [-1, 1] {
                        let c_x = x as i32 + x_dir;
                        let c_y = y as i32 + dir;

                        if c_y >= 0 || c_y <= 7 && c_x >= 0 || c_x <= 7 {
                            let c_x = c_x as usize;
                            let c_y = c_y as usize;

                            let oc_piece = self.get_piece(c_x as usize, c_y as usize);

                            if oc_piece.is_some() && oc_piece.unwrap().color != piece.color {
                                if c_y == final_rank {
                                    moves.append(
                                        &mut promotion_pieces
                                            .iter()
                                            .map(|&p| Move::CapturePromotion {
                                                from: (x, y),
                                                to: (c_x, c_y),
                                                capture: (c_x, c_y),
                                                promotion: p,
                                                flags: 0,
                                            })
                                            .collect(),
                                    );
                                } else {
                                    moves.push(Move::Capture {
                                        from: (x, y),
                                        to: (c_x, c_y),
                                        capture: (c_x, c_y),
                                        flags: 0,
                                    });
                                }
                            }
                        }
                    }
                }

                // En passant
                {
                    if let Some((ep_x, ep_y)) = self.en_passant {
                        for x_dir in [-1, 1] {
                            let c_x = x as i32 + x_dir;
                            let c_y = y as i32 + dir;

                            if c_y >= 0 || c_y <= 7 && c_x >= 0 || c_x <= 7 {
                                let c_x = c_x as usize;
                                let c_y = c_y as usize;

                                if (c_x, y) == (ep_x, ep_y) {
                                    moves.push(Move::Capture {
                                        from: (x, y),
                                        to: (c_x, c_y),
                                        capture: (ep_x, ep_y),
                                        flags: 0,
                                    });
                                }
                            }
                        }
                    }
                }

                moves
            }
            PieceType::Rook
            | PieceType::Bishop
            | PieceType::Queen
            | PieceType::Knight
            | PieceType::King => {
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
                    PieceType::King => (
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
                                        flags: 0,
                                    });
                                }

                                break;
                            }
                            None => {
                                moves.push(Move::Quiet {
                                    from: (x, y),
                                    to: (c_x, c_y),
                                    flags: 0,
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
pub enum BoardApplyMoveError {
    #[error("The move is not valid for this board")]
    InvalidMove,
}
