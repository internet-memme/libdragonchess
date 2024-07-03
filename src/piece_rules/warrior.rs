use crate::chess_board::{ChessBoard, ChessMove, ChessPiece, ChessPieceType, Color, Position};
use crate::util::{Vec3d};

fn is_move_legal(board: &ChessBoard, chess_move: ChessMove) -> bool{
    let moves = get_legal_moves(board, chess_move.0);
    moves.contains(&chess_move)
}

fn get_legal_moves(board: &ChessBoard, pos: Position) -> Vec<ChessMove>{
    let mut moves : Vec<ChessMove> = Vec::new();

    if let Some(curr_piece) = board.at(pos){
        // test move forward
        if let Some(forward_pos) = pos + Vec3d::new(0, 1, 0){
            if board.at(forward_pos) == None {
                moves.push(ChessMove(pos, forward_pos))
            }
        }
        // test diag capture
        for x in [-1,1]{
            if let Some(diag_pos) = pos + Vec3d::new(x, 1, 0){
                if let Some(diag_piece) = board.at(diag_pos){
                    if diag_piece.color != curr_piece.color{
                        moves.push(ChessMove(pos, diag_pos))
                    }
                }
            }
        }
    }
    moves
}

#[cfg(test)]
mod tests {
    use std::option::Option;
    use crate::chess_board::ChessPieceType::{King, Warrior};
    use crate::chess_board::Color::{BLACK, WHITE};
    use super::*;

    const test_piece_pos: Position = Position{ x: 6, y: 4, z: 1 };

    pub fn setup_test_board_valid_moves() -> ChessBoard {
        let mut board = ChessBoard::new();
        board.set_at(
            test_piece_pos,
            Some(ChessPiece::new(Warrior, BLACK)));
        // board.set_at(
        //     (test_piece_pos+Vec3d::new(0,1,0)).unwrap(),
        //     None
        // );
        for x in [-1,1]{
            board.set_at(
                (test_piece_pos + Vec3d::new(x, 1,0)).unwrap(),
                Some(ChessPiece::new(Warrior,WHITE)));
        }

        board
    }
    pub fn setup_test_board_invalid_moves() -> ChessBoard {
        let mut board = ChessBoard::new();
        board.set_at(
            test_piece_pos,
            Some(ChessPiece::new(Warrior, BLACK)));
        board.set_at(
            (test_piece_pos+Vec3d::new(0,1,0)).unwrap(),
            Some(ChessPiece::new(Warrior, BLACK))
        );
        for x in [-1,1]{
            board.set_at(
                (test_piece_pos + Vec3d::new(x, 1,0)).unwrap(),
                Some(ChessPiece::new(Warrior,BLACK)));
        }

        //some positions at the end of the map
        board.set_at(Position::new(0, 7, 1).unwrap(), Some(ChessPiece::new(Warrior, WHITE)));
        board.set_at(Position::new(11,0,1).unwrap(), Some(ChessPiece::new(Warrior, WHITE)));

        board
    }

    #[test]
    fn test_get_legal_moves_valid() {
        let board = setup_test_board_valid_moves();
        let mut moves:[ChessMove;3] =
            [ChessMove(
                Position::new(0,0,0).unwrap(),
                Position::new(0,0,0).unwrap()); 3];
        for (x_n, x) in (-1 ..2).enumerate(){
            moves[x_n] = (ChessMove(test_piece_pos,
                      (test_piece_pos+Vec3d::new(x,1,0)).unwrap()))
        }
        for m in moves {
            assert_eq!(is_move_legal(&board, m), true)
        }
    }

    #[test]
    fn test_get_legal_moves_invalid() {
        let board = setup_test_board_invalid_moves();
        let mut moves:[ChessMove;3] =
            [ChessMove(
                Position::new(0,0,0).unwrap(),
                Position::new(0,0,0).unwrap()); 3];
        for (x_n, x) in (-1 ..2).enumerate() {
            moves[x_n] = (ChessMove(test_piece_pos,
                                    (test_piece_pos+Vec3d::new(0,1,0)).unwrap()))
        }

    }
}
