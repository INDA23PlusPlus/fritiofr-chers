//! # Fritiofs awesome chess library 🤩
//!
//! This is a cool chess library that I made. The internals are pretty messy 🫣, but hopefully the
//! exposed api's are nice atleast!
//!
//! ## How to use the library 📚
//! Since chess doesn't make sense without knowing what has happened before in the game, this
//! library is currated around a `Game` struct. This struct contains all the information about the
//! game state and how the game can continue to progess.
//!
//! This means that it's very difficult to make an arbitrary move on the board. You can't juse move
//! pieces around yourself, you have to use the `apply_move` function to apply a move to the game
//! with a valid move enum.
//!
//! The idea on how to play a game of chess with this library:
//! - Start by checking `is_checkmate` and `is_stalemate` to see if the game has ended
//! - Call either `gen_moves` or `gen_all_moves` to get a vector containing all the moves for the
//! current turn
//! - Pick a move from the vector and apply it to the game with `apply_move`
//! - Repeat 🔁
//!
//! ## Things that are not implemented by design 🚫
//!
//! - There is no real way to switch turns in the game.
//! - There is no way to move pieces arbitrarily around in a game.
//!
//! ## If you have any questions or suggestions 🤔
//!
//! Contact me on discord or mail!
//! - Discord: `Fritiof#3698`
//! - Mail: `fritiof@rusck.se`
//!
//! *Psst i check my mail more often than my discord, so if you want to reach me quickly send a
//! mail*

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
