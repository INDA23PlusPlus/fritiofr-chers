use crate::{Board, Color, PieceType};

mod apply_move;
mod from_fen;
mod gen_pseudo_legal_moves;
use super::Move;

/// A game of chess
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Game {
    board: Board,
    turn: Color,
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

    /// Returns the Board that for the game
    ///
    /// # Return
    /// * `Board` - The board for the game
    pub fn get_board(&self) -> Board {
        self.board
    }

    /// Sets the Board for the game
    ///
    /// **This will reset en passant and castling**
    ///
    /// # Arguments
    /// * `board` - The board to set
    pub fn set_board(&mut self, board: Board) {
        self.white_kingside_castle = false;
        self.white_queenside_castle = false;
        self.black_kingside_castle = false;
        self.black_queenside_castle = false;

        self.en_passant = None;

        self.board = board;
    }

    /// Returns the current turn
    ///
    /// # Return
    /// * `Color` - The current turn
    pub fn get_turn(&mut self) -> Color {
        self.turn
    }

    /// Sets the current turn
    ///
    /// **This will reset en passant**
    ///
    /// # Arguments
    /// * `Color` - The current turn
    pub fn set_turn(&mut self, turn: Color) {
        self.en_passant = None;
        self.turn = turn;
    }

    /// Returns if a certain color can capture the other color's king
    fn can_capture_king(&self, color: Color) -> bool {
        self.board
            .tiles
            .iter()
            .enumerate()
            .filter(|(_, p)| p.is_some())
            .map(|(i, p)| {
                let x = i % 8;
                let y = i / 8;

                ((x, y), p.unwrap())
            })
            .filter(|(_, p)| p.color == color)
            .map(|((x, y), _)| {
                self.gen_pseudo_legal_moves(x, y, false)
                    .unwrap_or(Vec::new())
            })
            .flatten()
            .any(|m| match m {
                Move::Capture { capture, .. } | Move::CapturePromotion { capture, .. } => {
                    self.board
                        .get_tile(capture.0, capture.1)
                        .unwrap()
                        .piece_type
                        == PieceType::King
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
        if let Some(piece) = self.board.get_tile(x, y) {
            if piece.color != self.turn {
                return None;
            }
        } else {
            return None;
        }

        let moves = self.gen_pseudo_legal_moves(x, y, false);

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
}
