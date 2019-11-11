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
            Color::White => 840124342865041275,
            Color::Black => 2565603302264732697,
        }
    }
}
