#![allow(unstable)]

extern crate core;
use core::nonzero::NonZero;
use std::mem::transmute;

#[derive(Copy, Clone, Show, PartialEq, Eq)]
pub struct Piece(NonZero<u8>);

#[derive(Copy, Clone, Show, PartialEq, Eq, Hash)]
struct DoublePiece(u8);

#[repr(u8)]
#[derive(Copy, Clone, Show, PartialEq, Eq, Hash)]
pub enum Player {
    White = 0,
    Black = 1
}

#[repr(u8)]
#[derive(Copy, Clone, Show, PartialEq, Eq, Hash)]
pub enum Chessman {
    Pawn   = 1,
    Bishop = 2,
    Knight = 3,
    Rook   = 4,
    King   = 5,
    Queen  = 6
}

#[derive(Copy, Clone, Show, PartialEq, Eq)]
pub struct Board {
    b: [[DoublePiece; 4]; 8]
}

#[derive(Copy, Clone, Show, PartialEq, Eq, Hash)]
pub struct Pos(u8);

impl DoublePiece {
    fn new(top: Option<Piece>, bottom: Option<Piece>) -> DoublePiece {
        unsafe {
            let v1: u8 = transmute(top);
            let v2: u8 = transmute(bottom);
            transmute((v1 << 4) | v2)
        }
    }

    fn empty() -> DoublePiece {
        unsafe {
            transmute(0u8)
        }
    }

    fn access(self, shift: u8) -> Option<Piece> {
        unsafe {
            let v: u8 = transmute(self);
            transmute((v >> (4 * shift)) & 0xf)
        }
    }

    #[allow(dead_code)]
    fn top(self) -> Option<Piece> {
        unsafe {
            let v: u8 = transmute(self);
            transmute(v >> 4)
        }
    }

    #[allow(dead_code)]
    fn bottom(self) -> Option<Piece> {
        unsafe {
            let v: u8 = transmute(self);
            transmute(v & 0xf)
        }
    }
}

impl Piece {
    pub fn new(player: Player, chessman: Chessman) -> Piece {
        unsafe {
            let pv: u8 = transmute(player);
            let cv: u8 = transmute(chessman);
            Piece(NonZero::new((pv << 3) | cv))
        }
    }

    pub fn player(self) -> Player {
        unsafe {
            let v: u8 = transmute(self);
            transmute(v >> 3)
        }
    }

    pub fn chessman(self) -> Chessman {
        unsafe {
            let v: u8 = transmute(self);
            transmute(v & 0b111)
        }
    }

    pub fn info(self) -> (Player, Chessman) {
        unsafe {
            let v: u8 = transmute(self);
            transmute((v >> 3, v & 0b111))
        }
    }
}

impl Board {
    pub fn new() -> Board {
        Board{ b:
        [
            [
             DoublePiece::new(Some(Piece::new(Player::Black, Chessman::Rook)),
                              Some(Piece::new(Player::Black, Chessman::Pawn))),
             DoublePiece::empty(),
             DoublePiece::empty(),
             DoublePiece::new(Some(Piece::new(Player::White, Chessman::Pawn)),
                              Some(Piece::new(Player::White, Chessman::Rook)))
            ],
            [
             DoublePiece::new(Some(Piece::new(Player::Black, Chessman::Knight)),
                              Some(Piece::new(Player::Black, Chessman::Pawn))),
             DoublePiece::empty(),
             DoublePiece::empty(),
             DoublePiece::new(Some(Piece::new(Player::White, Chessman::Pawn)),
                              Some(Piece::new(Player::White, Chessman::Knight)))
            ],
            [
             DoublePiece::new(Some(Piece::new(Player::Black, Chessman::Bishop)),
                              Some(Piece::new(Player::Black, Chessman::Pawn))),
             DoublePiece::empty(),
             DoublePiece::empty(),
             DoublePiece::new(Some(Piece::new(Player::White, Chessman::Pawn)),
                              Some(Piece::new(Player::White, Chessman::Bishop)))
            ],
            [
             DoublePiece::new(Some(Piece::new(Player::Black, Chessman::Queen)),
                              Some(Piece::new(Player::Black, Chessman::Pawn))),
             DoublePiece::empty(),
             DoublePiece::empty(),
             DoublePiece::new(Some(Piece::new(Player::White, Chessman::Pawn)),
                              Some(Piece::new(Player::White, Chessman::Queen)))
            ],
            [
             DoublePiece::new(Some(Piece::new(Player::Black, Chessman::King)),
                              Some(Piece::new(Player::Black, Chessman::Pawn))),
             DoublePiece::empty(),
             DoublePiece::empty(),
             DoublePiece::new(Some(Piece::new(Player::White, Chessman::Pawn)),
                              Some(Piece::new(Player::White, Chessman::King)))
            ],
            [
             DoublePiece::new(Some(Piece::new(Player::Black, Chessman::Bishop)),
                              Some(Piece::new(Player::Black, Chessman::Pawn))),
             DoublePiece::empty(),
             DoublePiece::empty(),
             DoublePiece::new(Some(Piece::new(Player::White, Chessman::Pawn)),
                              Some(Piece::new(Player::White, Chessman::Bishop)))
            ],
            [
             DoublePiece::new(Some(Piece::new(Player::Black, Chessman::Knight)),
                              Some(Piece::new(Player::Black, Chessman::Pawn))),
             DoublePiece::empty(),
             DoublePiece::empty(),
             DoublePiece::new(Some(Piece::new(Player::White, Chessman::Pawn)),
                              Some(Piece::new(Player::White, Chessman::Knight)))
            ],
            [
             DoublePiece::new(Some(Piece::new(Player::Black, Chessman::Rook)),
                              Some(Piece::new(Player::Black, Chessman::Pawn))),
             DoublePiece::empty(),
             DoublePiece::empty(),
             DoublePiece::new(Some(Piece::new(Player::White, Chessman::Pawn)),
                              Some(Piece::new(Player::White, Chessman::Rook)))
            ],
        ]}
    }

