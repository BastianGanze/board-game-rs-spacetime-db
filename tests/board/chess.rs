use chess::ChessMove;

use board_game::board::Board;
use board_game::games::chess::ChessBoard;

use crate::board::board_test_main;

#[test]
fn chess_start() {
    board_test_main(&ChessBoard::default());
}

#[test]
fn chess_en_passant() {
    let moves = vec!["e4", "e6", "e5", "d5"];

    let mut board = ChessBoard::default();
    for &mv in &moves {
        println!("{}", board);
        board.play(ChessMove::from_san(board.inner(), mv).unwrap());
    }

    let capture = ChessMove::from_san(board.inner(), "ed6").unwrap();
    assert!(board.is_available_move(capture));

    board_test_main(&board);
}

//TODO add tests for 50 move and 3-move rule
