use crate::cache::Cache;
use crate::color::Color;
use crate::game::Game;
use std::sync::{Arc, Mutex};

pub trait PushSome<T> {
    fn push_some(&mut self, item: Option<T>);
}

impl<T> PushSome<T> for Vec<T> {
    fn push_some(&mut self, item: Option<T>) {
        match item {
            Some(i) => self.push(i),
            None => (),
        };
    }
}

pub trait BestFirstSort {
    fn best_first_sort(&mut self, depth: u8, game: &Game, color: Color, cache: &Arc<Mutex<Cache>>);
}
