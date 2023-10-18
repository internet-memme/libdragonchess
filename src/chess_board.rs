pub struct ChessPiece {
    piece_type: ChessPieceType,
    color: Color
}

pub enum ChessPieceType {
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

//maybe move me
pub type ChessMove = [u32; 2]; //describes a move from a to b


pub type ChessBoard = [[[ChessPiece; 15]; 8]; 3];

pub enum Color {
    WHITE,
    BLACK
}