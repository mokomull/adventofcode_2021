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
                part2 = part2.checked_mul(5).unwrap();
                match c {
                    '(' => part2 = part2.checked_add(1).unwrap(),
                    '[' => part2 = part2.checked_add(2).unwrap(),
                    '{' => part2 = part2.checked_add(3).unwrap(),
                    '<' => part2 = part2.checked_add(4).unwrap(),
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
