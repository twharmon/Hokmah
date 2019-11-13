#![feature(test)]
#![feature(shrink_to)]
extern crate crossbeam;
extern crate rand;
extern crate serde;
extern crate serde_yaml;
extern crate test;
extern crate regex;
#[macro_use]
extern crate lazy_static;

mod board;
mod color;
mod direction;
pub mod engine;
pub mod eval;
mod file;
pub mod game;
mod kind;
pub mod params;
mod piece;
mod ply;
mod position;
mod rank;
pub mod trainer;
mod traits;
pub mod cache;

#[cfg(test)]
mod tests {
    use super::game::Game;

    pub fn make_game(history: &str) -> Game {
        let mut g = Game::new();
        g.try_plies(history).unwrap();
        g
    }

    pub fn assert_ply_valid(history: &str, s: &str) {
        assert_eq!(ply_is_valid(history, s), true);
    }

    pub fn assert_ply_invalid(history: &str, s: &str) {
        assert_eq!(ply_is_valid(history, s), false);
    }

    pub fn assert_reverts(history: &str, s: &str) {
        let mut g = make_game(history);
        let before = g.board;
        let before_hash = g.hash;
        g.try_ply(s).unwrap();
        g.revert_last_ply();
        assert_eq!(g.board.eq(&before), true);
        assert_eq!(g.hash, before_hash);
    }

    pub fn assert_draw(history: &str) {
        let g = make_game(history);
        assert_eq!(g.is_over, true);
        let has_victor = match g.victor {
            Some(_) => true,
            None => false,
        };
        assert_eq!(has_victor, false);
    }

    pub fn assert_not_draw(history: &str) {
        let g = make_game(history);
        assert_eq!(g.is_over, false);
    }

    fn ply_is_valid(history: &str, s: &str) -> bool {
        let mut g = make_game(history);
        let valid_plies = g.get_valid_plies();
        for ply in &valid_plies {
            if &ply.uci() == s || &ply.san(&valid_plies) == s {
                return true
            }
        }
        false
    }

}
