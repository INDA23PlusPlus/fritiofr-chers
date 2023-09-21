use std::error::Error;

use crate::{error::GameApplyMoveError, Color, Game, Move, Piece, PieceType};

impl Game {
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
    /// use fr_chess::{Game, Move};
    ///
    /// let mut game = Game::start_pos();
    /// // Move the pawn on e2 to e3
    /// game.apply_move(Move::Quiet { from: (4, 6), to: (4, 5) });
    /// ```
    pub fn apply_move(&mut self, mv: Move) -> Result<(), Box<dyn Error>> {
        self.en_passant = None;

        match mv {
            Move::Quiet { from, to, .. } => {
                if let Some(piece) = self.board.get_tile(from.0, from.1) {
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

                    self.board.remove_tile(from.0, from.1);
                    self.board.set_tile(to.0, to.1, piece);
                } else {
                    return Err(Box::new(GameApplyMoveError::InvalidMove));
                }
            }
            Move::Capture {
                from, to, capture, ..
            } => {
                if let Some(piece) = self.board.get_tile(from.0, from.1) {
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

                    if let Some(capture_piece) = self.board.get_tile(capture.0, capture.1) {
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

                    self.board.remove_tile(capture.0, capture.1);
                    self.board.remove_tile(from.0, from.1);
                    self.board.set_tile(to.0, to.1, piece);
                } else {
                    return Err(Box::new(GameApplyMoveError::InvalidMove));
                }
            }
            Move::Castle {
                from,
                to,
                rook_from,
                rook_to,
                ..
            } => {
                if let Some(king_piece) = self.board.get_tile(from.0, from.1) {
                    if let Some(rook_piece) = self.board.get_tile(rook_from.0, rook_from.1) {
                        let color = king_piece.color;

                        if color == Color::White {
                            self.white_kingside_castle = false;
                            self.white_queenside_castle = false;
                        } else {
                            self.black_kingside_castle = false;
                            self.black_queenside_castle = false;
                        }

                        self.board.remove_tile(from.0, from.1);
                        self.board.remove_tile(rook_from.0, rook_from.1);

                        self.board.set_tile(to.0, to.1, king_piece);
                        self.board.set_tile(rook_to.0, rook_to.1, rook_piece);
                    } else {
                        return Err(Box::new(GameApplyMoveError::Debug(0)));
                    }
                } else {
                    return Err(Box::new(GameApplyMoveError::Debug(1)));
                }
            }
            Move::QuietPromotion {
                from,
                to,
                promotion,
                ..
            } => {
                if let Some(piece) = self.board.get_tile(from.0, from.1) {
                    self.board.remove_tile(from.0, from.1);
                    self.board.set_tile(
                        to.0,
                        to.1,
                        Piece {
                            piece_type: promotion,
                            color: piece.color,
                        },
                    );
                } else {
                    return Err(Box::new(GameApplyMoveError::InvalidMove));
                }
            }
            Move::CapturePromotion {
                from,
                to,
                capture,
                promotion,
                ..
            } => {
                if let Some(piece) = self.board.get_tile(from.0, from.1) {
                    if let Some(capture_piece) = self.board.get_tile(capture.0, capture.1) {
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

                    self.board.remove_tile(from.0, from.1);
                    self.board.remove_tile(capture.0, capture.1);
                    self.board.set_tile(
                        to.0,
                        to.1,
                        Piece {
                            piece_type: promotion,
                            color: piece.color,
                        },
                    );
                } else {
                    return Err(Box::new(GameApplyMoveError::InvalidMove));
                }
            }
        };

        self.turn = self.turn.opposite();

        Ok(())
    }
}