    pub fn new_empty() -> Board {
        Board { b: [[DoublePiece::empty() ;4]; 8] }
    }

    pub fn at(&self, index: Pos) -> Option<Piece> {
        let Pos(p) = index;
        let x = p >> 4;
        let y = p & 0xf;
        let shift = (!y) & 0b1;
        let ac_y = y >> 1;

        unsafe {
            self.b.get_unchecked(x as usize)
                  .get_unchecked(ac_y as usize)
                  .access(shift)
        }
    }
}

impl Pos {
    /// These coordinates start from the top-right hand corner of
    /// the board and increase going down to the left.
    pub fn from_coords(x: u8, y: u8) -> Option<Pos> {
        if x < 8 || y < 8 {
            Some(Pos(x << 4 | y))
        } else {
            None
        }
    }

    /// Using this function gives you coordinates that aren't checked to
    /// be correct bounds!  Although this isn't memory unsafe, it is something
    /// that should be avoided at all costs.
    pub unsafe fn from_raw_coords(x: u8, y: u8) -> Pos {
        debug_assert!(x < 8, "x is greater than 8: {}", x);
        debug_assert!(y < 8, "x is greater than 8: {}", y);

        let x = x & 0b111;
        let y = y & 0b111;

        Pos(x << 4 | y)
    }

    pub fn from_an(x: char, y: u8) -> Option<Pos> {
        if y <= 8 && y != 0 {
            return None;
        }

        let y = y - 1;

        let x = match x {
            'a' | 'A' => 0,
            'b' | 'B' => 1,
            'c' | 'C' => 2,
            'd' | 'D' => 3,
            'e' | 'E' => 4,
            'f' | 'F' => 5,
            'g' | 'G' => 6,
            'h' | 'H' => 7,
            _ => return None
        };

        Pos::from_coords(x, y)
    }

    pub fn from_an_string(s: &str) -> Option<Pos> {
        let mut chrs = s.chars();
        let x = match chrs.next() {
            Some(c) => c,
            None => return None
        };

        let y = match chrs.next() {
            Some(c) => {
                match c.to_digit(10) {
                    Some(d) => {
                        d as u8
                    }
                    None => return None
                }
            }
            None => return None
        };

        if chrs.next() != None {
            None
        } else {
            Pos::from_an(x, y)
        }
    }
}

#[test] fn test_player_repr() {
    use core::mem::size_of;
    assert!(size_of::<Player>() == 1);
    let p = Player::White;
    let px: u8 = unsafe { transmute(p) };
    assert!(px == 0u8);

    let p = Player::Black;
    let px: u8 = unsafe { transmute(p) };
    assert!(px == 1u8);
}

#[test] fn test_chessman_repr() {
    use core::mem::size_of;
    assert!(size_of::<Chessman>() == 1);

    let c = Chessman::Pawn;
    let cx: u8 = unsafe { transmute(c) };
    assert!(cx == 1u8);
}

#[test] fn test_piece_repr() {
    use core::mem::size_of;
    assert!(size_of::<Piece>() == 1);
    assert!(size_of::<Option<Piece>>() == 1);

    assert!(Piece::new(Player::White, Chessman::Pawn).info() == (Player::White, Chessman::Pawn));
    assert!(Piece::new(Player::Black, Chessman::King).info() == (Player::Black, Chessman::King));
    assert!(Piece::new(Player::Black, Chessman::Queen).info() == (Player::Black, Chessman::Queen));
}

#[test] fn test_double_piece_repr() {
    let v1 = DoublePiece(0);
    assert!(v1.top().is_none());
    assert!(v1.bottom().is_none());

    let top = Piece::new(Player::White, Chessman::Pawn);
    let bottom = Piece::new(Player::Black, Chessman::King);

    let v2 = DoublePiece::new(Some(top), Some(bottom));

    assert!(v2.bottom() == Some(bottom));
    assert!(v2.top() == Some(top));

    assert!(v2.access(1) == Some(top));
    assert!(v2.access(0) == Some(bottom));
}

#[test] fn test_board_repr() {
    let board = Board::new();
    let p1 = Pos(0);
    let p2 = Pos(1);

    assert_eq!(board.at(p1), Some(Piece::new(Player::Black, Chessman::Rook)));
    assert_eq!(board.at(p2), Some(Piece::new(Player::Black, Chessman::Pawn)));
}
