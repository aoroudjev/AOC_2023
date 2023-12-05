fn to_cal_data(line: &str) -> u32 {
    let cal_data = line.find(|c: char| c.is_ascii_digit()).unwrap();
    let r_cal_data = line.rfind(|c: char| c.is_ascii_digit()).unwrap();

    let cal_data_val = line.chars().nth(cal_data).unwrap();
    let r_cal_data_val = line.chars().nth(r_cal_data).unwrap();

    let num = String::from_iter([cal_data_val, r_cal_data_val]);

    num.parse().unwrap()
}

fn main() {
    let input = include_str!("../inputs/day01.txt");

    let sum = input.lines().map(to_cal_data).sum::<u32>();

    println!("{}", sum);
}
