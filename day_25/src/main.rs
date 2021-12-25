use prelude::*;

#[derive(Copy, Clone, Eq, PartialEq)]
enum Cucumber {
    East,
    South,
    Empty,
}
use Cucumber::*;

fn do_main(input: &str) {
    let input = read_lines_from_file(input)
        .map(|line| {
            line.bytes()
                .map(|b| match b {
                    b'>' => East,
                    b'v' => South,
                    b'.' => Empty,
                    x => panic!("unknown byte {}", x),
                })
                .collect_vec()
        })
        .collect_vec();

    let mut last_step = None;
    let mut cucumbers = input.clone();
    for step in 1.. {
        let mut next_cucumbers = cucumbers.clone();
        let mut moved = 0;

        // move all East cucumbers
        for (i, row) in cucumbers.iter().enumerate() {
            for (j, cucumber) in row.iter().enumerate() {
                if *cucumber != East {
                    continue;
                }

                let nextj = (j + 1) % row.len();
                if cucumbers[i][nextj] == Empty {
                    moved += 1;
                    next_cucumbers[i][j] = Empty;
                    next_cucumbers[i][nextj] = East;
                }
            }
        }

        // and then move all the South cucumbers
        cucumbers = next_cucumbers;
        next_cucumbers = cucumbers.clone();
        for (i, row) in cucumbers.iter().enumerate() {
            for (j, cucumber) in row.iter().enumerate() {
                if *cucumber != South {
                    continue;
                }

                let nexti = (j + 1) % cucumbers.len();
                if cucumbers[nexti][j] == Empty {
                    moved += 1;
                    next_cucumbers[i][j] = Empty;
                    next_cucumbers[nexti][j] = South;
                }
            }
        }
        cucumbers = next_cucumbers;

        if moved == 0 {
            last_step = Some(step);
        }
    }

    let part1 = last_step.unwrap();
    dbg!(part1);
}

fn main() {
    do_main("inputs/day_25.txt");
}
