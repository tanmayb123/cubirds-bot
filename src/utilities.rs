use std::collections::{HashMap, HashSet};
use std::collections::hash_map::Entry;
use std::hash::Hash;
use std::fmt::Debug;
use rand::{Rng, thread_rng};

pub fn weighted_choice<T: Eq + Hash + Copy + Debug>(choices: &HashMap<T, i32>, blacklist: Option<&HashSet<T>>) -> Option<T> {
    let mut keys: Vec<T> = choices.keys().cloned().collect();
    if let Some(bl) = blacklist {
        keys = keys.iter().filter(|x| !bl.contains(x)).cloned().collect();
    }
    let total_weights = keys.iter().map(|x| *choices.get(x).unwrap()).sum();
    let random = thread_rng().gen_range(0..total_weights);
    let mut visited = 0;
    for key in keys {
        let weight = choices.get(&key).unwrap();
        visited += weight;
        if visited > random {
            return Some(key);
        }
    }
    return None;
}

pub fn remove_from_hashmap<T: Eq + Hash, U>(map: &mut HashMap<T, U>, index: T) {
    if let Entry::Occupied(o) = map.entry(index) {
        o.remove_entry();
    }
}
