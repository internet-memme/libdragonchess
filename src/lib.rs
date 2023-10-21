use crate::chess_board::{ChessBoard, ChessMove, Color};

mod chess_board;
mod parser;

pub fn is_move_legal(board: ChessBoard, m: ChessMove) -> bool {
    false
}

pub fn get_all_moves(board: ChessBoard, color: Color) -> Vec<ChessMove> {
    Vec::new()
}

pub fn get_legal_moves(board: ChessBoard, pos: [u32; 2]) -> Vec<ChessMove> {
    Vec::new()
}

#[cfg(test)]
mod tests {}
