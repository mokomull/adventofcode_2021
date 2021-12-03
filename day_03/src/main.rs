use std::cmp::Ordering;

use prelude::*;

fn compare_ones_to_zeroes<'a>(input: &[&'a [u8]], bit_position: usize) -> Ordering {
    let ones = input.iter().filter(|&s| s[bit_position] == b'1').count();
    let zeroes = input.len() - ones;

    ones.cmp(&zeroes)
}

fn main() {
    let input = read_lines_from_file("inputs/day_03.txt").collect::<Vec<_>>();
    let input: Vec<_> = input.iter().map(|s| s.as_bytes()).collect();

    let mut gamma = String::new();
    let mut epsilon = String::new();
    for i in 0..input[0].len() {
        match compare_ones_to_zeroes(&input, i) {
            Ordering::Greater => {
                gamma.push('1');
                epsilon.push('0');
            }
            Ordering::Equal | Ordering::Less => {
                gamma.push('0');
                epsilon.push('1');
            }
        }
    }

    let gamma = u64::from_str_radix(&gamma, 2).expect("could not parse binary number");
    let epsilon = u64::from_str_radix(&epsilon, 2).expect("could not parse epsilon");

    let part1 = gamma * epsilon;
    dbg!(part1);

    let mut oxygen_candidates = input.clone();
    let mut co2_candidates = input.clone();
}
