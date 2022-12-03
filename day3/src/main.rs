use std::collections::HashMap;

use itertools::Itertools;
use std::{collections::HashSet, fs::read_to_string};

fn main() {
    let input = read_to_string("src/input.txt").expect("Failed to read input");

    let mut common_items: Vec<char> = Vec::new();

    // Part 1
    // let mut first_compartment_set: HashSet<char> = HashSet::new();
    // let mut second_compartment_set: HashSet<char> = HashSet::new();

    // for rucksack in input.lines() {
    //     let items: Vec<char> = rucksack.chars().collect();

    //     let mut items_set: HashSet<char> = HashSet::new();
    //     for &i in &items {
    //         items_set.insert(i);
    //     }

    //     let compartments = items.split_at(items.len() / 2);

    //     for &item in compartments.0 {
    //         first_compartment_set.insert(item);
    //     }

    //     for &item in compartments.1 {
    //         second_compartment_set.insert(item);
    //     }

    //     let common_item: HashSet<char> = first_compartment_set
    //         .intersection(&second_compartment_set)
    //         .copied()
    //         .collect();

    //     let common_item = common_item
    //         .into_iter()
    //         .nth(0)
    //         .expect("First item should exisits");

    //     common_items.push(common_item);

    //     first_compartment_set.clear();
    //     second_compartment_set.clear();
    // }

    // Part 2

    let mut items1_set: HashSet<char> = HashSet::new();
    let mut items2_set: HashSet<char> = HashSet::new();
    let mut items3_set: HashSet<char> = HashSet::new();

    for (first, second, third) in input.lines().into_iter().tuples() {
        let items1: Vec<char> = first.chars().collect();
        let items2: Vec<char> = second.chars().collect();
        let items3: Vec<char> = third.chars().collect();

        for &i in &items1 {
            items1_set.insert(i);
        }

        for &i in &items2 {
            items2_set.insert(i);
        }

        for &i in &items3 {
            items3_set.insert(i);
        }

        let common_items_intersection: HashSet<char> =
            items1_set.intersection(&items2_set).copied().collect();
        let common_items_intersection: HashSet<char> = common_items_intersection
            .intersection(&items3_set)
            .copied()
            .collect();

        let common_item = common_items_intersection
            .into_iter()
            .nth(0)
            .expect("First item should exisits");

        common_items.push(common_item);

        items1_set.clear();
        items2_set.clear();
        items3_set.clear();
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
