use std::error::Error;

use crate::{Color, Piece, PieceType};

mod fen_parser;

pub use fen_parser::BoardFromFenError;

mod gen_pseudo_legal_moves;

mod mv;
pub use mv::Move;

#[derive(Copy, Clone)]
pub struct Board {
    tiles: [Option<Piece>; 64],
    pub turn: Color,
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
    /// let board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq -");
    /// ```
    pub fn from_fen(fen: &str) -> Result<Board, Box<dyn Error>> {
        fen_parser::fen_parser(fen)
    }

    /// Returns a board with the starting position
    pub fn start_pos() -> Board {
        Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq -")
            .expect("This fen string is valid")
    }

    /// Returns a piece on the board
    ///
    /// # Arguments
    /// * `x` - The x coordinate of the tile
    /// * `y` - The y coordinate of the tile
    ///
    /// # Returns
    /// * `Option<Piece>` - The piece on the tile or None if there is no piece
    pub fn get_tile(&self, x: usize, y: usize) -> Option<Piece> {
        if x > 7 || y > 7 {
            panic!("x and y must be between 0 and 7");
        }

        let index = y * 8 + x;

        self.tiles[index]
    }

    /// Sets a tile on the board
    ///
    /// # Arguments
    /// * `x` - The x coordinate of the tile
    /// * `y` - The y coordinate of the tile
    /// * `piece` - The piece to set the tile to
    pub fn set_tile(&mut self, x: usize, y: usize, piece: Piece) {
        if x > 7 || y > 7 {
            panic!("x and y must be between 0 and 7");
        }

        let index = y * 8 + x;

        self.tiles[index] = Some(piece);
    }

    /// Removes a tile from the board
    ///
    /// # Arguments
    /// * `x` - The x coordinate of the tile
    /// * `y` - The y coordinate of the tile
    pub fn remove_tile(&mut self, x: usize, y: usize) {
        if x > 7 || y > 7 {
            panic!("x and y must be between 0 and 7");
        }

        let index = y * 8 + x;

        self.tiles[index] = None;
    }

