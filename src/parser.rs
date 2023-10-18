use serde_json::{Deserializer, Serializer};
use crate::chess_board::ChessPieceType as ChessPieceType;
use crate::chess_board::ChessBoard as ChessBoard;


pub fn read_json() {

}

pub fn write_json(board: ChessBoard) {

}

pub fn trans_char_to_chess_piece(c: char) -> ChessPieceType {

    match c {
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
        _ => ChessPieceType::Empty
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::env;

    #[test]
    fn testtest(){
        assert!(true)
    }

    #[test]
    fn read_json_file(){
        let cdir = env::current_dir().expect("can't read current dir");
        println!("{}", cdir.display());
        let file_path = "./examples/boards/setup.json";
        let content = fs::read_to_string(file_path).expect("Error reading file");
        let board_json: serde_json::Value = serde_json::from_str(&content).expect("error reading json");
        println!("{}", board_json)

    }
    
}
    
