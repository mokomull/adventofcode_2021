use prelude::*;

use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn top_left_to_bottom_right_cost(input: &[Vec<u8>]) -> u64 {
    let mut costs = vec![vec![u64::MAX; input[0].len()]; input.len()];
    let mut to_visit: BinaryHeap<Reverse<(u64, usize, usize)>> = BinaryHeap::new();

    to_visit.push(Reverse((0, 0, 0)));

    while !to_visit.is_empty() {
        let Reverse((cost, i, j)) = to_visit.pop().unwrap();

        if cost < costs[i][j] {
            // dbg!((i, j, cost));
            costs[i][j] = cost;

            let mut add_one = |x, y| {
                let row: &Vec<u8> = &input[x]; // because apparently input[x][y] is no longer able to be type-inferred
                to_visit.push(Reverse((cost + (row[y] as u64), x, y)))
            };

            if i > 0 {
                add_one(i - 1, j);
            }
            if i < input.len() - 1 {
                add_one(i + 1, j);
            }
            if j > 0 {
                add_one(i, j - 1);
            }
            if j < input[0].len() - 1 {
                add_one(i, j + 1);
            }
        }
    }

    costs[input.len() - 1][input[0].len() - 1]
}

fn do_main(input: &str) {
    let input = read_lines_from_file(input)
        .map(|line| line.bytes().map(|c| c - b'0').collect_vec())
        .collect_vec();

    let part1 = top_left_to_bottom_right_cost(&input);
    dbg!(part1);

    let mut extended = input.clone();
    for n in 1..5 {
        for (i, row) in extended.iter_mut().enumerate() {
            for &val in &input[i] {
                row.push(if val + n < 10 { val + n } else { val + n - 9 })
            }
        }
    }
    for n in 1..5 {
        for i in 0..input.len() {
            let mut new_row = extended[i].clone();
            for val in new_row.iter_mut() {
                *val += n;
                if *val > 9 {
                    *val -= 9;
                }
            }
            extended.push(new_row);
        }
    }
    // dbg!(&extended);
    let part2 = top_left_to_bottom_right_cost(&extended);
    dbg!(part2);
}

fn main() {
    do_main("inputs/day_15.txt");
}
