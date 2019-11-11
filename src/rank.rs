use crate::direction::Direction;

#[derive(Copy, Clone, PartialEq)]
pub enum Rank {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
}

impl std::convert::From<usize> for Rank {
    fn from(f: usize) -> Rank {
        match f {
            0 => Rank::One,
            1 => Rank::Two,
            2 => Rank::Three,
            3 => Rank::Four,
            4 => Rank::Five,
            5 => Rank::Six,
            6 => Rank::Seven,
            _ => Rank::Eight,
        }
    }
}

impl std::convert::From<Rank> for usize {
    fn from(f: Rank) -> usize {
        match f {
            Rank::One => 0,
            Rank::Two => 1,
            Rank::Three => 2,
            Rank::Four => 3,
            Rank::Five => 4,
            Rank::Six => 5,
            Rank::Seven => 6,
            Rank::Eight => 7,
        }
    }
}

impl Rank {
    pub fn step(&self, dir: Direction) -> Option<Self> {
        match dir {
            Direction::Dec => match self {
                Rank::One => None,
                Rank::Two => Some(Rank::One),
                Rank::Three => Some(Rank::Two),
                Rank::Four => Some(Rank::Three),
                Rank::Five => Some(Rank::Four),
                Rank::Six => Some(Rank::Five),
                Rank::Seven => Some(Rank::Six),
                Rank::Eight => Some(Rank::Seven),
            },
            Direction::Inc => match self {
                Rank::One => Some(Rank::Two),
                Rank::Two => Some(Rank::Three),
                Rank::Three => Some(Rank::Four),
                Rank::Four => Some(Rank::Five),
                Rank::Five => Some(Rank::Six),
                Rank::Six => Some(Rank::Seven),
                Rank::Seven => Some(Rank::Eight),
                Rank::Eight => None,
            },
            Direction::None => Some(*self),
        }
    }

    pub fn hash(&self) -> u64 {
        match self {
            Rank::One => 17783895526275986802,
            Rank::Two => 4833246216619820967,
            Rank::Three => 4440083055930105573,
            Rank::Four => 767873876851926362,
            Rank::Five => 8818440312199795523,
            Rank::Six => 14214638289766147609,
            Rank::Seven => 8667694656679311542,
            Rank::Eight => 6790052024758993717,
        }
    }
}

impl std::fmt::Display for Rank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Rank::One => write!(f, "1"),
            Rank::Two => write!(f, "2"),
            Rank::Three => write!(f, "3"),
            Rank::Four => write!(f, "4"),
            Rank::Five => write!(f, "5"),
            Rank::Six => write!(f, "6"),
            Rank::Seven => write!(f, "7"),
            Rank::Eight => write!(f, "8"),
        }
    }
}
