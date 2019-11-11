use crate::direction::Direction;
use crate::file::File;
use crate::rank::Rank;
use crate::piece::Piece;
use std::ops::BitXor;

pub static ALL_POSITIONS: [Position; 64] = [
    Position {
        file: File::E,
        rank: Rank::Eight,
    },
    Position {
        file: File::E,
        rank: Rank::One,
    },
    Position {
        file: File::A,
        rank: Rank::Eight,
    },
    Position {
        file: File::B,
        rank: Rank::Eight,
    },
    Position {
        file: File::C,
        rank: Rank::Eight,
    },
    Position {
        file: File::D,
        rank: Rank::Eight,
    },
    Position {
        file: File::F,
        rank: Rank::Eight,
    },
    Position {
        file: File::G,
        rank: Rank::Eight,
    },
    Position {
        file: File::H,
        rank: Rank::Eight,
    },
    Position {
        file: File::A,
        rank: Rank::One,
    },
    Position {
        file: File::B,
        rank: Rank::One,
    },
    Position {
        file: File::C,
        rank: Rank::One,
    },
    Position {
        file: File::D,
        rank: Rank::One,
    },
    Position {
        file: File::F,
        rank: Rank::One,
    },
    Position {
        file: File::G,
        rank: Rank::One,
    },
    Position {
        file: File::H,
        rank: Rank::One,
    },
    Position {
        file: File::A,
        rank: Rank::Two,
    },
    Position {
        file: File::B,
        rank: Rank::Two,
    },
    Position {
        file: File::C,
        rank: Rank::Two,
    },
    Position {
        file: File::D,
        rank: Rank::Two,
    },
    Position {
        file: File::E,
        rank: Rank::Two,
    },
    Position {
        file: File::F,
        rank: Rank::Two,
    },
    Position {
        file: File::G,
        rank: Rank::Two,
    },
    Position {
        file: File::H,
        rank: Rank::Two,
    },
    Position {
        file: File::A,
        rank: Rank::Seven,
    },
    Position {
        file: File::B,
        rank: Rank::Seven,
    },
    Position {
        file: File::C,
        rank: Rank::Seven,
    },
    Position {
        file: File::D,
        rank: Rank::Seven,
    },
    Position {
        file: File::E,
        rank: Rank::Seven,
    },
    Position {
        file: File::F,
        rank: Rank::Seven,
    },
    Position {
        file: File::G,
        rank: Rank::Seven,
    },
    Position {
        file: File::H,
        rank: Rank::Seven,
    },
    Position {
        file: File::A,
        rank: Rank::Three,
    },
    Position {
        file: File::B,
        rank: Rank::Three,
    },
    Position {
        file: File::C,
        rank: Rank::Three,
    },
    Position {
        file: File::D,
        rank: Rank::Three,
    },
    Position {
        file: File::E,
        rank: Rank::Three,
    },
    Position {
        file: File::F,
        rank: Rank::Three,
    },
    Position {
        file: File::G,
        rank: Rank::Three,
    },
    Position {
        file: File::H,
        rank: Rank::Three,
    },
    Position {
        file: File::A,
        rank: Rank::Four,
    },
    Position {
        file: File::B,
        rank: Rank::Four,
    },
    Position {
        file: File::C,
        rank: Rank::Four,
    },
    Position {
        file: File::D,
        rank: Rank::Four,
    },
    Position {
        file: File::E,
        rank: Rank::Four,
    },
    Position {
        file: File::F,
        rank: Rank::Four,
    },
    Position {
        file: File::G,
        rank: Rank::Four,
    },
    Position {
        file: File::H,
        rank: Rank::Four,
    },
    Position {
        file: File::A,
        rank: Rank::Five,
    },
    Position {
        file: File::B,
        rank: Rank::Five,
    },
    Position {
        file: File::C,
        rank: Rank::Five,
    },
    Position {
        file: File::D,
        rank: Rank::Five,
    },
    Position {
        file: File::E,
        rank: Rank::Five,
    },
    Position {
        file: File::F,
        rank: Rank::Five,
    },
    Position {
        file: File::G,
        rank: Rank::Five,
    },
    Position {
        file: File::H,
        rank: Rank::Five,
    },
    Position {
        file: File::A,
        rank: Rank::Six,
    },
    Position {
        file: File::B,
        rank: Rank::Six,
    },
    Position {
        file: File::C,
        rank: Rank::Six,
    },
    Position {
        file: File::D,
        rank: Rank::Six,
    },
    Position {
        file: File::E,
        rank: Rank::Six,
    },
    Position {
        file: File::F,
        rank: Rank::Six,
    },
    Position {
        file: File::G,
        rank: Rank::Six,
    },
    Position {
        file: File::H,
        rank: Rank::Six,
    },
];

pub static CENTER_POSITIONS: [Position; 4] = [
    Position {
        file: File::D,
        rank: Rank::Four,
    },
    Position {
        file: File::D,
        rank: Rank::Five,
    },
    Position {
        file: File::E,
        rank: Rank::Four,
    },
    Position {
        file: File::E,
        rank: Rank::Five,
    },
];

