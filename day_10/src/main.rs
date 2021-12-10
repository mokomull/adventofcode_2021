use prelude::*;

fn do_main(input: &str) {
    let input = read_lines_from_file(input).collect_vec();

    let mut part1 = 0u64;
    for line in &input {
        let mut state = vec![];
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
                    break;
                }
            }
        }
    }
    dbg!(part1);
}

fn main() {
    do_main("inputs/day_10.txt");
}
