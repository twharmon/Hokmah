use crate::direction::Direction;

#[derive(Copy, Clone, PartialEq)]
pub enum File {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

impl std::convert::From<File> for usize {
    fn from(f: File) -> usize {
        match f {
            File::A => 0,
            File::B => 1,
            File::C => 2,
            File::D => 3,
            File::E => 4,
            File::F => 5,
            File::G => 6,
            File::H => 7,
        }
    }
}

impl std::convert::From<usize> for File {
    fn from(f: usize) -> File {
        match f {
            0 => File::A,
            1 => File::B,
            2 => File::C,
            3 => File::D,
            4 => File::E,
            5 => File::F,
            6 => File::G,
            _ => File::H,
        }
    }
}

impl std::fmt::Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            File::A => write!(f, "a"),
            File::B => write!(f, "b"),
            File::C => write!(f, "c"),
            File::D => write!(f, "d"),
            File::E => write!(f, "e"),
            File::F => write!(f, "f"),
            File::G => write!(f, "g"),
            File::H => write!(f, "h"),
        }
    }
}

impl File {
    pub fn step(&self, dir: Direction) -> Option<Self> {
        match dir {
            Direction::Dec => match self {
                File::A => None,
                File::B => Some(File::A),
                File::C => Some(File::B),
                File::D => Some(File::C),
                File::E => Some(File::D),
                File::F => Some(File::E),
                File::G => Some(File::F),
                File::H => Some(File::G),
            },
            Direction::Inc => match self {
                File::A => Some(File::B),
                File::B => Some(File::C),
                File::C => Some(File::D),
                File::D => Some(File::E),
                File::E => Some(File::F),
                File::F => Some(File::G),
                File::G => Some(File::H),
                File::H => None,
            },
            Direction::None => Some(*self),
        }
    }
}
