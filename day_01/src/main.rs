use std::io::BufRead;

fn main() {
    let input = std::fs::File::open("inputs/day_01.txt").expect("could not open input");
    let input = std::io::BufReader::new(input)
        .lines()
        .map(|s| {
            s.expect("could not lines()")
                .parse()
                .expect("could not parse integer")
        })
        .collect::<Vec<i64>>();

    let part1 = input.windows(2).filter(|s| s[0] < s[1]).count();
    dbg!(part1);

    let part2 = input
        .windows(3)
        .map(|s| s.iter().sum())
        .collect::<Vec<i64>>()
        .windows(2)
        .filter(|s| s[0] < s[1])
        .count();
    dbg!(part2);
}
