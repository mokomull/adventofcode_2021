use std::{collections::VecDeque, convert::identity};

use prelude::*;

fn do_main(input: &str) {
    let input = read_lines_from_file(input)
        .map(|line| line.bytes().map(|cell| cell - b'0').collect_vec())
        .collect_vec();

    struct Work {
        basin: (usize, usize),
        point: (usize, usize),
    }

    let mut to_visit = VecDeque::new();

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
                part1 += 1 + input[i][j] as u64;
                to_visit.push_back(Work {
                    basin: (i, j),
                    point: (i, j),
                });
            }
        }
    }

    dbg!(part1);

    let mut visited = HashSet::new();
    let mut basins: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();

    while !to_visit.is_empty() {
        let this_work = to_visit.pop_front().unwrap();
        if !visited.insert(this_work.point) {
            continue;
        }

        let (i, j) = this_work.point;
        if input[i][j] == 9 {
            continue;
        }

        // assume that all basins are bordered by "9"s, and I don't actually have to do any gradient
        // calculations.
        basins
            .entry(this_work.basin)
            .or_default()
            .push(this_work.point);

        for (m, n) in [
            (i.wrapping_sub(1), j),
            (i + 1, j),
            (i, j.wrapping_sub(1)),
            (i, j + 1),
        ] {
            if visited.contains(&(m, n))
                || !(0..input.len()).contains(&m)
                || !(0..input[0].len()).contains(&n)
            {
                continue;
            }

            to_visit.push_back(Work {
                point: (m, n),
                basin: this_work.basin,
            });
        }
    }

    let part2 = basins
        .into_iter()
        .map(|(_, cells)| cells.len() as i64)
        .sorted_unstable_by_key(|&size| -size)
        .take(3)
        .product::<i64>();
    dbg!(part2);
}

fn main() {
    do_main("inputs/day_09.txt");
}
