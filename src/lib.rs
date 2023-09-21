mod chess;
pub use crate::chess::*;

/// Perft tester
#[cfg(test)]
mod tests {
    use super::*;

    /// Function that searches game recursively for moves
    /// Used for perft testing
    fn amount_of_moves_recursively(game: Game, depth: u8) -> u64 {
        if depth == 0 {
            return 1;
        }
        let mut amount = 0;
        for m in game.gen_all_moves().unwrap_or(Vec::new()) {
            let mut game = game.clone();
            game.apply_move(m).unwrap();
            amount += amount_of_moves_recursively(game, depth - 1);
        }
        amount
    }

    #[test]
    #[ignore]
    fn perft_1() {
        let game = Game::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - -").unwrap();
        let amount_of_moves = amount_of_moves_recursively(game, 3);
        assert_eq!(amount_of_moves, 2812);
    }

    #[test]
    #[ignore]
    fn perft_2() {
        let game =
            Game::from_fen("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq -").unwrap();
        let amount_of_moves = amount_of_moves_recursively(game, 3);
        assert_eq!(amount_of_moves, 9467);
    }

    #[test]
    #[ignore]
    fn perft_3() {
        let game =
            Game::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -")
                .unwrap();
        let amount_of_moves = amount_of_moves_recursively(game, 3);
        assert_eq!(amount_of_moves, 97862);
    }

    #[test]
    #[ignore]
    fn perft_4() {
        let game = Game::from_fen("r3k2r/8/3Q4/8/8/5q2/8/R3K2R b KQkq -").unwrap();
        let amount_of_moves = amount_of_moves_recursively(game, 4);
        assert_eq!(amount_of_moves, 1720476);
    }

    #[test]
    #[ignore]
    fn perft_5() {
        let game = Game::from_fen("8/8/1P2K3/8/2n5/1q6/8/5k2 b - -").unwrap();
        let amount_of_moves = amount_of_moves_recursively(game, 5);
        assert_eq!(amount_of_moves, 1004658);
    }

    #[test]
    #[ignore]
    fn perft_6() {
        let game = Game::from_fen("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ -").unwrap();
        let amount_of_moves = amount_of_moves_recursively(game, 3);
        assert_eq!(amount_of_moves, 62379);
    }
}
