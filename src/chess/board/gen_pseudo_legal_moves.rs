use crate::{Board, Color, Move, Piece, PieceType};

/// Generates all pseudo legal moves for a piece
///
/// A pseudo legal move is a move that is legal except for the fact that it might leave the king
/// in check.
pub fn gen_pseudo_legal_moves(
    board: &Board,
    x: usize,
    y: usize,
    skip_castle: bool,
) -> Option<Vec<Move>> {
    let piece = board.get_tile(x, y);

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

                    let oc_piece = board.get_tile(c_x as usize, c_y as usize);

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

                    let oc_piece_further = board.get_tile(c_x as usize, c_y);
                    let oc_piece_close = board.get_tile(c_x as usize, (c_y as i32 - dir) as usize);

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

                        let oc_piece = board.get_tile(c_x as usize, c_y as usize);

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
                if let Some((ep_x, ep_y)) = board.en_passant {
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

                    let oc_piece = board.get_tile(c_x, c_y);

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

    if piece.piece_type == PieceType::King && !skip_castle {
        // (non attacked positions, empty positions, king end position, rook start
        // position, rook end position)
        let queen_side_tiles: (Vec<usize>, Vec<usize>, usize, usize, usize) =
            (vec![4, 3, 2], vec![1, 2, 3], 2, 0, 3);
        let king_side_tiles: (Vec<usize>, Vec<usize>, usize, usize, usize) =
            (vec![4, 5, 6], vec![5, 6], 6, 7, 5);

        let rank = if piece.color == Color::White { 7 } else { 0 };
        let (kingside_castle, queenside_castle) = if piece.color == Color::White {
            (board.white_kingside_castle, board.white_queenside_castle)
        } else {
            (board.black_kingside_castle, board.black_queenside_castle)
        };
        let mut check_data = vec![];
        if kingside_castle {
            check_data.push(king_side_tiles);
        }
        if queenside_castle {
            check_data.push(queen_side_tiles);
        }

        for (tiles_not_attacked, tiles_empty, king_start_x, rook_start_x, rook_end_x) in check_data
        {
            if tiles_empty
                .into_iter()
                .all(|x| board.get_tile(x, rank).is_none())
                && tiles_not_attacked
                    .into_iter()
                    .all(|x| !tile_under_attack(&board, x, rank, piece.color.opposite()))
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

/// Checks if a tile is attacked by a certain color
///
/// # Arguments
/// * `x` - The x coordinate of the tile
/// * `y` - The y coordinate of the tile
/// * `color` - The color of the attacking pieces
///
/// # Returns
/// * `bool` - If theres a piece that can immediately capture the tile
fn tile_under_attack(board: &Board, x: usize, y: usize, color: Color) -> bool {
    // The idea here is to create a dummy board and on the tile we want to check add a piece
    // Then we run move generation and check if any of the moves are a capture of the tile
    // If so then the tile is under attack
    let mut dummy_board = board.clone();

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
        .map(|((x, y), _)| gen_pseudo_legal_moves(&dummy_board, x, y, true))
        .flatten()
        .flatten()
        .any(|m| match m {
            Move::Capture { capture, .. } | Move::CapturePromotion { capture, .. } => {
                capture == (x, y)
            }
            _ => false,
        })
}
