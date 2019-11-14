#[derive(Copy, Clone, PartialEq)]
pub enum Color {
    White,
    Black,
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Color::White => write!(f, "white"),
            Color::Black => write!(f, "black"),
        }
    }
}

impl Color {
    pub fn hash(&self) -> u64 {
        match self {
            Color::White => 2887066399212044383,
            Color::Black => 14573829706215326249,
        }
    }
}
