use crate::color::Color;
use crate::file::File;
use crate::kind::Kind;
use crate::params::Params;
use crate::position::Position;
use crate::rank::Rank;
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
                Color::White => match position.rank {
                    Rank::Two => 100,
                    Rank::Three => 100,
                    Rank::Four => 100 + params.max_pawn_bonus / 8,
                    Rank::Five => 100 + params.max_pawn_bonus / 4,
                    Rank::Six => 100 + params.max_pawn_bonus / 2,
                    Rank::Seven => 100 + params.max_pawn_bonus,
                    _ => 0,
                },
                Color::Black => match position.rank {
                    Rank::Two => 100 + params.max_pawn_bonus,
                    Rank::Three => 100 + params.max_pawn_bonus / 2,
                    Rank::Four => 100 + params.max_pawn_bonus / 4,
                    Rank::Five => 100 + params.max_pawn_bonus / 8,
                    Rank::Six => 100,
                    Rank::Seven => 100,
                    _ => 0,
                },
            },
            Kind::Knight => {
                let mut val = params.knight_value;
                match position.rank {
                    Rank::Two => val += params.max_knight_bonus / 10,
                    Rank::Three => val += params.max_knight_bonus / 4,
                    Rank::Four => val += params.max_knight_bonus / 2,
                    Rank::Five => val += params.max_knight_bonus / 2,
                    Rank::Six => val += params.max_knight_bonus / 4,
                    Rank::Seven => val += params.max_knight_bonus / 10,
                    _ => (),
                };
                match position.file {
                    File::B => val += params.max_knight_bonus / 10,
                    File::C => val += params.max_knight_bonus / 4,
                    File::D => val += params.max_knight_bonus / 2,
                    File::E => val += params.max_knight_bonus / 2,
                    File::F => val += params.max_knight_bonus / 4,
                    File::G => val += params.max_knight_bonus / 10,
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
