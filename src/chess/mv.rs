use crate::PieceType;

/// A move that can be made in a game
#[derive(Debug, Clone, Copy)]
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
