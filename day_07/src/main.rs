use prelude::*;

fn do_main(input: &str) {
    let crabs = read_lines_from_file(input)
        .map(|line| {
            line.split(',')
                .map(|s| s.parse::<i64>().unwrap())
                .collect_vec()
        })
        .flatten()
        .collect_vec();

    let (_target, part1) = (0..=*crabs.iter().max().unwrap())
        .map(|target| {
            (
                target,
                crabs.iter().map(|&crab| (crab - target).abs()).sum::<i64>(),
            )
        })
        .min_by_key(|&(_target, fuel)| fuel)
        .unwrap();
    dbg!(part1);

    let part2 = (0..=*crabs.iter().max().unwrap())
        .map(|target| {
            crabs
                .iter()
                .map(|&crab| {
                    let n = crab - target.abs();
                    n * n + 1 / 2
                })
                .sum::<i64>()
        })
        .min()
        .unwrap();
    dbg!(part2);
}

fn main() {
    do_main("inputs/day_07.txt");
}
