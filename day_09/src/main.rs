use std::convert::identity;

use prelude::*;

fn do_main(input: &str) {
    let input = read_lines_from_file(input)
        .map(|line| line.bytes().map(|cell| cell - b'0').collect_vec())
        .collect_vec();

    let mut part1 = 0u64;
    for i in 0..input.len() {
        for j in 0..input[0].len() {
            let adjacent = [
                input.get(i).and_then(|row| row.get(j.wrapping_sub(1))),
                input.get(i).and_then(|row| row.get(j + 1)),
                input.get(i.wrapping_sub(1)).and_then(|row| row.get(j)),
                input.get(i + 1).and_then(|row| row.get(j)),
            ];

            if adjacent
                .into_iter()
                .filter_map(identity)
                .all(|&other| other > input[i][j])
            {
                part1 += input[i][j] as u64;
            }
        }
    }

    dbg!(part1);
}

fn main() {
    do_main("inputs/day_09.txt");
}
