use prelude::*;

fn main() {
    let input = read_lines_from_file("inputs/day_03.txt").collect::<Vec<_>>();

    let mut gamma = String::new();
    let mut epsilon = String::new();
    for i in 0..input[0].as_bytes().len() {
        let ones = input.iter().filter(|&s| s.as_bytes()[i] == b'1').count();
        let zeroes = input.len() - ones;

        if ones > zeroes {
            gamma.push('1');
            epsilon.push('0');
        } else {
            gamma.push('0');
            epsilon.push('1');
        }
    }

    let gamma = u64::from_str_radix(&gamma, 2).expect("could not parse binary number");
    let epsilon = u64::from_str_radix(&epsilon, 2).expect("could not parse epsilon");

    let part1 = gamma * epsilon;
    dbg!(part1);
}
