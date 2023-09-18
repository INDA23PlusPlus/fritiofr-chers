use crate::{Board, Piece};

pub struct Move {
    pub piece: Piece,
    pub is_check: bool,
    pub is_checkmate: bool,
    pub capture: Option<(usize, usize)>,
    pub tiles_traversed: Vec<(usize, usize)>,
    pub board_before_move: Board,
    pub board_after_move: Board,
}

impl Move {
    fn from(&self) -> (usize, usize) {
        if self.tiles_traversed.len() == 0 {
            panic!("No tiles traversed");
        }

        self.tiles_traversed[0]
    }

    fn to(&self) -> (usize, usize) {
        if self.tiles_traversed.len() == 0 {
            panic!("No tiles traversed");
        }

        *self
            .tiles_traversed
            .last()
            .expect("The check above should prevent this")
    }
}
