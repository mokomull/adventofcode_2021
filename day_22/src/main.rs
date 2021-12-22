use std::ops::RangeInclusive;

use prelude::*;

type Axis = RangeInclusive<i32>;

enum Instruction {
    On(Axis, Axis, Axis),
    Off(Axis, Axis, Axis),
}
use Instruction::*;

fn do_main(input: &str) {
    let regex = Regex::new(
        "(on|off) x=(-?[0-9]+)..(-?[0-9]+),y=(-?[0-9]+)..(-?[0-9]+),z=(-?[0-9]+)..(-?[0-9]+)",
    )
    .unwrap();
    let input = read_lines_from_file(input)
        .map(|line| {
            let groups = regex.captures(&line).unwrap();
            let mut groups = groups.iter();
            let on_off = groups.nth(1).unwrap().unwrap();
            let numbers = groups
                .map(|m| m.unwrap().as_str().parse::<i32>().unwrap())
                .collect_vec();
            match on_off.as_str() {
                "on" => On(
                    numbers[0]..=numbers[1],
                    numbers[2]..=numbers[3],
                    numbers[4]..=numbers[5],
                ),
                "off" => Off(
                    numbers[0]..=numbers[1],
                    numbers[2]..=numbers[3],
                    numbers[4]..=numbers[5],
                ),
                x => panic!("what is {}", x),
            }
        })
        .collect_vec();
}

fn main() {
    do_main("inputs/day_22.txt");
}
