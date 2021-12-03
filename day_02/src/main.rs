use prelude::*;

enum Instruction {
    Forward(i64),
    Up(i64),
    Down(i64),
}

use Instruction::*;

fn main() {
    let input = read_lines_from_file("inputs/day_02.txt")
        .map(|l| {
            let (direction, count) = l.split_once(" ").expect("no space found");
            let count = count.parse().expect("could not parse count");
            match direction {
                "forward" => Forward(count),
                "up" => Up(count),
                "down" => Down(count),
                x => panic!("unexpected direction {}", x),
            }
        })
        .collect::<Vec<_>>();

    let mut x = 0;
    let mut y = 0;

    for i in input {
        match i {
            Forward(dx) => x += dx,
            Up(dy) => y -= dy,
            Down(dy) => y += dy,
        }
    }

    let part1 = x * y;
    dbg!(part1);
}
