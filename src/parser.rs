use crate::chess_board::ChessBoard;
use crate::chess_board::ChessPiece;
use crate::chess_board::ChessPieceType;
use crate::chess_board::Color;

use serde::{Serialize, Serializer};

impl Serialize for ChessPiece {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let chess_char = trans_chess_piece_to_char(self);
        match serializer.serialize_char(chess_char) {
            Ok(res) => Ok(res),
            Err(e) => Err(e),
        }
    }
}

impl Serialize for ChessBoard{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {

    }
}

// impl<'de> Deserialize<'de> for ChessPiece {
//     fn deserialize<D>(deserializer: D) -> Result<Self, Error>
//     where
//         D: Deserializer<'de>,
//     {
//         deserializer.deserialize_struct(/* &'static str */, /* &'static [&'static str] */, /* value */)
//     }
// }

#[derive(Serialize)]
pub struct SerialChessBoard {
    board: ChessBoard,
}

pub fn write_json(board: ChessBoard) {
    //let mut json:
}

fn get_color_from_char(c: char) -> Color {
    if c.is_ascii_lowercase() {
        Color::WHITE
    } else {
        Color::BLACK
    }
}

pub fn trans_char_to_chess_piece(c: char) -> ChessPiece {
    let piece_type = match c.to_ascii_uppercase() {
        'S' => ChessPieceType::Sylph,
        'G' => ChessPieceType::Griffon,
        'R' => ChessPieceType::Dragon,
        'W' => ChessPieceType::Warrior,
        'O' => ChessPieceType::Oliphant,
        'U' => ChessPieceType::Unicorn,
        'H' => ChessPieceType::Hero,
        'T' => ChessPieceType::Thief,
        'C' => ChessPieceType::Cleric,
        'M' => ChessPieceType::Mage,
        'K' => ChessPieceType::King,
        'P' => ChessPieceType::Paladin,
        'D' => ChessPieceType::Dwarf,
        'B' => ChessPieceType::Basilisk,
        'E' => ChessPieceType::Elemental,
        _ => ChessPieceType::Empty,
    };
    let color = get_color_from_char(c);

    let chess_piece: ChessPiece = ChessPiece { piece_type, color };
    chess_piece
}

pub fn trans_chess_piece_to_char(chess_piece: &ChessPiece) -> char {
    let mut chess_char = match chess_piece.piece_type {
        ChessPieceType::Empty => 'x',
        ChessPieceType::Sylph => 'S',
        ChessPieceType::Griffon => 'G',
        ChessPieceType::Dragon => 'R',
        ChessPieceType::Warrior => 'W',
        ChessPieceType::Oliphant => 'O',
        ChessPieceType::Unicorn => 'U',
        ChessPieceType::Hero => 'H',
        ChessPieceType::Thief => 'T',
        ChessPieceType::Cleric => 'C',
        ChessPieceType::Mage => 'M',
        ChessPieceType::King => 'K',
        ChessPieceType::Paladin => 'P',
        ChessPieceType::Dwarf => 'D',
        ChessPieceType::Basilisk => 'B',
        ChessPieceType::Elemental => 'E',
    };
    if chess_piece.color == Color::WHITE {
        chess_char = chess_char.to_ascii_lowercase();
    }
    chess_char
}

#[cfg(test)]
mod tests {
    //use crate::parser::read_json;
    use crate::chess_board::{ChessBoard, ChessPiece, ChessPieceType, Color};
    use std::{array, fs};


    fn setup_test_board() -> ChessBoard {
        let chessboard: ChessBoard = array::from_fn(|_| {
            array::from_fn(|_| {
                array::from_fn(|_| ChessPiece {
                    piece_type: ChessPieceType::Empty,
                    color: Color::WHITE,
                })
            })
        });
        chessboard
    }

    #[test]
    fn read_json_file() {
        let file_path = "./examples/boards/setup.json";
        let content = fs::read_to_string(file_path).expect("Error reading file");
        let board_json: serde_json::Value =
            serde_json::from_str(&content).expect("error reading json");
        println!("{}", board_json);
        let chessboard = setup_test_board();
        let jstring = serde_json::to_string(&chessboard).expect("can't convert to string");
        println!("{}", jstring)

        //read_json(board_json)
    }
}
