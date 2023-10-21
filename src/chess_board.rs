pub struct ChessPiece {
    pub(crate) piece_type: ChessPieceType,
    pub(crate) color: Color,
}

pub enum ChessPieceType {
    Empty, // (default)
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

//maybe move me
pub type ChessMove = [u32; 2]; //describes a move from a to b

pub type ChessBoard = [[[ChessPiece; 15]; 8]; 3];

#[derive(PartialEq)]
pub enum Color {
    WHITE,
    BLACK,
}
