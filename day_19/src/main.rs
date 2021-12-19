use prelude::*;

fn do_main(input: &str) {
    let mut scanners = Vec::new();
    let mut this_scanner = Vec::new();

    for line in read_lines_from_file(input).chain(std::iter::once(String::new())) {
        if line.starts_with("---") {
            this_scanner = Vec::new();
            continue;
        }

        if line.is_empty() {
            scanners.push(this_scanner);
            this_scanner = Vec::new(); // one of these is unnecessary, but doesn't hurt.
            continue;
        }

        let coords: [i32; 3] = line
            .split(',')
            .map(|c| c.parse().unwrap())
            .collect_vec()
            .try_into()
            .unwrap();
        this_scanner.push(coords);
    }
    dbg!(scanners);
}

fn main() {
    do_main("inputs/day_19.txt");
}
