use std::collections::{HashMap, HashSet};
use crate::STARTING_CARDS_HAND;
use crate::bird::Bird;
use crate::partial_cards::PartialCards;

#[derive(Debug, Clone)]
pub struct Player {
    pub collection: HashMap<Bird, i32>,
    pub cards: PartialCards,
}

#[derive(Debug, Clone)]
pub struct MaterializedPlayer {
    pub collection: HashMap<Bird, i32>,
    pub cards: HashMap<Bird, i32>,
}

impl Player {
    pub fn new() -> Player {
        Player{
            collection: HashMap::new(),
            cards: PartialCards::new(),
        }
    }
}

impl MaterializedPlayer {
    pub fn flockable(&self) -> Vec<Bird> {
        let mut valid = Vec::new();
        for (bird, bird_count) in &self.cards {
            if *bird_count >= bird.information().small {
                valid.push(*bird);
            }
        }
        return valid;
    }

    pub fn fly_home(&mut self, bird: Bird, discard_pile: &mut HashMap<Bird, i32>) {
        let bird_count = *self.cards.get(&bird).unwrap();
        self.cards.remove(&bird);
        let large = bird_count >= bird.information().large;
        let flock_size = if large { 2 } else { 1 };
        *self.collection.entry(bird).or_insert(0) += flock_size;
        *discard_pile.entry(bird).or_insert(0) += bird_count - flock_size;
    }
}
