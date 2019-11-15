use crate::color::Color;
use crate::kind::Kind;
use crate::params::Params;
use crate::position::Position;
use std::ops::BitXor;

#[derive(Copy, Clone, PartialEq)]
pub struct Piece {
    pub color: Color,
    pub kind: Kind,
}

impl Piece {
    pub fn to_letter<'a>(&self) -> &'a str {
        match self.kind {
            Kind::Pawn => "P",
            Kind::Rook => "R",
            Kind::Knight => "N",
            Kind::Bishop => "B",
            Kind::King => "K",
            Kind::Queen => "Q",
        }
    }

    pub fn hash(&self) -> u64 {
        self.color.hash().bitxor(self.kind.hash())
    }

    pub fn value(&self, position: &Position, params: &Params) -> i16 {
        match self.kind {
            Kind::Pawn => match self.color {
                Color::White => match position.1 {
                    1 => 100,
                    2 => 100,
                    3 => 100 + params.max_pawn_bonus / 8,
                    4 => 100 + params.max_pawn_bonus / 4,
                    5 => 100 + params.max_pawn_bonus / 2,
                    6 => 100 + params.max_pawn_bonus,
                    _ => 0,
                },
                Color::Black => match position.1 {
                    1 => 100 + params.max_pawn_bonus,
                    2 => 100 + params.max_pawn_bonus / 2,
                    3 => 100 + params.max_pawn_bonus / 4,
                    4 => 100 + params.max_pawn_bonus / 8,
                    5 => 100,
                    6 => 100,
                    _ => 0,
                },
            },
            Kind::Knight => {
                let mut val = params.knight_value;
                match position.1 {
                    1 => val += params.max_knight_bonus / 10,
                    2 => val += params.max_knight_bonus / 4,
                    3 => val += params.max_knight_bonus / 2,
                    4 => val += params.max_knight_bonus / 2,
                    5 => val += params.max_knight_bonus / 4,
                    6 => val += params.max_knight_bonus / 10,
                    _ => (),
                };
                match position.0 {
                    1 => val += params.max_knight_bonus / 10,
                    2 => val += params.max_knight_bonus / 4,
                    3 => val += params.max_knight_bonus / 2,
                    4 => val += params.max_knight_bonus / 2,
                    5 => val += params.max_knight_bonus / 4,
                    6 => val += params.max_knight_bonus / 10,
                    _ => (),
                };
                val
            }
            Kind::Bishop => params.bishop_value,
            Kind::Rook => params.rook_value,
            Kind::Queen => params.queen_value,
            Kind::King => 0,
        }
    }

    pub fn naive_value(&self) -> i16 {
        match self.kind {
            Kind::Pawn => 1,
            Kind::Bishop | Kind::Knight => 3,
            Kind::Rook => 5,
            Kind::Queen => 9,
            _ => 0,
        }
    }
}
