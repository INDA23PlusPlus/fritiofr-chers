use crate::PieceType;

pub enum CastleSide {
    KingSide,
    QueenSide,
}

/// A move that can be made on a board
pub enum Move {
    /// A move that is not a capture
    Quiet {
        from: (usize, usize),
        to: (usize, usize),
    },
    /// A move that is a capture
    Capture {
        from: (usize, usize),
        to: (usize, usize),
        capture: (usize, usize),
    },
    /// A move that is a castle
    Castle {
        from: (usize, usize),
        to: (usize, usize),
        rook_from: (usize, usize),
        rook_to: (usize, usize),
        side: CastleSide,
    },
    /// A move that is a promotion
    QuietPromotion {
        from: (usize, usize),
        to: (usize, usize),
        promotion: PieceType,
    },
    /// A move that is a capture and a promotion
    CapturePromotion {
        from: (usize, usize),
        to: (usize, usize),
        capture: (usize, usize),
        promotion: PieceType,
    },
}
