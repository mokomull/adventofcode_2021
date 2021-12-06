use prelude::*;

fn do_main(input: &str) {
    let input = read_lines_from_file(input)
        .map(|line| {
            line.split(',')
                .map(|num| num.parse::<u64>().unwrap())
                .collect_vec()
        })
        .flatten()
        .collect_vec();

    let mut lanternfish = input.clone();
    for _round in 0..80 {
        for i in 0..lanternfish.len() {
            if lanternfish[i] == 0 {
                lanternfish[i] = 7;
                lanternfish.push(8);
            }
            lanternfish[i] -= 1;
        }
    }

    let part1 = lanternfish.len();
    dbg!(part1);
}

fn main() {
    do_main("inputs/day_06.txt");
}
