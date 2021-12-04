use prelude::*;

enum Instruction {
    Forward(i64),
    Up(i64),
    Down(i64),
}

use Instruction::*;

fn do_main(input: &str) {
    let input = read_lines_from_file(input)
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

    for i in &input {
        match i {
            Forward(dx) => x += dx,
            Up(dy) => y -= dy,
            Down(dy) => y += dy,
        }
    }

    let part1 = x * y;
    dbg!(part1);
    assert_eq!(part1, 1604850);

    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;

    for i in &input {
        match i {
            Forward(dx) => {
                x += dx;
                y += aim * dx;
            }
            Down(dy) => aim += dy,
            Up(dy) => aim -= dy,
        }
    }

    let part2 = x * y;
    dbg!(part2);
    assert_eq!(part2, 1685186100);
}

fn main() {
    do_main("inputs/day_02.txt");
}

#[cfg(test)]
mod test {
    #[test]
    fn main() {
        super::do_main("../inputs/day_02.txt");
    }
}
