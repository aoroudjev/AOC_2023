use std::fs;

fn to_cal_data(mut line: String) -> u32 {
    word_to_digit(&mut line);
    let cal_data = line.find(|c: char| c.is_ascii_digit()).unwrap();
    let r_cal_data = line.rfind(|c: char| c.is_ascii_digit()).unwrap();

    let cal_data_val = line.chars().nth(cal_data).unwrap();
    let r_cal_data_val = line.chars().nth(r_cal_data).unwrap();

    let num = String::from_iter([cal_data_val, r_cal_data_val]);

    num.parse().unwrap()
}

fn word_to_digit(line: &mut String) {
    println!("{}", line);
    let mut digit_map = [
        ("zero", "0"),
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ];

    //fix overlaps prioritizing smaller numbers regardless of order.
    digit_map.sort_by(|&a, &b| a.partial_cmp(&b).unwrap() );

    for (word, digit) in digit_map {
        *line = line.replace(word, digit);
    }

    println!("{}", line);
}

fn main() {
    let input = fs::read_to_string("./src/inputs/day01.txt").unwrap();

    let sum = input.lines().map(|s| s.to_owned()).map(to_cal_data).sum::<u32>();
    println!("{}", sum);
}
