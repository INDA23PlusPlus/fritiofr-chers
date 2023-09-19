use crate::PieceType;

pub enum CastleSide {
    KingSide,
    QueenSide,
}

pub const CHECK: u8 = 0b00000001;
pub const CHECKMATE: u8 = 0b00000010;
pub const DOUBLE_PAWN_PUSH: u8 = 0b00000100;

/// A move that can be made on a board
pub enum Move {
    /// A move that is not a capture
    Quiet {
        from: (usize, usize),
        to: (usize, usize),
        flags: u8,
    },
    /// A move that is a capture
    Capture {
        from: (usize, usize),
        to: (usize, usize),
        capture: (usize, usize),
        flags: u8,
    },
    /// A move that is a castle
    Castle {
        from: (usize, usize),
        to: (usize, usize),
        rook_from: (usize, usize),
        rook_to: (usize, usize),
        side: CastleSide,
        flags: u8,
    },
    /// A move that is a promotion
    QuietPromotion {
        from: (usize, usize),
        to: (usize, usize),
        promotion: PieceType,
        flags: u8,
    },
    /// A move that is a capture and a promotion
    CapturePromotion {
        from: (usize, usize),
        to: (usize, usize),
        capture: (usize, usize),
        promotion: PieceType,
        flags: u8,
    },
}

impl Move {
    /// Get the from tile of the move
    ///
    /// # Returns
    /// The from tile of the move
    pub fn is_check(&self) -> bool {
        self.flags() & CHECK != 0
    }

    pub fn is_checkmate(&self) -> bool {
        self.flags() & CHECKMATE != 0
    }

    pub fn is_double_pawn_push(&self) -> bool {
        self.flags() & DOUBLE_PAWN_PUSH != 0
    }

    fn flags(&self) -> u8 {
        match self {
            Move::Quiet { flags, .. } => flags,
            Move::Capture { flags, .. } => flags,
            Move::Castle { flags, .. } => flags,
            Move::QuietPromotion { flags, .. } => flags,
            Move::CapturePromotion { flags, .. } => flags,
        }
        .clone()
    }
}
