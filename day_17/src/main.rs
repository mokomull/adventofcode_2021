use prelude::*;

fn highest_reached(xvel: i64, yvel: i64, x1: i64, x2: i64, y1: i64, y2: i64) -> Option<i64> {
    panic!("I don't feel like coding this.");
}

fn do_main(input: &str) {
    let format =
        Regex::new("target area: x=(-?[0-9]+)..(-?[0-9]+), y=(-?[0-9]+)..(-?[0-9]+)").unwrap();
    let input = read_lines_from_file(input).next().unwrap();
    let captures = format.captures(&input).unwrap();
    let x1 = captures[0].parse::<i64>().unwrap();
    let x2 = captures[1].parse::<i64>().unwrap();
    let y1 = captures[2].parse::<i64>().unwrap();
    let y2 = captures[3].parse::<i64>().unwrap();

    let mut part1 = 0;
    for xvel in -50..50 {
        for yvel in 0..1000 {
            if let Some(y) = highest_reached(xvel, yvel, x1, x2, y1, y2) {
                if y > part1 {
                    part1 = y
                }
            }
        }
    }
    dbg!(part1);
}

fn main() {
    do_main("inputs/day_17.txt");
}
