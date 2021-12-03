use std::cmp::Ordering;

use prelude::*;

fn compare_ones_to_zeroes(input: &[&[u8]], bit_position: usize) -> Ordering {
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

    for i in 0..oxygen_candidates[0].len() {
        if oxygen_candidates.len() == 1 {
            break;
        }

        match compare_ones_to_zeroes(&oxygen_candidates, i) {
            Ordering::Greater | Ordering::Equal => {
                oxygen_candidates.retain(|&s| s[i] == b'1');
            }
            Ordering::Less => oxygen_candidates.retain(|&s| s[i] == b'0'),
        }
    }
    assert!(oxygen_candidates.len() == 1);

    for i in 0..co2_candidates[0].len() {
        if co2_candidates.len() == 1 {
            break;
        }

        match compare_ones_to_zeroes(&co2_candidates, i) {
            Ordering::Greater | Ordering::Equal => co2_candidates.retain(|&s| s[i] == b'0'),
            Ordering::Less => co2_candidates.retain(|&s| s[i] == b'1'),
        }
    }
    assert!(co2_candidates.len() == 1);

    let oxygen = u64::from_str_radix(std::str::from_utf8(oxygen_candidates[0]).unwrap(), 2)
        .expect("can't parse oxygen");
    let co2 = u64::from_str_radix(std::str::from_utf8(co2_candidates[0]).unwrap(), 2)
        .expect("can't parse co2");
    let part2 = oxygen * co2;
    dbg!(part2);
}
