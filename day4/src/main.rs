use std::fs::read_to_string;

fn main() {
    let input = read_to_string("src/input.txt").expect("Failed to read input");

    let mut sum = 0;
    for pair in input.lines() {
        let elves: Vec<&str> = pair.split(",").collect();

        let elve1_range: Vec<&str> = elves[0].split("-").collect();
        let elve2_range: Vec<&str> = elves[1].split("-").collect();

        let (from1, to1) = (elve1_range[0], elve1_range[1]);
        let (from2, to2) = (elve2_range[0], elve2_range[1]);

        let from1: u32 = from1.parse().expect("Failed to parse string to number");
        let to1: u32 = to1.parse().expect("Failed to parse string to number");
        let from2: u32 = from2.parse().expect("Failed to parse string to number");
        let to2: u32 = to2.parse().expect("Failed to parse string to number");

        if (from1 >= from2 && to1 <= to2) || (to1 >= to2 && from1 <= from2) {
            sum += 1;
        }
    }

    print!("{sum}")
}
