use chess::Board;

fn main() {
    let board = Board::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - -").unwrap();
    let amount_of_moves = amount_of_moves_recursively(board, 3);
    assert_eq!(amount_of_moves, 2812);

    let board = Board::from_fen("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ -").unwrap();
    let amount_of_moves = amount_of_moves_recursively(board, 3);
    assert_eq!(amount_of_moves, 62379);

    let board =
        Board::from_fen("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq -").unwrap();
    let amount_of_moves = amount_of_moves_recursively(board, 3);
    assert_eq!(amount_of_moves, 9467);

    let board = Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -")
        .unwrap();

    let amount_of_moves = amount_of_moves_recursively(board, 3);
    assert_eq!(amount_of_moves, 97862);
}

fn amount_of_moves_recursively(board: Board, depth: u8) -> u64 {
    if depth == 0 {
        return 1;
    }
    let mut amount = 0;
    for m in board.gen_all_moves().unwrap_or(Vec::new()) {
        let mut board = board.clone();
        board.apply_move(m).unwrap();
        amount += amount_of_moves_recursively(board, depth - 1);
    }
    amount
}
