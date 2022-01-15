use std::collections::{HashMap, HashSet};
use crate::{Bird, STARTING_CARDS_HAND};
use crate::utilities::weighted_choice;

#[derive(Debug, Clone)]
pub struct PartialCards {
    pub known_cards: HashMap<Bird, i32>,
    pub blacklisted_cards: HashSet<Bird>,
    pub total_cards: i32,
}

impl PartialCards {
    pub fn new() -> PartialCards {
        PartialCards{
            known_cards: HashMap::new(),
            blacklisted_cards: HashSet::new(),
            total_cards: STARTING_CARDS_HAND,
        }
    }

    pub fn sample(&self, available_cards: &mut HashMap<Bird, i32>) -> HashMap<Bird, i32> {
        let known_card_count: i32 = self.known_cards.values().sum();
        let unknown_cards = self.total_cards - known_card_count;
        let mut sampled_cards = self.known_cards.clone();
        for _ in 0..unknown_cards {
            let choice = weighted_choice(&available_cards, Some(&self.blacklisted_cards)).unwrap();
            *available_cards.get_mut(&choice).unwrap() -= 1;
            *sampled_cards.entry(choice).or_insert(0) += 1;
        }
        return sampled_cards;
    }
}
