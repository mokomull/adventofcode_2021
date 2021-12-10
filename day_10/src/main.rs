use prelude::*;

fn do_main(input: &str) {
    let input = read_lines_from_file(input).collect_vec();

    let mut part1 = 0u64;
    let mut incomplete = vec![];
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
            let mut this_score = 0u64;
            for c in state.into_iter().rev() {
                this_score = this_score.checked_mul(5).unwrap();
                match c {
                    '(' => this_score = this_score.checked_add(1).unwrap(),
                    '[' => this_score = this_score.checked_add(2).unwrap(),
                    '{' => this_score = this_score.checked_add(3).unwrap(),
                    '<' => this_score = this_score.checked_add(4).unwrap(),
                    _ => panic!("oops {}", c),
                }
            }
            incomplete.push(this_score)
        }
    }
    dbg!(part1);

    incomplete.sort_unstable();
    assert_eq!(incomplete.len() % 2, 1);
    let part2 = incomplete[incomplete.len() / 2];
    dbg!(part2);
}

fn main() {
    do_main("inputs/day_10.txt");
}
