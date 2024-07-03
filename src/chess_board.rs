use std::array;
use std::cmp::min;
use std::ops::Add;
use crate::util::{Vec3d};

const BOARD_WIDTH: i32 = 12; //x
const BOARD_DEPTH: i32 =  8; //y
const BOARD_HEIGHT: i32 =  3; //z

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct ChessPiece {
    pub(crate) piece_type: ChessPieceType,
    pub(crate) color: Color,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum ChessPieceType {
    // Upper
    Sylph,   // (S)
    Griffon, // (G)
    Dragon,  // (R)
    // Middle
    Warrior,  // (W)
    Oliphant, // (O)
    Unicorn,  // (U)
    Hero,     // (H)
    Thief,    // (T)
    Cleric,   // (C)
    Mage,     // (M)
    King,     // (K)
    Paladin,  // (P)
    //Lower
    Dwarf,     // (D)
    Basilisk,  // (B)
    Elemental, // (E)
}

impl ChessPiece {
    pub fn new(piece_type: ChessPieceType, color: Color) -> ChessPiece {
        ChessPiece { piece_type, color }
    }
}


#[derive(PartialEq, Clone, Copy)]
pub struct Position {
    pub x: i32,
    pub y: i32,
    pub z: i32
}

impl Position {
    pub fn new(x:i32, y:i32, z:i32) -> Option<Position>{
        // must be positive but we use i32 to deal with less type conversions
        if x < 0 || y < 0 || z < 0 {
            None
        }else if x > BOARD_WIDTH || y > BOARD_DEPTH || z > BOARD_HEIGHT {
            None
        }else {
            Some(Position{
                x,
                y,
                z
            })
        }
    }
}

impl Add<Vec3d> for Position{
    type Output = Option<Position>;


    fn add(self, rhs: Vec3d) -> Self::Output {
        Position::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z
        )
    }
}

#[derive(PartialEq, Clone, Copy)]
pub struct ChessMove (pub Position, pub Position); //describes a move from a to b

#[derive(Debug)]
pub struct ChessBoard {
    pub board: [[[Option<ChessPiece>; 3]; 8]; 15]
}

impl ChessBoard{
    pub fn new () -> ChessBoard{
        ChessBoard {
            board: { array::from_fn(|_| {
                array::from_fn(|_| {
                    array::from_fn(|_| None
                    )
                })
            })}
        }
    }

    pub fn at (&self, pos: Position) ->  Option<ChessPiece> {
        self.board[pos.x as usize][pos.y as usize][pos.z as usize]
    }

    pub fn set_at(&mut self, pos: Position, piece: Option<ChessPiece>){
        self.board[pos.x as usize][pos.y as usize][pos.z as usize] = piece;
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Color {
    WHITE,
    BLACK,
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use std::array;

    #[test]
    fn test_access() {
        println!("running access test");
        let chessboard = ChessBoard::new();
        let empty = None;
        assert_eq!(chessboard.at(Position::new(0, 0 ,0).unwrap()), empty);
        assert_eq!(chessboard.at(Position::new(BOARD_WIDTH - 1, BOARD_DEPTH -1,BOARD_HEIGHT -1 ).unwrap()), empty);


    }
}