    /// Applies a move to the board
    ///
    /// # Arguments
    /// * `mv` - The move to apply
    ///
    /// # Returns
    /// * `Result<(), Box<dyn Error>>` - A result that holds nothing if the move was applied
    /// successfully or an error if the move was invalid
    ///
    /// # Examples
    /// ```
    /// let mut board = Board::start_pos();
    /// // Move the pawn on e2 to e3
    /// board.apply_move(Move::Quiet { from: (4, 6), to: (4, 5) });
    /// ```
    pub fn apply_move(&mut self, mv: Move) -> Result<(), Box<dyn Error>> {
        self.en_passant = None;

        match mv {
            Move::Quiet { from, to, .. } => {
                if let Some(piece) = self.get_tile(from.0, from.1) {
                    // Check en peasant possibility
                    if piece.piece_type == PieceType::Pawn
                        && (from.1 as i32 - to.1 as i32).abs() == 2
                    {
                        self.en_passant = Some((to.0, to.1));
                    }

                    if piece.piece_type == PieceType::King {
                        if piece.color == Color::White {
                            self.white_kingside_castle = false;
                            self.white_queenside_castle = false;
                        } else {
                            self.black_kingside_castle = false;
                            self.black_queenside_castle = false;
                        }
                    }

                    if piece.piece_type == PieceType::Rook {
                        if piece.color == Color::White {
                            if from == (0, 7) {
                                self.white_queenside_castle = false;
                            } else if from == (7, 7) {
                                self.white_kingside_castle = false;
                            }
                        } else {
                            if from == (0, 0) {
                                self.black_queenside_castle = false;
                            } else if from == (7, 0) {
                                self.black_kingside_castle = false;
                            }
                        }
                    }

                    self.remove_tile(from.0, from.1);
                    self.set_tile(to.0, to.1, piece);
                } else {
                    return Err(Box::new(BoardApplyMoveError::InvalidMove));
                }
            }
            Move::Capture {
                from, to, capture, ..
            } => {
                if let Some(piece) = self.get_tile(from.0, from.1) {
                    if piece.piece_type == PieceType::King {
                        if piece.color == Color::White {
                            self.white_kingside_castle = false;
                            self.white_queenside_castle = false;
                        } else {
                            self.black_kingside_castle = false;
                            self.black_queenside_castle = false;
                        }
                    }

                    if piece.piece_type == PieceType::Rook {
                        if piece.color == Color::White {
                            if from == (0, 7) {
                                self.white_queenside_castle = false;
                            } else if from == (7, 7) {
                                self.white_kingside_castle = false;
                            }
                        } else {
                            if from == (0, 0) {
                                self.black_queenside_castle = false;
                            } else if from == (7, 0) {
                                self.black_kingside_castle = false;
                            }
                        }
                    }

                    if let Some(capture_piece) = self.get_tile(capture.0, capture.1) {
                        if capture_piece.color == Color::White {
                            if capture == (0, 7) {
                                self.white_queenside_castle = false;
                            } else if capture == (7, 7) {
                                self.white_kingside_castle = false;
                            }
                        } else {
                            if capture == (0, 0) {
                                self.black_queenside_castle = false;
                            } else if capture == (7, 0) {
                                self.black_kingside_castle = false;
                            }
                        }
                    }

                    self.remove_tile(capture.0, capture.1);
                    self.remove_tile(from.0, from.1);
                    self.set_tile(to.0, to.1, piece);
                } else {
                    return Err(Box::new(BoardApplyMoveError::InvalidMove));
                }
            }
            Move::Castle {
                from,
                to,
                rook_from,
                rook_to,
                ..
            } => {
                if let Some(king_piece) = self.get_tile(from.0, from.1) {
                    if let Some(rook_piece) = self.get_tile(rook_from.0, rook_from.1) {
                        let color = king_piece.color;

                        if color == Color::White {
                            self.white_kingside_castle = false;
                            self.white_queenside_castle = false;
                        } else {
                            self.black_kingside_castle = false;
                            self.black_queenside_castle = false;
                        }

                        self.remove_tile(from.0, from.1);
                        self.remove_tile(rook_from.0, rook_from.1);

                        self.set_tile(to.0, to.1, king_piece);
                        self.set_tile(rook_to.0, rook_to.1, rook_piece);
                    } else {
                        return Err(Box::new(BoardApplyMoveError::Debug(0)));
                    }
                } else {
                    return Err(Box::new(BoardApplyMoveError::Debug(1)));
                }
            }
            Move::QuietPromotion {
                from,
                to,
                promotion,
                ..
            } => {
                if let Some(piece) = self.get_tile(from.0, from.1) {
                    self.remove_tile(from.0, from.1);
                    self.set_tile(
                        to.0,
                        to.1,
                        Piece {
                            piece_type: promotion,
                            color: piece.color,
                        },
                    );
                } else {
                    return Err(Box::new(BoardApplyMoveError::InvalidMove));
                }
            }
            Move::CapturePromotion {
                from,
                to,
                capture,
                promotion,
                ..
            } => {
                if let Some(piece) = self.get_tile(from.0, from.1) {
                    if let Some(capture_piece) = self.get_tile(capture.0, capture.1) {
                        if capture_piece.color == Color::White {
                            if capture == (0, 7) {
                                self.white_queenside_castle = false;
                            } else if capture == (7, 7) {
                                self.white_kingside_castle = false;
                            }
                        } else {
                            if capture == (0, 0) {
                                self.black_queenside_castle = false;
                            } else if capture == (7, 0) {
                                self.black_kingside_castle = false;
                            }
                        }
                    }

                    self.remove_tile(from.0, from.1);
                    self.remove_tile(capture.0, capture.1);
                    self.set_tile(
                        to.0,
                        to.1,
                        Piece {
                            piece_type: promotion,
                            color: piece.color,
                        },
                    );
                } else {
                    return Err(Box::new(BoardApplyMoveError::InvalidMove));
                }
            }
        };

        self.turn = self.turn.opposite();

        Ok(())
    }

