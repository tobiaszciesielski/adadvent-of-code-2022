use std::fs;

fn main() {
    let calories =
        fs::read_to_string("src/input.txt").expect("Should have been able to read the file");

    let mut highest_calories: u64 = 0;

    let groups = calories.split("\n\n").collect::<Vec<&str>>();

    let mut elves: Vec<u64> = Vec::new();
    for group in groups {
        let mut current_calories: u64 = 0;
        for calories in group.split_whitespace() {
            let num: u64 = calories.parse().expect("Should be number");
            current_calories = current_calories + num;
        }
        elves.push(current_calories)
    }

    elves.sort();

    let len = elves.len();
    let sum = elves[len - 1] + elves[len - 2] + elves[len - 3];

    print!("{sum}")
}
