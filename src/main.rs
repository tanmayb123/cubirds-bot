mod utilities;
mod bird;
mod player;
mod line;
mod state;
mod materialized_state;
mod partial_cards;

use std::collections::HashMap;
use std::time::Instant;
use crate::bird::Bird;
use crate::line::Line;
use crate::state::CubirdsState;
use crate::materialized_state::MaterializedCubirdsState;

const LINES: usize = 4;
const STARTING_CARDS_HAND: i32 = 8;

fn evaluate_state(state: &CubirdsState) {
    let mut move_scores = HashMap::new();

    let now = Instant::now();
    while now.elapsed().as_secs() <= 10 {
        let mut sampled = MaterializedCubirdsState::sample_from(state);
        if let Some((fmove, win)) = sampled.rollout() {
            let score = move_scores.entry(fmove).or_insert((0, 0));
            if win {
                score.0 += 1;
            }
            score.1 += 1;

            let mut scores = Vec::new();
            for (fmove, winrate) in &move_scores {
                let score = (winrate.0 as f64) / (winrate.1 as f64);
                scores.push((fmove.simplified(), score * 100.0));
            }
            scores.sort_by(|x, y| y.1.partial_cmp(&x.1).unwrap());
            for i in 0..5 {
                if i == scores.len() {
                    break;
                }
                print!("{} ({}%) ", scores[i].0, scores[i].1);
            }
            print!("\r");
        }
    }

    let mut scores = Vec::new();
    for (fmove, winrate) in &move_scores {
        let score = (winrate.0 as f64) / (winrate.1 as f64);
        scores.push((fmove.simplified(), score * 100.0));
    }
    scores.sort_by(|x, y| y.1.partial_cmp(&x.1).unwrap());
    for i in 0..5 {
        if i == scores.len() {
            break;
        }
        print!("{} ({}%) ", scores[i].0, scores[i].1);
    }
    print!("\n");
}

fn main() {
    let mut state = CubirdsState::initial_state();
    evaluate_state(&state);
}