    /// Returns if a certain color can capture the other color's king
    fn can_capture_king(&self, color: Color) -> bool {
        self.tiles
            .iter()
            .enumerate()
            .filter(|(_, p)| p.is_some())
            .map(|(i, p)| {
                let x = i % 8;
                let y = i / 8;

                ((x, y), p.unwrap())
            })
            .filter(|(_, p)| p.color == color)
            .map(|((x, y), _)| self.gen_pseudo_legal_moves(x, y).unwrap_or(Vec::new()))
            .flatten()
            .any(|m| match m {
                Move::Capture { capture, .. } | Move::CapturePromotion { capture, .. } => {
                    self.get_tile(capture.0, capture.1).unwrap().piece_type == PieceType::King
                }
                _ => false,
            })
    }

    /// Returns if the current turn is in check
    ///
    /// # Returns
    /// * `bool` - If the current turn is in check, if it's black to move and black is in check,
    /// this will return true
    pub fn is_check(&self) -> bool {
        self.can_capture_king(self.turn.opposite())
    }

    /// Returns if the current turn is in checkmate
    ///
    /// # Returns
    /// * `bool` - If the current turn is in checkmate, if it's black to move and black is in
    /// checkmate, this will return true
    pub fn is_checkmate(&self) -> bool {
        self.is_check() && self.gen_all_moves().is_none()
    }

    /// Returns if the current turn is in stalemate
    ///
    /// # Returns
    /// * `bool` - If the current turn is in stalemate, if it's black to move and black is in
    /// stalemate, this will return true
    pub fn is_stalemate(&self) -> bool {
        !self.is_check() && self.gen_all_moves().is_none()
    }

    /// Returns all moves for the current turn
    ///
    /// # Returns
    /// * `Option<Vec<Move>>` - A vector of all the moves for the current turn, if there are no
    /// moves, this will return None
    pub fn gen_all_moves(&self) -> Option<Vec<Move>> {
        let moves = (0..64)
            .map(|i| {
                let x = i % 8;
                let y = i / 8;

                (x, y)
            })
            .map(|(x, y)| self.gen_moves(x, y).unwrap_or(Vec::new()))
            .flatten()
            .collect::<Vec<Move>>();

        if moves.len() == 0 {
            return None;
        }

        Some(moves)
    }

    /// Returns all moves for a certain tile and the current turn
    ///
    /// # Arguments
    /// * `x` - The x coordinate of the tile
    /// * `y` - The y coordinate of the tile
    ///
    /// # Returns
    /// * `Option<Vec<Move>>` - A vector of all the moves for the tile, if there are no moves, this
    /// will return None. If the piece of x and y is the opposite color of the current turn, this
    /// will return None
    pub fn gen_moves(&self, x: usize, y: usize) -> Option<Vec<Move>> {
        if let Some(piece) = self.get_tile(x, y) {
            if piece.color != self.turn {
                return None;
            }
        } else {
            return None;
        }

        let moves = self.gen_pseudo_legal_moves(x, y);

        if moves.is_none() {
            return None;
        }

        let moves = moves
            .unwrap()
            .into_iter()
            .filter(|m| {
                let mut board = self.clone();
                board
                    .apply_move(*m)
                    .expect("gen_pseudo_legal_moves only returns valid moves");

                let can_cap = !board.can_capture_king(board.turn);
                can_cap
            })
            .collect::<Vec<Move>>();

        if moves.len() == 0 {
            return None;
        }

        Some(moves)
    }

    fn gen_pseudo_legal_moves(&self, x: usize, y: usize) -> Option<Vec<Move>> {
        gen_pseudo_legal_moves::gen_pseudo_legal_moves(self, x, y, false)
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
    #[error("The move is not valid for this board")]
    Debug(u8),
}
