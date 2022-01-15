use std::collections::HashMap;
use rand::{Rng, thread_rng};
use crate::bird::Bird;
use crate::materialized_state::MaterializedCubirdsState;

#[derive(Debug, Clone)]
pub struct Line(pub Vec<Bird>);

impl Line {
    pub fn new() -> Line {
        Line(Vec::new())
    }

    pub fn sandwich(&mut self, left: bool) -> Option<HashMap<Bird, i32>> {
        let reference: i32 = if left { 0 } else { self.0.len() as i32 - 1 };
        let movement: i32 = if left { 1 } else { -1 };
        let mut start_index = reference;
        while self.0[start_index as usize] == self.0[reference as usize] {
            start_index += movement;
        }
        let mut end_index = start_index + movement;
        while end_index >= 0 && end_index < self.0.len() as i32 && self.0[end_index as usize] != self.0[reference as usize] {
            end_index += movement;
        }
        if end_index >= 0 && end_index < self.0.len() as i32 {
            let mut birds = HashMap::new();
            let lower_bound = if left { start_index } else { end_index + 1 };
            let upper_bound = if left { end_index } else { start_index + 1 };
            let mut i = lower_bound;
            while i < upper_bound {
                let bird = self.0[lower_bound as usize];
                *birds.entry(bird).or_insert(0) += 1;
                self.0.remove(lower_bound as usize);
                i += 1;
            }
            return Some(birds);
        }
        return None
    }

    pub fn play(&mut self, bird: Bird, count: i32, left: bool) -> Option<HashMap<Bird, i32>> {
        if left {
            for _ in 0..count {
                self.0.insert(0, bird);
            }
        } else {
            for _ in 0..count {
                self.0.push(bird);
            }
        }

        return self.sandwich(left);
    }

    pub fn draw_new(&mut self, left: bool, draw_pile: &mut Vec<Bird>, discard_pile: &mut HashMap<Bird, i32>) -> bool {
        while self.0[0] == *self.0.last().unwrap() {
            if let Some(drawn) = MaterializedCubirdsState::draw(draw_pile, discard_pile) {
                if left {
                    self.0.insert(0, drawn);
                } else {
                    self.0.push(drawn);
                }
            } else {
                return false;
            }
        }
        return true;
    }
}
