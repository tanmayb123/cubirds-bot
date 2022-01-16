use std::collections::HashMap;
use rand::seq::index::{sample, sample_weighted};
use rand::seq::SliceRandom;
use rand::{Rng, thread_rng};
use crate::bird::Bird;
use crate::line::Line;
use crate::{LINES, STARTING_CARDS_HAND};
use crate::player::{MaterializedPlayer, Player};
use crate::state::CubirdsState;
use crate::utilities::weighted_choice;

pub trait SimplifiableMove {
    fn simplified(&self) -> String;
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct FlockMove {
    pub bird: Option<Bird>,
}

impl SimplifiableMove for FlockMove {
    fn simplified(&self) -> String {
        let mut string = String::new();
        if let Some(bird) = self.bird {
            string += &bird.to_char().to_string();
        } else {
            string += "_";
        }
        return string;
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct LineMove {
    pub line: usize,
    pub bird: Bird,
    pub left: bool,
    pub draw: bool,
}

impl SimplifiableMove for LineMove {
    fn simplified(&self) -> String {
        let mut string = String::new();
        string += &self.bird.to_char().to_string();
        string += &self.line.to_string();
        string += if self.left { "L" } else { "R" };
        string += if self.draw { "D" } else { "P" };
        return string;
    }
}

#[derive(Debug, Clone)]
pub struct MaterializedCubirdsState {
    pub board: [Line; LINES],
    pub players: Vec<MaterializedPlayer>,
    pub player_idx: i32,
    pub draw_pile: Vec<Bird>,
    pub discard_pile: HashMap<Bird, i32>,
    pub turn: usize,
}

impl MaterializedCubirdsState {
    fn sample_pile(pile: &HashMap<Bird, i32>) -> Vec<Bird> {
        let mut sampled = Vec::new();
        for (bird, bird_count) in pile {
            for _ in 0..*bird_count {
                sampled.push(*bird);
            }
        }
        sampled.shuffle(&mut thread_rng());
        return sampled;
    }

    pub fn sample_from(state: &CubirdsState) -> MaterializedCubirdsState {
        let mut available_cards = state.all_available_cards();

        let mut players = Vec::new();
        for player in &state.players {
            players.push(MaterializedPlayer{
                collection: player.collection.clone(),
                cards: player.cards.sample(&mut available_cards),
            });
        }

        let mut discarded = HashMap::new();
        for discard in &state.discard_pile {
            for (discarded_bird, discard_count) in discard.sample(&mut available_cards) {
                *discarded.entry(discarded_bird).or_insert(0) += discard_count;
            }
        }

        let deck = MaterializedCubirdsState::sample_pile(&available_cards);

        return MaterializedCubirdsState{
            board: state.board.clone(),
            players: players,
            player_idx: state.player_idx,
            draw_pile: deck,
            discard_pile: discarded,
            turn: state.turn,
        };
    }

    pub fn draw(draw_pile: &mut Vec<Bird>, discard_pile: &mut HashMap<Bird, i32>) -> Option<Bird> {
        if draw_pile.len() == 0 {
            *draw_pile = MaterializedCubirdsState::sample_pile(&discard_pile);
            *discard_pile = HashMap::new();
        }

        return draw_pile.pop();
    }

    fn reset(&mut self) -> bool {
        for player in &self.players {
            for (discarded, discard_count) in &player.cards {
                *self.discard_pile.entry(*discarded).or_insert(0) += *discard_count;
            }
        }

        for player in &mut self.players {
            player.cards = HashMap::new();
            for _ in 0..STARTING_CARDS_HAND {
                if let Some(drawn) = MaterializedCubirdsState::draw(&mut self.draw_pile, &mut self.discard_pile) {
                    *player.cards.entry(drawn).or_insert(0) += 1;
                } else {
                    return false;
                }
            }
        }

        return true;
    }

    pub fn random_play(&mut self) -> Option<LineMove> {
        let mut player = &mut self.players[self.turn];

        let available_birds: Vec<Bird> = player.cards.keys().cloned().collect();
        let bird_idx = thread_rng().gen_range(0..available_birds.len());
        let bird = available_birds[bird_idx];
        let bird_count = player.cards[&bird];
        player.cards.remove(&bird);

        let line = thread_rng().gen_range(0..LINES);
        let direction = thread_rng().gen_range(0..2) == 0;

        let mut retval = LineMove{
            line: line,
            bird: bird,
            left: direction,
            draw: false,
        };

        if let Some(sandwiched) = self.board[line].play(bird, bird_count, direction) {
            let direction = thread_rng().gen_range(0..2) == 0;
            if !self.board[line].draw_new(direction, &mut self.draw_pile, &mut self.discard_pile) {
                return None;
            }
            for (bird, bird_count) in sandwiched {
                *player.cards.entry(bird).or_insert(0) += bird_count;
            }
        } else {
            let should_draw = thread_rng().gen_range(0..2) == 0;
            if should_draw {
                retval.draw = true;
                for _ in 0..2 {
                    if let Some(drawn) = MaterializedCubirdsState::draw(&mut self.draw_pile, &mut self.discard_pile) {
                        *player.cards.entry(drawn).or_insert(0) += 1;
                    } else {
                        return None;
                    }
                }
            } else if player.cards.keys().len() == 0 {
                if !self.reset() {
                    return None;
                }
            }
        }

        let _ = self.random_flock_play();
        player = &mut self.players[self.turn];

        if let Some(reset_success) = self.determine_reset() {
            if reset_success {
                return Some(retval);
            }
            return None;
        }

        return Some(retval);
    }

    fn random_flock_play(&mut self) -> FlockMove {
        let mut player = &mut self.players[self.turn];

        let flockable = player.flockable();
        let flock_idx = thread_rng().gen_range(0..(flockable.len() + 1));
        if flock_idx != flockable.len() {
            player.fly_home(flockable[flock_idx], &mut self.discard_pile);

            return FlockMove{bird: Some(flockable[flock_idx])};
        }

        return FlockMove{bird: None};
    }

    fn determine_reset(&mut self) -> Option<bool> {
        let player = &mut self.players[self.turn];
        if player.cards.keys().len() == 0 {
            return Some(self.reset());
        }
        self.turn = (self.turn + 1) % self.players.len();
        return None;
    }

    pub fn player_win(&self) -> Option<i32> {
        for (player_idx, player) in self.players.iter().enumerate() {
            if player.collection.keys().len() >= 7 {
                return Some(player_idx as i32);
            }
            let three_count: i32 = player.collection.values().map(|x| if *x >= 3 { 1 } else { 0 }).sum();
            if three_count >= 2 {
                return Some(player_idx as i32);
            }
        }
        return None;
    }

    fn complete_rollout(&mut self) -> Option<i32> {
        let mut win = self.player_win();
        while win == None {
            let cmove = self.random_play();
            if cmove == None {
                return None;
            }
            win = self.player_win();
        }
        return win;
    }

    pub fn full_rollout(&mut self) -> Option<(LineMove, bool)> {
        let first_move = self.random_play().unwrap();
        if let Some(winner) = self.complete_rollout() {
            return Some((first_move, winner == self.player_idx));
        }
        return None;
    }

    pub fn flock_rollout(&mut self) -> Option<(FlockMove, bool)> {
        let first_move = self.random_flock_play();
        if let Some(x) = self.determine_reset() {
            if !x {
                return None;
            }
        }
        if let Some(winner) = self.complete_rollout() {
            return Some((first_move, winner == self.player_idx));
        }
        return None;
    }
}
