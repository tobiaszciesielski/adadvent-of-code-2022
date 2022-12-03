use std::collections::HashMap;

use std::{collections::HashSet, fs::read_to_string};

fn main() {
    let input = read_to_string("src/input.txt").expect("Failed to read input");

    let mut first_compartment_set: HashSet<char> = HashSet::new();
    let mut second_compartment_set: HashSet<char> = HashSet::new();

    let mut common_items: Vec<char> = Vec::new();
    for rucksack in input.lines() {
        let items: Vec<char> = rucksack.chars().collect();

        let mut items_set: HashSet<char> = HashSet::new();
        for &i in &items {
            items_set.insert(i);
        }

        let compartments = items.split_at(items.len() / 2);

        for &item in compartments.0 {
            first_compartment_set.insert(item);
        }

        for &item in compartments.1 {
            second_compartment_set.insert(item);
        }

        let common_item: HashSet<char> = first_compartment_set
            .intersection(&second_compartment_set)
            .copied()
            .collect();

        let common_item = common_item
            .into_iter()
            .nth(0)
            .expect("First item should exisits");

        common_items.push(common_item);

        first_compartment_set.clear();
        second_compartment_set.clear();
    }

    let alphabet = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars();

    let mut scores: HashMap<char, usize> = HashMap::new();
    for (i, c) in alphabet.enumerate() {
        scores.insert(c, i + 1);
    }

    let mut sum = 0;
    for common_item in common_items {
        sum += scores[&common_item]
    }
    print!("{}", sum)
}
