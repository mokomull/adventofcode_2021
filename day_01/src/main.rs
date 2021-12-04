use prelude::*;

fn do_main(input: &str) {
    let input = read_lines_from_file(input)
        .map(|s| s.parse().expect("could not parse integer"))
        .collect::<Vec<i64>>();

    let part1 = input.windows(2).filter(|s| s[0] < s[1]).count();
    dbg!(part1);
    assert_eq!(part1, 1374);

    let part2 = input
        .windows(3)
        .map(|s| s.iter().sum())
        .collect::<Vec<i64>>()
        .windows(2)
        .filter(|s| s[0] < s[1])
        .count();
    dbg!(part2);
    assert_eq!(part2, 1418);
}

fn main() {
    do_main("inputs/day_01.txt");
}

#[cfg(test)]
mod test {
    #[test]
    fn main() {
        super::do_main("../inputs/day_01.txt");
    }
}
