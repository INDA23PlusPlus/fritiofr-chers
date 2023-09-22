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
    /// use fritiofr_chess::{Game, Move};
    ///
    /// let mut game = Game::start_pos();
    /// // Move the pawn on e2 to e3
    /// game.apply_move(Move::Quiet { from: (4, 6), to: (4, 5) });
    /// ```
    pub fn apply_move(&mut self, mv: Move) -> Result<(), GameApplyMoveError> {
        self.en_passant = None;

        if mv.is_capture() {
            let (c_x, c_y) = mv.capture().expect("This is a capture move");

            self.board.remove_tile(c_x, c_y);
            remove_castling_rights_pos(self, (c_x, c_y));
        }

        let (to_x, to_y) = mv.to();
        let (from_x, from_y) = mv.from();

        let piece = self
            .board
            .get_tile(from_x, from_y)
            .ok_or(GameApplyMoveError::InvalidMove)?;

        self.board.remove_tile(from_x, from_y);
        self.board.set_tile(to_x, to_y, piece);

        // Remove castling rights if the type is king
        if piece.piece_type == PieceType::King {
            remove_castling_rights_color(self, piece.color);
        }

        // Remove castling right if we move from a corner
        remove_castling_rights_pos(self, (from_x, from_y));

        // Set en passant
        if mv.is_double_pawn_push() {
            self.en_passant = Some((to_x, to_y));
        }

        match mv {
            Move::Castle {
                rook_from, rook_to, ..
            } => {
                if let Some(rook_piece) = self.board.get_tile(rook_from.0, rook_from.1) {
                    remove_castling_rights_color(self, rook_piece.color);

                    self.board.remove_tile(rook_from.0, rook_from.1);
                    self.board.set_tile(rook_to.0, rook_to.1, rook_piece);
                } else {
                    return Err(GameApplyMoveError::InvalidMove);
                }
            }
            Move::QuietPromotion { .. } | Move::CapturePromotion { .. } => {
                let piece = self
                    .board
                    .get_tile(to_x, to_y)
                    .expect("The to tile is set further up in this function");

                self.board.set_tile(
                    to_x,
                    to_y,
                    Piece {
                        piece_type: mv.promotion().expect("This is a promotion move"),
                        color: piece.color,
                    },
                );
            }
            _ => (),
        };

        self.turn = self.turn.opposite();

        Ok(())
    }
}

/// Internal helper that takes a position in one of the corners and removes the castling rights
/// from that corner
fn remove_castling_rights_pos(game: &mut Game, pos: (usize, usize)) {
    if pos == (0, 7) {
        game.white_queenside_castle = false;
    } else if pos == (7, 7) {
        game.white_kingside_castle = false;
    } else if pos == (0, 0) {
        game.black_queenside_castle = false;
    } else if pos == (7, 0) {
        game.black_kingside_castle = false;
    }
}

/// Internal helper that takes a color and removes the castling rights from that color
fn remove_castling_rights_color(game: &mut Game, color: Color) {
    if color == Color::White {
        game.white_kingside_castle = false;
        game.white_queenside_castle = false;
    } else {
        game.black_kingside_castle = false;
        game.black_queenside_castle = false;
    }
}
