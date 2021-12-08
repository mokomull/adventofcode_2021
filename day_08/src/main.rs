use prelude::*;

fn do_main(input: &str) {
    let input = read_lines_from_file(input)
        .map(|line| {
            let mut parts = line.split(" | ");
            let observations = parts
                .next()
                .unwrap()
                .split_ascii_whitespace()
                .map(String::from)
                .collect_vec();
            let output_values = parts
                .exactly_one()
                .unwrap()
                .split_ascii_whitespace()
                .map(String::from)
                .collect_vec();
            (observations, output_values)
        })
        .collect_vec();

    let part1 = input
        .iter()
        .map(|(_, output)| {
            output
                .iter()
                .filter(|&segment| matches!(segment.len(), 2 | 4 | 3 | 7))
        })
        .flatten()
        .count();
    dbg!(part1);

    let part2 = input
        .iter()
        .map(|(observations, output)| {
            // the problem statement says the signals are
            //     a
            //   b   c
            //     d
            //   e   f
            //     g

            // we can't just iteratively intersect the set of possibilities, because *all* of the
            // unique-count-of-segments have both C and F lit.

            // signals a-g correspond to indexes 0 through 6 in the following array of possibilities:
            // created with the trick in https://stackoverflow.com/a/69756635 to initialize with
            // something that's not Copy (i.e. a HashSet)
            let mut possibilities = [(); 7].map(|()| {
                ['a', 'b', 'c', 'd', 'e', 'f', 'g']
                    .into_iter()
                    .collect::<HashSet<_>>()
            });

            for observation in observations {
                let potential_segments: &[char] = match observation.len() {
                    2 => &['c', 'f'],
                    4 => &['b', 'c', 'd', 'f'],
                    3 => &['a', 'c', 'f'],
                    _ => continue,
                };

                for c in observation.chars() {
                    possibilities[c as usize - 'a' as usize]
                        .retain(|c| potential_segments.contains(c));
                }
            }

            dbg!(possibilities);
        })
        .collect_vec();
}

fn main() {
    do_main("inputs/day_08.txt");
}
