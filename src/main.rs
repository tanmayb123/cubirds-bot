mod utilities;
mod bird;
mod player;
mod line;
mod state;
mod materialized_state;
mod partial_cards;

use std::collections::HashMap;
use std::str::FromStr;
use std::io::stdin;
use std::time::Instant;
use std::hash::Hash;
use crate::bird::Bird;
use crate::line::Line;
use crate::state::CubirdsState;
use crate::materialized_state::MaterializedCubirdsState;
use crate::materialized_state::SimplifiableMove;

const LINES: usize = 4;
const STARTING_CARDS_HAND: i32 = 8;

fn evaluate_state<T: SimplifiableMove + Eq + Hash>(state: &CubirdsState, rollout_func: fn(&mut MaterializedCubirdsState) -> Option<(T, bool)>) {
    let mut move_scores = HashMap::new();

    let now = Instant::now();
    while now.elapsed().as_secs() <= 10 {
        let mut sampled = MaterializedCubirdsState::sample_from(state);
        if let Some((fmove, win)) = rollout_func(&mut sampled) {
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

pub struct CommandHandler {
    pub state: CubirdsState,
    prev_state: Option<CubirdsState>,
}

impl CommandHandler {
    fn from(state: CubirdsState) -> CommandHandler {
        CommandHandler {
            state: state,
            prev_state: None,
        }
    }

    fn handle_play(&mut self, components: Vec<&str>) {
        let pnum = usize::from_str(components[1]).unwrap();
        let bird = Bird::from_char(components[2].chars().next().unwrap()).unwrap();
        let count = i32::from_str(components[3]).unwrap();
        let line = usize::from_str(components[4]).unwrap();
        let play_dir = components[5] == "L";
        let mut new: Option<(Vec<Bird>, bool)> = None;
        if components.len() > 6 {
            let new_bird = Bird::from_string(components[6]).unwrap();
            let new_dir = components[7] == "L";
            new = Some((new_bird, new_dir));
        }
        self.state.play(pnum, bird, count, line, play_dir, new);
    }

    fn handle_draw(&mut self, components: Vec<&str>) {
        let pnum = usize::from_str(components[1]).unwrap();
        let mut new_birds: Option<(Bird, Bird)> = None;
        if components.len() > 2 {
            let bird1 = Bird::from_char(components[2].chars().next().unwrap()).unwrap();
            let bird2 = Bird::from_char(components[3].chars().next().unwrap()).unwrap();
            new_birds = Some((bird1, bird2));
        }
        self.state.draw(pnum, new_birds);
    }

    fn handle_fly(&mut self, components: Vec<&str>) {
        let pnum = usize::from_str(components[1]).unwrap();
        let bird = Bird::from_char(components[2].chars().next().unwrap()).unwrap();
        let hand_size = i32::from_str(components[3]).unwrap();
        let flock_small = components[4] == "SMALL";
        self.state.fly(pnum, bird, hand_size, flock_small);
    }

    fn handle_set(&mut self, components: Vec<&str>) {
        let pnum = usize::from_str(components[1]).unwrap();
        let birds = Bird::from_string(components[2]).unwrap();
        self.state.set_birds(pnum, &birds);
    }

    fn handle_reset(&mut self) {
        self.state.reset();
    }

    fn handle_play_score(&mut self) {
        evaluate_state(&self.state, |x| x.full_rollout());
    }

    fn handle_flock_score(&mut self) {
        evaluate_state(&self.state, |x| x.flock_rollout());
    }

    fn handle_undo(&mut self) {
        self.state = self.prev_state.as_ref().unwrap().clone();
        self.prev_state = None;
    }

    pub fn evaluate_command(&mut self) {
        let previous_state = self.state.clone();

        let mut input = String::new();
        let _ = stdin().read_line(&mut input);
        input.pop();

        let components: Vec<&str> = input.split(" ").collect();

        match components[0] {
            "PLAY" => self.handle_play(components),
            "DRAW" => self.handle_draw(components),
            "FLY" => self.handle_fly(components),
            "SET" => self.handle_set(components),
            "RESET" => self.handle_reset(),
            "PLAYSCORE" => self.handle_play_score(),
            "FLOCKSCORE" => self.handle_flock_score(),
            "UNDO" => self.handle_undo(),
            "PRINT" => println!("{:?}", self.state),
            _ => println!("Invalid command."),
        }

        self.prev_state = Some(previous_state);
    }
}

fn main() {
    let state = CubirdsState::initial_state();
    let mut handler = CommandHandler::from(state);

    loop {
        handler.evaluate_command();
    }

    /*loop {
        let evaluate = state.turn == (state.player_idx as usize);
        if evaluate {
            evaluate_state(&state, |x| x.full_rollout());
        }
        state.next_move(&|ns| {
            if evaluate {
                evaluate_state(&ns, |x| x.flock_rollout());
            }
        })
    }*/
}