#[derive(Copy, Clone, PartialEq)]
pub struct Position {
    pub file: File,
    pub rank: Rank,
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.file, self.rank)
    }
}

impl Position {
    pub fn step(&self, file_dir: Direction, rank_dir: Direction) -> Option<Self> {
        let mut file = self.file;
        let mut rank = self.rank;
        file = match file.step(file_dir) {
            Some(f) => f,
            None => return None,
        };

        rank = match rank.step(rank_dir) {
            Some(r) => r,
            None => return None,
        };

        Some(Position { file, rank })
    }

    pub fn step_n(&self) -> Option<Self> {
        self.step(Direction::None, Direction::Inc)
    }

    pub fn step_nn(&self) -> Option<Self> {
        match self.step_n() {
            Some(p) => p.step_n(),
            None => None,
        }
    }

    pub fn step_nne(&self) -> Option<Self> {
        match self.step_ne() {
            Some(p) => p.step_n(),
            None => None,
        }
    }

    pub fn step_ne(&self) -> Option<Self> {
        self.step(Direction::Inc, Direction::Inc)
    }

    pub fn step_nene(&self) -> Option<Self> {
        match self.step_ne() {
            Some(p) => p.step_ne(),
            None => None,
        }
    }

    pub fn step_ene(&self) -> Option<Self> {
        match self.step_ne() {
            Some(p) => p.step_e(),
            None => None,
        }
    }

    pub fn step_e(&self) -> Option<Self> {
        self.step(Direction::Inc, Direction::None)
    }

    pub fn step_ee(&self) -> Option<Self> {
        match self.step_e() {
            Some(p) => p.step_e(),
            None => None,
        }
    }

    pub fn step_ese(&self) -> Option<Self> {
        match self.step_se() {
            Some(p) => p.step_e(),
            None => None,
        }
    }

    pub fn step_se(&self) -> Option<Self> {
        self.step(Direction::Inc, Direction::Dec)
    }

    pub fn step_sese(&self) -> Option<Self> {
        match self.step_se() {
            Some(p) => p.step_se(),
            None => None,
        }
    }

    pub fn step_sse(&self) -> Option<Self> {
        match self.step_se() {
            Some(p) => p.step_s(),
            None => None,
        }
    }

    pub fn step_s(&self) -> Option<Self> {
        self.step(Direction::None, Direction::Dec)
    }

    pub fn step_ss(&self) -> Option<Self> {
        match self.step_s() {
            Some(p) => p.step_s(),
            None => None,
        }
    }

    pub fn step_ssw(&self) -> Option<Self> {
        match self.step_sw() {
            Some(p) => p.step_s(),
            None => None,
        }
    }

    pub fn step_sw(&self) -> Option<Self> {
        self.step(Direction::Dec, Direction::Dec)
    }

    pub fn step_swsw(&self) -> Option<Self> {
        match self.step_sw() {
            Some(p) => p.step_sw(),
            None => None,
        }
    }

    pub fn step_wsw(&self) -> Option<Self> {
        match self.step_sw() {
            Some(p) => p.step_w(),
            None => None,
        }
    }

    pub fn step_w(&self) -> Option<Self> {
        self.step(Direction::Dec, Direction::None)
    }

    pub fn step_ww(&self) -> Option<Self> {
        match self.step_w() {
            Some(p) => p.step_w(),
            None => None,
        }
    }

    pub fn step_wnw(&self) -> Option<Self> {
        match self.step_nw() {
            Some(p) => p.step_w(),
            None => None,
        }
    }

    pub fn step_nw(&self) -> Option<Self> {
        self.step(Direction::Dec, Direction::Inc)
    }

    pub fn step_nwnw(&self) -> Option<Self> {
        match self.step_nw() {
            Some(p) => p.step_nw(),
            None => None,
        }
    }

    pub fn step_nnw(&self) -> Option<Self> {
        match self.step_nw() {
            Some(p) => p.step_n(),
            None => None,
        }
    }

    pub fn is_black(&self) -> bool {
        (usize::from(self.file) + usize::from(self.rank)) % 2 == 0
    }

    pub fn hash(&self, piece: &Piece) -> u64 {
        piece.hash()
            .bitxor(self.file.hash())
            .bitxor(self.rank.hash())
    }
}

pub fn north_stepper(p: &Position) -> Option<Position> {
    p.step(Direction::None, Direction::Inc)
}

pub fn north_east_stepper(p: &Position) -> Option<Position> {
    p.step(Direction::Inc, Direction::Inc)
}

pub fn north_west_stepper(p: &Position) -> Option<Position> {
    p.step(Direction::Dec, Direction::Inc)
}

pub fn east_stepper(p: &Position) -> Option<Position> {
    p.step(Direction::Inc, Direction::None)
}

pub fn west_stepper(p: &Position) -> Option<Position> {
    p.step(Direction::Dec, Direction::None)
}

pub fn south_east_stepper(p: &Position) -> Option<Position> {
    p.step(Direction::Inc, Direction::Dec)
}

pub fn south_west_stepper(p: &Position) -> Option<Position> {
    p.step(Direction::Dec, Direction::Dec)
}

pub fn south_stepper(p: &Position) -> Option<Position> {
    p.step(Direction::None, Direction::Dec)
}
