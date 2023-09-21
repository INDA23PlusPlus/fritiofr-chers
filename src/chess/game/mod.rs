use std::error::Error;

use crate::{Board, Color, Piece, PieceType};

mod from_fen;
mod gen_pseudo_legal_moves;
use super::Move;

#[derive(Copy, Clone)]
pub struct Game {
    board: Board,
    pub turn: Color,
    /// Stores a pawn that can be captured en passant
    en_passant: Option<(usize, usize)>,

    white_kingside_castle: bool,
    white_queenside_castle: bool,
    black_kingside_castle: bool,
    black_queenside_castle: bool,
}

impl Game {
    /// Returns a game with the starting position
    pub fn start_pos() -> Game {
        Game::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq -")
            .expect("This fen string is valid")
    }

    /// Applies a move to the game
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
    /// let mut game = game::start_pos();
    /// // Move the pawn on e2 to e3
    /// game.apply_move(Move::Quiet { from: (4, 6), to: (4, 5) });
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

                    // Remove castling rights if a rook is moved
                    // If there is another from 0, 7 the rook has already moved and we can set it
                    // to false either way. No need to check that the move was actually a rook
                    if from == (0, 7) {
                        self.white_queenside_castle = false;
                    } else if from == (7, 7) {
                        self.white_kingside_castle = false;
                    } else if from == (0, 0) {
                        self.black_queenside_castle = false;
                    } else if from == (7, 0) {
                        self.black_kingside_castle = false;
                    }

                    self.remove_tile(from.0, from.1);
                    self.set_tile(to.0, to.1, piece);
                } else {
                    return Err(Box::new(gameApplyMoveError::InvalidMove));
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
                    return Err(Box::new(gameApplyMoveError::InvalidMove));
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
                        return Err(Box::new(gameApplyMoveError::Debug(0)));
                    }
                } else {
                    return Err(Box::new(gameApplyMoveError::Debug(1)));
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
                    return Err(Box::new(gameApplyMoveError::InvalidMove));
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
                    return Err(Box::new(gameApplyMoveError::InvalidMove));
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
                let mut game = self.clone();
                game.apply_move(*m)
                    .expect("gen_pseudo_legal_moves only returns valid moves");

                let can_cap = !game.can_capture_king(game.turn);
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

#[derive(thiserror::Error, Debug)]
pub enum gameApplyMoveError {
    #[error("The move is not valid for this game")]
    InvalidMove,
    #[error("The move is not valid for this game")]
    Debug(u8),
}
