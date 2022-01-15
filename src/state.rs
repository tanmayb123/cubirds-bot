use std::collections::{HashMap, HashSet};
use std::io::stdin;
use std::str::FromStr;
use crate::bird::Bird;
use crate::line::Line;
use crate::{LINES, STARTING_CARDS_HAND};
use crate::materialized_state::MaterializedCubirdsState;
use crate::player::Player;
use strum::IntoEnumIterator;
use crate::partial_cards::PartialCards;
use crate::utilities::remove_from_hashmap;

#[derive(Debug, Clone)]
pub struct CubirdsState {
    pub board: [Line; LINES],
    pub players: Vec<Player>,
    pub player_idx: i32,
    pub discard_pile: Vec<PartialCards>,
    pub total_discarded: i32,
    pub turn: usize,
}

impl CubirdsState {
    pub fn new() -> CubirdsState {
        CubirdsState{
            board: [
                Line::new(),
                Line::new(),
                Line::new(),
                Line::new()
            ],
            players: Vec::new(),
            player_idx: 0,
            discard_pile: Vec::new(),
            total_discarded: 0,
            turn: 0,
        }
    }

    pub fn all_available_cards(&self) -> HashMap<Bird, i32> {
        let mut cards_available = HashMap::new();
        for bird in Bird::iter() {
            cards_available.insert(bird, bird.information().cards);
        }
        for player in &self.players {
            for (bird, &count) in player.cards.known_cards.iter() {
                *cards_available.get_mut(bird).unwrap() -= count;
            }
            for (bird, &count) in player.collection.iter() {
                *cards_available.get_mut(bird).unwrap() -= count;
            }
        }
        for line_idx in 0..LINES {
            let line = &self.board[line_idx];
            for bird in &line.0 {
                *cards_available.get_mut(bird).unwrap() -= 1;
            }
        }
        for discard in &self.discard_pile {
            for (discarded, discard_count) in &discard.known_cards {
                *cards_available.get_mut(discarded).unwrap() -= *discard_count;
            }
        }
        return cards_available;
    }

    fn get_single_bird() -> Bird {
        let mut bird_string = String::new();
        let _ = stdin().read_line(&mut bird_string);
        bird_string.pop();

        let bird = Bird::from_char(bird_string.chars().next().unwrap()).unwrap();
        return bird;
    }

    fn get_multiple_birds() -> Vec<Bird> {
        let mut hand_string = String::new();
        let _ = stdin().read_line(&mut hand_string);
        hand_string.pop();

        let birds = Bird::from_string(hand_string.as_str()).unwrap();
        return birds;
    }

    fn get_number() -> usize {
        let mut number_string = String::new();
        let _ = stdin().read_line(&mut number_string);
        number_string.pop();

        let number = usize::from_str(number_string.as_str()).unwrap();
        return number;
    }

    pub fn initial_state() -> CubirdsState {
        let mut state = CubirdsState::new();

        {
            println!("Number of players:");
            let players = CubirdsState::get_number() as i32;
            for _ in 0..players {
                state.players.push(Player::new());
            }
        }

        {
            println!("First player:");
            state.turn = CubirdsState::get_number();
        }

        {
            println!("Main player:");
            state.player_idx = CubirdsState::get_number() as i32;
        }

        {
            println!("Player initial hand:");
            for bird in CubirdsState::get_multiple_birds() {
                *state.players[state.player_idx as usize].cards.known_cards.entry(bird).or_insert(0) += 1;
            }
        }

        {
            for index in 0..state.players.len() {
                println!("Partial player {} initial collection:", index);
                state.players[index].collection.insert(CubirdsState::get_single_bird(), 1);
            }
        }

        {
            for index in 0..LINES {
                println!("Line {}:", index);
                state.board[index] = Line(CubirdsState::get_multiple_birds());
            }
        }

        return state;
    }

    pub fn next_move(&mut self) {
        println!("Turn {}", self.turn);

        let mut player = &mut self.players[self.turn];
        let is_main = self.turn == (self.player_idx as usize);

        let determine_player_draw = |player: &mut Player| {
            println!("Did player draw?");
            let draw = CubirdsState::get_number() == 0;

            if draw {
                if is_main {
                    println!("Cards drawn:");
                    let birds = CubirdsState::get_multiple_birds();

                    for bird in birds {
                        *player.cards.known_cards.entry(bird).or_insert(0) += 1;
                        player.cards.total_cards += 1;
                    }
                } else {
                    player.cards.blacklisted_cards = HashSet::new();
                    player.cards.total_cards += 2;
                }
            }
            
            return draw;
        };

        println!("Bird placed:");
        let bird = CubirdsState::get_single_bird();
        println!("Bird count:");
        let bird_count = CubirdsState::get_number() as i32;
        println!("Line:");
        let line = CubirdsState::get_number();
        println!("Direction:");
        let direction = CubirdsState::get_number() == 0;

        remove_from_hashmap(&mut player.cards.known_cards, bird);
        player.cards.blacklisted_cards.insert(bird);
        player.cards.total_cards -= bird_count;

        if let Some(sandwiched) = self.board[line].play(bird, bird_count, direction) {
            println!("Birds new on line:");
            let new_birds = CubirdsState::get_multiple_birds();
            println!("New birds direction:");
            let new_direction = CubirdsState::get_number() == 0;

            if new_direction {
                for (bird_idx, bird) in new_birds.iter().enumerate() {
                    self.board[line].0.insert(bird_idx, *bird);
                }
            } else {
                for bird in new_birds {
                    self.board[line].0.push(bird);
                }
            }

            for (sbird, sbird_count) in sandwiched {
                *player.cards.known_cards.entry(sbird).or_insert(0) += sbird_count;
                player.cards.total_cards += sbird_count;
            }
        } else {
            let _ = determine_player_draw(&mut player, );
        }

        println!("Did player fly home?");
        let flew_home = CubirdsState::get_number() == 0;
        if flew_home {
            println!("Bird flown home:");
            let bird_flown = CubirdsState::get_single_bird();

            println!("Flock size:");
            let flock_small = CubirdsState::get_number() == 0;

            println!("New discard size:");
            let discard_size = CubirdsState::get_number();

            let old_discard_size = self.discard_pile.iter().map(|x| x.total_cards).sum::<i32>() as usize;
            let new_discarded = discard_size - old_discard_size;
            let birds_required = if flock_small { bird_flown.information().small } else { bird_flown.information().large } as usize;
            let flown_count = new_discarded + birds_required;

            remove_from_hashmap(&mut player.cards.known_cards, bird_flown);
            player.cards.blacklisted_cards.insert(bird_flown);
            player.cards.total_cards -= flown_count as i32;

            *player.collection.entry(bird_flown).or_insert(0) += if flock_small { 1 } else { 2 };

            if new_discarded > 0 {
                let mut discarded_cards = PartialCards{
                    known_cards: HashMap::new(),
                    blacklisted_cards: HashSet::new(),
                    total_cards: new_discarded as i32,
                };
                discarded_cards.known_cards.insert(bird_flown, new_discarded as i32);
                self.discard_pile.push(discarded_cards);
            }
        }

        if player.cards.total_cards == 0 {
            if !determine_player_draw(&mut player) {
                for player in &mut self.players {
                    player.cards.known_cards = HashMap::new();
                    player.cards.blacklisted_cards = HashSet::new();
                    player.cards.total_cards = STARTING_CARDS_HAND;
                }

                println!("Player hand:");
                for bird in CubirdsState::get_multiple_birds() {
                    *self.players[self.player_idx as usize].cards.known_cards.entry(bird).or_insert(0) += 1;
                }
            }
        }
    }
}
