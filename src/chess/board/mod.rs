use std::error::Error;

use crate::{chess::board::mv::CastleSide, Color, Piece, PieceType};

mod fen_parser;

pub use fen_parser::BoardFromFenError;

mod mv;
pub use mv::Move;

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

    /// Returns a board with the starting position
    pub fn start_pos() -> Board {
        Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
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

                    if piece.piece_type == PieceType::Pawn {
                        self.halfmove = 0;
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

                    self.halfmove = 0;

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
                    self.halfmove = 0;

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

                    self.halfmove = 0;

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
            .map(|((x, y), _)| self.gen_pseudo_legal_moves(x, y))
            .flatten()
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

                !board.can_capture_king(board.turn)
            })
            .collect::<Vec<Move>>();

        if moves.len() == 0 {
            return None;
        }

        Some(moves)
    }

    /// Checks if a tile is attacked by a certain color
    ///
    /// # Arguments
    /// * `x` - The x coordinate of the tile
    /// * `y` - The y coordinate of the tile
    /// * `color` - The color of the attacking pieces
    ///
    /// # Returns
    /// * `bool` - If theres a piece that can immediately capture the tile
    fn tile_under_attack(&self, x: usize, y: usize, color: Color) -> bool {
        // The idea here is to create a dummy board and on the tile we want to check add a piece
        // Then we run move generation and check if any of the moves are a capture of the tile
        // If so then the tile is under attack
        let mut dummy_board = self.clone();

        if let Some(piece) = dummy_board.get_tile(x, y) {
            if piece.color == color {
                return false;
            }
        } else {
            dummy_board.set_tile(
                x,
                y,
                Piece {
                    piece_type: PieceType::Pawn,
                    color: color.opposite(),
                },
            );
        }

        dummy_board
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
            .map(|((x, y), _)| dummy_board.gen_pseudo_legal_moves(x, y))
            .flatten()
            .flatten()
            .any(|m| match m {
                Move::Capture { capture, .. } | Move::CapturePromotion { capture, .. } => {
                    capture == (x, y)
                }
                _ => false,
            })
    }

    fn gen_pseudo_legal_moves(&self, x: usize, y: usize) -> Option<Vec<Move>> {
        let piece = self.get_tile(x, y);

        if piece.is_none() {
            return None;
        }

        let piece = piece.expect("Already checked that this is not none");

        let mut moves = match piece.piece_type {
            PieceType::Pawn => {
                let mut moves = Vec::new();

                let final_rank = if piece.color == Color::White { 0 } else { 7 };
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

                    if c_y >= 0 && c_y <= 7 {
                        let c_x = c_x as usize;
                        let c_y = c_y as usize;

                        let oc_piece = self.get_tile(c_x as usize, c_y as usize);

                        if oc_piece.is_none() {
                            if c_y == final_rank {
                                moves.append(
                                    &mut promotion_pieces
                                        .iter()
                                        .map(|&p| Move::QuietPromotion {
                                            from: (x, y),
                                            to: (c_x, c_y),
                                            promotion: p,
                                        })
                                        .collect(),
                                );
                            } else {
                                moves.push(Move::Quiet {
                                    from: (x, y),
                                    to: (c_x, c_y),
                                });
                            }
                        }
                    }
                }

                // Double forwards
                {
                    let c_x = x as i32;
                    let c_y = y as i32 + dir * 2;

                    if (c_y >= 0 && c_y <= 7) && y == starting_rank {
                        let c_x = c_x as usize;
                        let c_y = c_y as usize;

                        let oc_piece_further = self.get_tile(c_x as usize, c_y);
                        let oc_piece_close =
                            self.get_tile(c_x as usize, (c_y as i32 - dir) as usize);

                        if oc_piece_further.is_none() && oc_piece_close.is_none() {
                            moves.push(Move::Quiet {
                                from: (x, y),
                                to: (c_x, c_y),
                            });
                        }
                    }
                }

                // Capture
                {
                    for x_dir in [-1, 1] {
                        let c_x = x as i32 + x_dir;
                        let c_y = y as i32 + dir;

                        if c_y >= 0 && c_y <= 7 && c_x >= 0 && c_x <= 7 {
                            let c_x = c_x as usize;
                            let c_y = c_y as usize;

                            let oc_piece = self.get_tile(c_x as usize, c_y as usize);

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
                                            })
                                            .collect(),
                                    );
                                } else {
                                    moves.push(Move::Capture {
                                        from: (x, y),
                                        to: (c_x, c_y),
                                        capture: (c_x, c_y),
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

                            if c_y >= 0 && c_y <= 7 && c_x >= 0 && c_x <= 7 {
                                let c_x = c_x as usize;
                                let c_y = c_y as usize;

                                if (c_x, y) == (ep_x, ep_y) {
                                    moves.push(Move::Capture {
                                        from: (x, y),
                                        to: (c_x, c_y),
                                        capture: (ep_x, ep_y),
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
                    for i in 1..=depth {
                        let c_x = dir.0 * i + x as i32;
                        let c_y = dir.1 * i + y as i32;

                        if c_x < 0 || c_x > 7 || c_y < 0 || c_y > 7 {
                            break;
                        }

                        let c_x = c_x as usize;
                        let c_y = c_y as usize;

                        let oc_piece = self.get_tile(c_x, c_y);

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
        };

        if piece.piece_type == PieceType::King {
            if piece.color == Color::White {
                const WHITE_QUEEN_SIDE_TILES: [(usize, usize); 3] = [(4, 7), (3, 7), (2, 7)];
                const WHITE_KING_SIDE_TILES: [(usize, usize); 3] = [(4, 7), (5, 7), (6, 7)];
                if self.white_queenside_castle
                    && WHITE_QUEEN_SIDE_TILES
                        .into_iter()
                        .skip(1)
                        .all(|(x, y)| self.get_tile(x, y).is_none())
                    && WHITE_QUEEN_SIDE_TILES
                        .into_iter()
                        .all(|(x, y)| !self.tile_under_attack(x, y, Color::Black))
                {
                    moves.push(Move::Castle {
                        from: (4, 7),
                        to: (2, 7),
                        rook_from: (0, 7),
                        rook_to: (3, 7),
                    });
                }

                if self.white_kingside_castle
                    && WHITE_KING_SIDE_TILES
                        .into_iter()
                        .skip(1)
                        .all(|(x, y)| self.get_tile(x, y).is_none())
                    && WHITE_KING_SIDE_TILES
                        .into_iter()
                        .all(|(x, y)| !self.tile_under_attack(x, y, Color::Black))
                {
                    moves.push(Move::Castle {
                        from: (4, 7),
                        to: (6, 7),
                        rook_from: (7, 7),
                        rook_to: (5, 7),
                    });
                }
            }

            if piece.color == Color::Black {
                const BLACK_QUEEN_SIDE_TILES: [(usize, usize); 3] = [(4, 0), (3, 0), (2, 0)];
                const BLACK_KING_SIDE_TILES: [(usize, usize); 3] = [(4, 0), (5, 0), (6, 0)];
                if self.black_queenside_castle
                    && BLACK_QUEEN_SIDE_TILES
                        .into_iter()
                        .skip(1)
                        .all(|(x, y)| self.get_tile(x, y).is_none())
                    && BLACK_QUEEN_SIDE_TILES
                        .into_iter()
                        .all(|(x, y)| !self.tile_under_attack(x, y, Color::Black))
                {
                    moves.push(Move::Castle {
                        from: (4, 0),
                        to: (2, 0),
                        rook_from: (0, 0),
                        rook_to: (3, 0),
                    });
                }

                if self.black_kingside_castle
                    && BLACK_KING_SIDE_TILES
                        .into_iter()
                        .skip(1)
                        .all(|(x, y)| self.get_tile(x, y).is_none())
                    && BLACK_KING_SIDE_TILES
                        .into_iter()
                        .all(|(x, y)| !self.tile_under_attack(x, y, Color::Black))
                {
                    moves.push(Move::Castle {
                        from: (4, 0),
                        to: (6, 0),
                        rook_from: (7, 0),
                        rook_to: (5, 0),
                    });
                }
            }
        }

        Some(moves)
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
