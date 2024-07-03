//basically a bishop

use crate::chess_board::{ChessBoard, ChessMove, Position};

fn is_move_legal(board: &ChessBoard, chess_move: ChessMove) ->bool{
    let moves = get_legal_moves(board, chess_move.0);
    moves.contains(&chess_move)
}

fn get_legal_moves(board: &ChessBoard, pos: Position) -> Vec<ChessMove>{
    let mut moves : Vec<ChessMove> = Vec::new();
    //Idea "raycast in all 4 directions
    //end on free space with no piece in between or on enemy piece
    for (x,y) in [(-1,-1), (1,-1), (1,1), (-1,1)]{
        for a in (1..14){
            let newpos = Position::new( a*x, a*y, 1).unwrap();//TODO: error checking
            let cmove = ChessMove(pos, newpos);

        }

    }

    moves
}
