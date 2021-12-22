use std::ops::RangeInclusive;

use prelude::*;

type Axis = RangeInclusive<i32>;

enum Instruction {
    On,
    Off,
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
            let on_off = match groups.nth(1).unwrap().unwrap().as_str() {
                "on" => On,
                "off" => Off,
                x => panic!("what is {}", x),
            };
            let numbers = groups
                .map(|m| m.unwrap().as_str().parse::<i32>().unwrap())
                .collect_vec();
            (
                on_off,
                numbers[0]..=numbers[1],
                numbers[2]..=numbers[3],
                numbers[4]..=numbers[5],
            )
        })
        .collect_vec();

    let mut cubes = HashSet::new();

    for i in &input {
        for x in (*i.1.start()).max(-50)..=(*i.1.end()).min(50) {
            for y in (*i.2.start()).max(-50)..=(*i.2.end()).min(50) {
                for z in (*i.3.start()).max(-50)..=(*i.3.end()).min(50) {
                    match i.0 {
                        On => {
                            cubes.insert((x, y, z));
                        }
                        Off => {
                            cubes.remove(&(x, y, z));
                        }
                    }
                }
            }
        }
    }

    let part1 = cubes.len();
    dbg!(part1);
}

fn main() {
    do_main("inputs/day_22.txt");
}
