use crate::PieceType;

/// A move that can be applied to a game
///
/// There are a number of helper functions to check what type of move it is and get the relevant
/// information quickly.
///
/// A good way to render a move is to check `from()` and `to()` first, if you need to render the
/// capture square you can use `capture()`.
#[derive(Debug, Clone, Copy)]
pub enum Move {
    /// A move that is not a capture
    Quiet {
        from: (usize, usize),
        to: (usize, usize),
    },
    /// A move that is a double pawn push
    DoublePawnPush {
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

impl Move {
    pub fn is_double_pawn_push(&self) -> bool {
        match self {
            Move::DoublePawnPush { .. } => true,
            _ => false,
        }
    }

    pub fn is_capture(&self) -> bool {
        match self {
            Move::Capture { .. } | Move::CapturePromotion { .. } => true,
            _ => false,
        }
    }

    pub fn is_castle(&self) -> bool {
        match self {
            Move::Castle { .. } => true,
            _ => false,
        }
    }

    pub fn is_promotion(&self) -> bool {
        match self {
            Move::QuietPromotion { .. } | Move::CapturePromotion { .. } => true,
            _ => false,
        }
    }

    /// Returns the move from square. If the move is a castle, it returns the king square
    pub fn from(&self) -> (usize, usize) {
        match self {
            Move::Quiet { from, .. }
            | Move::DoublePawnPush { from, .. }
            | Move::Capture { from, .. }
            | Move::Castle { from, .. }
            | Move::QuietPromotion { from, .. }
            | Move::CapturePromotion { from, .. } => *from,
        }
    }

    /// Returns the move to square. If the move is a castle, it returns the king square
    pub fn to(&self) -> (usize, usize) {
        match self {
            Move::Quiet { to, .. }
            | Move::DoublePawnPush { to, .. }
            | Move::Capture { to, .. }
            | Move::Castle { to, .. }
            | Move::QuietPromotion { to, .. }
            | Move::CapturePromotion { to, .. } => *to,
        }
    }

    pub fn capture(&self) -> Option<(usize, usize)> {
        match self {
            Move::Capture { capture, .. } | Move::CapturePromotion { capture, .. } => {
                Some(*capture)
            }
            _ => None,
        }
    }

    pub fn promotion(&self) -> Option<PieceType> {
        match self {
            Move::QuietPromotion { promotion, .. } | Move::CapturePromotion { promotion, .. } => {
                Some(*promotion)
            }
            _ => None,
        }
    }

    /// Returns the rook from square if the move is a castle
    pub fn rook_from(&self) -> Option<(usize, usize)> {
        match self {
            Move::Castle { rook_from, .. } => Some(*rook_from),
            _ => None,
        }
    }

    /// Returns the rook to square if the move is a castle
    pub fn rook_to(&self) -> Option<(usize, usize)> {
        match self {
            Move::Castle { rook_to, .. } => Some(*rook_to),
            _ => None,
        }
    }
}
