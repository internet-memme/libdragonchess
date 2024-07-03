use crate::chess_board::{ChessBoard, ChessMove, Color};

mod util;
mod chess_board;
//mod parser;
mod piece_rules;

pub fn is_move_legal(board: ChessBoard, _m: ChessMove) -> bool {
    false
}

pub fn get_all_moves(board: ChessBoard, _color: Color) -> Vec<ChessMove> {
    Vec::new()
}

pub fn get_legal_moves(board: ChessBoard, _pos: [u32; 2]) -> Vec<ChessMove> {
    Vec::new()
}


#[cfg(test)]
mod tests {}
