use prelude::*;

fn do_main(input: &str) {
    let input = read_lines_from_file(input).collect_vec();

    let mut part1 = 0u64;
    let mut part2 = 0u64;
    for line in &input {
        let mut state = vec![];
        let mut error = false;

        for c in line.chars() {
            match c {
                '[' | '(' | '<' | '{' => {
                    state.push(c);
                }

                _ => {
                    let opener = state.pop().unwrap();
                    match c {
                        ']' if opener != '[' => part1 += 57,
                        ')' if opener != '(' => part1 += 3,
                        '}' if opener != '{' => part1 += 1197,
                        '>' if opener != '<' => part1 += 25137,
                        _ => continue,
                    }
                    error = true;
                    break;
                }
            }
        }

        if !error {
            for c in state.into_iter().rev() {
                part2 *= 5;
                match c {
                    '(' => part2 += 1,
                    '[' => part2 += 2,
                    '{' => part2 += 3,
                    '<' => part2 += 4,
                    _ => panic!("oops {}", c),
                }
            }
        }
    }
    dbg!(part1);
    dbg!(part2);
}

fn main() {
    do_main("inputs/day_10.txt");
}
