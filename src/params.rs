use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
pub struct Params {
    // material value
    pub max_pawn_bonus: i16,
    pub knight_value: i16,
    pub max_knight_bonus: i16,
    pub bishop_value: i16,
    pub rook_value: i16,
    pub queen_value: i16,

    // king protection
    pub max_king_corner_reward: i16,
    pub max_pawn_protection_reward: i16,

    // misc
    pub center_control_bonus: i16,
    pub lonely_king_cornered_penalty_factor: i16,
    pub many_moves_bonus: i16,
    pub doubled_pawn_penalty: i16,
    pub double_bishop_reward: i16,
}

impl Params {
    pub fn get() -> Self {
        let file = match std::fs::File::open("./params.yml") {
            Ok(f) => f,
            Err(e) => panic!(e),
        };
        match serde_yaml::from_reader(file) {
            Ok(v) => v,
            Err(e) => panic!(e),
        }
    }

    pub fn randomly_mutate(&mut self) -> (String, i16) {
        let mut rng = rand::thread_rng();
        let mut multiplier = 1f32;
        while (multiplier - 1f32).abs() < 0.1 {
            multiplier = rng.gen_range(0.8, 1.2);
        }
        match rng.gen_range(0, 14) {
            0 => {
                self.doubled_pawn_penalty = (self.doubled_pawn_penalty as f32 * multiplier) as i16;
                ("doubled_pawn_penalty".to_string(), self.doubled_pawn_penalty)
            }
            1 => {
                self.max_pawn_bonus = (self.max_pawn_bonus as f32 * multiplier) as i16;
                ("max_pawn_bonus".to_string(), self.max_pawn_bonus)
            }
            2 => {
                self.bishop_value = (self.bishop_value as f32 * multiplier) as i16;
                ("bishop_value".to_string(), self.bishop_value)
            }
            3 => {
                self.knight_value = (self.knight_value as f32 * multiplier) as i16;
                ("knight_value".to_string(), self.knight_value)
            }
            4 => {
                self.max_knight_bonus = (self.max_knight_bonus as f32 * multiplier) as i16;
                ("max_knight_bonus".to_string(), self.max_knight_bonus)
            }
            5 => {
                self.rook_value = (self.rook_value as f32 * multiplier) as i16;
                ("rook_value".to_string(), self.rook_value)
            }
            6 => {
                self.queen_value = (self.queen_value as f32 * multiplier) as i16;
                ("queen_value".to_string(), self.queen_value)
            }
            7 => {
                self.center_control_bonus = (self.center_control_bonus as f32 * multiplier) as i16;
                (
                    "center_control_bonus".to_string(),
                    self.center_control_bonus,
                )
            }
            8 => {
                self.lonely_king_cornered_penalty_factor = (self.lonely_king_cornered_penalty_factor as f32 * multiplier) as i16;
                (
                    "lonely_king_cornered_penalty_factor".to_string(),
                    self.lonely_king_cornered_penalty_factor,
                )
            }
            9 => {
                self.max_pawn_protection_reward = (self.max_pawn_protection_reward as f32 * multiplier) as i16;
                (
                    "max_pawn_protection_reward".to_string(),
                    self.max_pawn_protection_reward,
                )
            }
            10 => {
                self.max_king_corner_reward = (self.max_king_corner_reward as f32 * multiplier) as i16;
                (
                    "max_king_corner_reward".to_string(),
                    self.max_king_corner_reward,
                )
            }
            11 => {
                self.many_moves_bonus = (self.many_moves_bonus as f32 * multiplier) as i16;
                ("many_moves_bonus".to_string(), self.many_moves_bonus)
            }
            _ => {
                self.double_bishop_reward = (self.double_bishop_reward as f32 * multiplier) as i16;
                ("double_bishop_reward".to_string(), self.double_bishop_reward)
            }
        }
    }
}
