use prelude::*;

fn do_main(input: &str) {
    let input = read_lines_from_file(input)
        .map(|line| {
            let mut parts = line.split(" | ");
            let observations = parts
                .next()
                .unwrap()
                .split_ascii_whitespace()
                .map(String::from)
                .collect_vec();
            let output_values = parts
                .exactly_one()
                .unwrap()
                .split_ascii_whitespace()
                .map(String::from)
                .collect_vec();
            (observations, output_values)
        })
        .collect_vec();

    let part1 = input
        .iter()
        .map(|(_, output)| {
            output
                .iter()
                .filter(|&segment| matches!(segment.len(), 2 | 4 | 3 | 7))
        })
        .flatten()
        .count();
    dbg!(part1);
}

fn main() {
    do_main("inputs/day_08.txt");
}
