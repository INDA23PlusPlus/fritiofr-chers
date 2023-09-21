/// This file is very messy -.- i know...
/// Hopefully it's abstracted away enough that no one will need to read this
use crate::{Color, Game, Move, Piece, PieceType};

const ROOK_DIRS: &[(i32, i32)] = &[(1, 0), (-1, 0), (0, 1), (0, -1)];
const BISHOP_DIRS: &[(i32, i32)] = &[(1, 1), (-1, 1), (1, -1), (-1, -1)];
const KING_QUEEN_DIRS: &[(i32, i32)] = &[
    (1, 0),
    (-1, 0),
    (0, 1),
    (0, -1),
    (1, 1),
    (-1, 1),
    (1, -1),
    (-1, -1),
];
const KNIGHT_DIRS: &[(i32, i32)] = &[
    (2, 1),
    (2, -1),
    (-2, 1),
    (-2, -1),
    (1, 2),
    (1, -2),
    (-1, 2),
    (-1, -2),
];

/// Internal struct used for storing search data about a move
struct OutSearch<'a> {
    /// How far out the search can continue
    depth: i32,
    /// Direction of the search
    dirs: &'a [(i32, i32)],
    /// If quiet moves are allowed
    quiet: bool,
    /// If captures are allowed
    captures: bool,
}
const ROOK_OUT_SEARCH: OutSearch = OutSearch {
    depth: 8,
    dirs: ROOK_DIRS,
    quiet: true,
    captures: true,
};
const BISHOP_OUT_SEARCH: OutSearch = OutSearch {
    depth: 8,
    dirs: BISHOP_DIRS,
    quiet: true,
    captures: true,
};
const QUEEN_OUT_SEARCH: OutSearch = OutSearch {
    depth: 8,
    dirs: KING_QUEEN_DIRS,
    quiet: true,
    captures: true,
};
const KNIGHT_OUT_SEARCH: OutSearch = OutSearch {
    depth: 1,
    dirs: KNIGHT_DIRS,
    quiet: true,
    captures: true,
};
const KING_OUT_SEARCH: OutSearch = OutSearch {
    depth: 1,
    dirs: KING_QUEEN_DIRS,
    quiet: true,
    captures: true,
};

impl Game {
    /// Generates all pseudo legal moves for a piece
    ///
    /// A pseudo legal move is a move that is legal except for the fact that it might leave the king
    /// in check.
    pub(crate) fn gen_pseudo_legal_moves(
        &self,
        x: usize,
        y: usize,
        skip_castle: bool,
    ) -> Option<Vec<Move>> {
        let piece = self.board.get_tile(x, y);

        if piece.is_none() {
            return None;
        }

        let piece = piece.expect("Already checked that this is not none");

        let mut moves: Vec<Move> = vec![];

        if piece.piece_type == PieceType::Pawn {
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

                    let oc_piece = self.board.get_tile(c_x as usize, c_y as usize);

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

                    let oc_piece_further = self.board.get_tile(c_x as usize, c_y);
                    let oc_piece_close = self
                        .board
                        .get_tile(c_x as usize, (c_y as i32 - dir) as usize);

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

                        let oc_piece = self.board.get_tile(c_x as usize, c_y as usize);

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
        } else {
            let searches = match piece.piece_type {
                PieceType::Rook => vec![ROOK_OUT_SEARCH],
                PieceType::Bishop => vec![BISHOP_OUT_SEARCH],
                PieceType::Queen => vec![QUEEN_OUT_SEARCH],
                PieceType::Knight => vec![KNIGHT_OUT_SEARCH],
                PieceType::King => vec![KING_OUT_SEARCH],
                PieceType::Pawn => unreachable!(),
            };

            // Regular search out
            for search in searches {
                for dir in search.dirs {
                    for i in 1..=search.depth {
                        let c_x = dir.0 * i + x as i32;
                        let c_y = dir.1 * i + y as i32;

                        if c_x < 0 || c_x > 7 || c_y < 0 || c_y > 7 {
                            break;
                        }

                        let c_x = c_x as usize;
                        let c_y = c_y as usize;

                        let oc_piece = self.board.get_tile(c_x, c_y);

                        match oc_piece {
                            Some(oc_piece) => {
                                if oc_piece.color != piece.color && search.captures {
                                    moves.push(Move::Capture {
                                        from: (x, y),
                                        to: (c_x, c_y),
                                        capture: (c_x, c_y),
                                    });
                                }

                                break;
                            }
                            None => {
                                if search.quiet {
                                    moves.push(Move::Quiet {
                                        from: (x, y),
                                        to: (c_x, c_y),
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }

        // Castling
        if piece.piece_type == PieceType::King && !skip_castle {
            // (non attacked positions, empty positions, king end position, rook start
            // position, rook end position)
            let queen_side_tiles: (Vec<usize>, Vec<usize>, usize, usize, usize) =
                (vec![4, 3, 2], vec![1, 2, 3], 2, 0, 3);
            let king_side_tiles: (Vec<usize>, Vec<usize>, usize, usize, usize) =
                (vec![4, 5, 6], vec![5, 6], 6, 7, 5);

            let rank = if piece.color == Color::White { 7 } else { 0 };
            let (kingside_castle, queenside_castle) = if piece.color == Color::White {
                (self.white_kingside_castle, self.white_queenside_castle)
            } else {
                (self.black_kingside_castle, self.black_queenside_castle)
            };
            let mut check_data = vec![];
            if kingside_castle {
                check_data.push(king_side_tiles);
            }
            if queenside_castle {
                check_data.push(queen_side_tiles);
            }

            for (tiles_not_attacked, tiles_empty, king_start_x, rook_start_x, rook_end_x) in
                check_data
            {
                if tiles_empty
                    .into_iter()
                    .all(|x| self.board.get_tile(x, rank).is_none())
                    && tiles_not_attacked
                        .into_iter()
                        .all(|x| !tile_under_attack(&self, x, rank, piece.color.opposite()))
                {
                    moves.push(Move::Castle {
                        from: (4, rank),
                        to: (king_start_x, rank),
                        rook_from: (rook_start_x, rank),
                        rook_to: (rook_end_x, rank),
                    });
                }
            }
        }

        Some(moves)
    }
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
fn tile_under_attack(board: &Game, x: usize, y: usize, color: Color) -> bool {
    // The idea here is to create a dummy board and on the tile we want to check add a piece
    // Then we run move generation and check if any of the moves are a capture of the tile
    // If so then the tile is under attack
    let mut dummy_board = board.clone();

    if let Some(piece) = dummy_board.board.get_tile(x, y) {
        if piece.color == color {
            return false;
        }
    } else {
        dummy_board.board.set_tile(
            x,
            y,
            Piece {
                piece_type: PieceType::Pawn,
                color: color.opposite(),
            },
        );
    }

    dummy_board
        .board
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
        .map(|((x, y), _)| dummy_board.gen_pseudo_legal_moves(x, y, true))
        .flatten()
        .flatten()
        .any(|m| match m {
            Move::Capture { capture, .. } | Move::CapturePromotion { capture, .. } => {
                capture == (x, y)
            }
            _ => false,
        })
}
