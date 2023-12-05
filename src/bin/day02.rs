use itertools::Itertools;
use std::collections::HashMap;
use std::fs;

fn main() {
    let mut sum: usize = 0;
    let input: String = fs::read_to_string("./src/inputs/day02.txt").unwrap();
    let lookup: HashMap<&str, u32> =
        HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

    for (index, line) in input.lines().enumerate() {
        let game: Vec<&str> = line.split(':').nth(1).unwrap().split(';').collect();

        if game_valid(&game, &lookup) {
            sum += index + 1;
        }
    }

    println!("{}", sum);
}

fn game_valid(game: &Vec<&str>, lookup: &HashMap<&str, u32>) -> bool {
    for draw_set in game {
        if !draw_valid(draw_set, lookup) {
            return false;
        }
    }

    true
}

fn draw_valid(draw: &str, lookup: &HashMap<&str, u32>) -> bool {
    for color in draw.split(',') {
        let (num, color) = color.split_ascii_whitespace().collect_tuple().unwrap();
        if num.parse::<u32>().unwrap() > *lookup.get(color).unwrap() {
            return false;
        }
    }

    true
}
