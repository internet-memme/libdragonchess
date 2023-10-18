
//defines the pieces of the game

pub enum ChessPiece {
    Empty,      // (default)
      // Upper
    Sylph,      // (S)
    Griffon,    // (G)
    Dragon,     // (R)
      // Middle   
    Warrior,    // (W)
    Oliphant,   // (O)
    Unicorn,    // (U)
    Hero,       // (H)
    Thief,      // (T)
    Cleric,     // (C)
    Mage,       // (M)
    King,       // (K)
    Paladin,    // (P)
      //Lower   
    Dwarf,      // (D)
    Basilisk,   // (B)
    Elemental   // (E)
}

pub type ChessMove = [u32; 2]; //describes a move from a to b


pub type ChessBoard = [[[ChessPiece; 15]; 8]; 3];

pub enum Color {
    WHITE,
    BLACK
}

pub fn trans_char_to_ChessPiece (c: char) -> ChessPiece {
    match c {
	'S' => ChessPiece::Sylph,
	'G' => ChessPiece::Griffon,
	'R' => ChessPiece::Dragon,
	'W' => ChessPiece::Warrior,
	'O' => ChessPiece::Oliphant,
	'U' => ChessPiece::Unicorn,
	'H' => ChessPiece::Hero,
	'T' => ChessPiece::Thief,
	'C' => ChessPiece::Cleric,
	'M' => ChessPiece::Mage,
	'K' => ChessPiece::King,
	'P' => ChessPiece::Paladin,
	'D' => ChessPiece::Dwarf,
	'B' => ChessPiece::Basilisk,
	'E' => ChessPiece::Elemental,
	_ => ChessPiece::Empty
    }

}

pub fn is_move_legal (board: ChessBoard, m: ChessMove) -> bool {
    false
}

pub fn get_all_moves (board: ChessBoard, color: Color) -> Vec<ChessMove> {
    Vec::new()
}

pub fn get_legal_moves (board: ChessBoard, pos: [u32; 2]) -> Vec<ChessMove> {
    Vec::new()
}

#[cfg(test)]
mod tests {

    

}
