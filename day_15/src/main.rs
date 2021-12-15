use prelude::*;

fn do_main(input: &str) {
    let input = read_lines_from_file(input)
        .map(|line| line.bytes().map(|c| c - b'0').collect_vec())
        .collect_vec();

    let mut costs = vec![vec![u64::MAX; input[0].len()]; input.len()];
    let mut to_visit = vec![(0, 0, 0)];

    while !to_visit.is_empty() {
        let (i, j, cost) = to_visit.pop().unwrap();

        if cost < costs[i][j] {
            // dbg!((i, j, cost));
            costs[i][j] = cost;

            let mut add_one = |x, y| to_visit.push((x, y, cost + input[x][y] as u64));

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

    let part1 = costs[input.len() - 1][input[0].len() - 1];
    dbg!(part1);
}

fn main() {
    do_main("inputs/day_15.txt");
}
