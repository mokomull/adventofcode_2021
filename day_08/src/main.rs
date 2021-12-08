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

            let c_or_f = observations
                .iter()
                .filter(|&obs| obs.len() == 2)
                .next()
                .unwrap()
                .chars()
                .collect_vec();
            assert_eq!(c_or_f.len(), 2);

            // 7 is three segments, and the one that's not C or F must be A.
            let a = observations
                .iter()
                .filter(|&obs| obs.len() == 3)
                .next()
                .unwrap()
                .chars()
                .filter(|seg| !c_or_f.contains(seg))
                .exactly_one()
                .unwrap();

            // 4 is four segments, and the ones that're not C or F must be B or D.
            let b_or_d = observations
                .iter()
                .filter(|&obs| obs.len() == 4)
                .next()
                .unwrap()
                .chars()
                .filter(|seg| !c_or_f.contains(seg))
                .collect_vec();
            assert_eq!(b_or_d.len(), 2);

            // 0, 6, or 9 are both six segments, but only 6 is missing a C or F segment [it's missing C]
            let six = observations
                .iter()
                .filter(|&obs| obs.len() == 6)
                .filter(|&obs| c_or_f.iter().any(|seg| !obs.contains(*seg)))
                .next()
                .unwrap();
            let c = *c_or_f
                .iter()
                .filter(|seg| !six.contains(**seg))
                .exactly_one()
                .unwrap();
            let f = *c_or_f
                .iter()
                .filter(|&&seg| seg != c)
                .exactly_one()
                .unwrap();

            // 2, 3, and 5 have five segments, but can be distinguished by having either C, F, or
            // both.  Since 2 has D and not B, this narrows down more segments.
            let two = observations
                .iter()
                .filter(|&obs| obs.len() == 5)
                .filter(|&obs| obs.contains(c) && !obs.contains(f))
                .next()
                .unwrap();
            let b = *b_or_d
                .iter()
                .filter(|seg| !two.contains(**seg))
                .exactly_one()
                .unwrap();
            let d = *b_or_d
                .iter()
                .filter(|&&seg| seg != b)
                .exactly_one()
                .unwrap();

            // 9 has every segment except E, so that narrows E down
            let nine = observations
                .iter()
                .filter(|&obs| obs.len() == 6)
                .filter(|&obs| obs != six)
                .filter(|&obs| obs.contains(d))
                .next()
                .unwrap();
            let e = ['a', 'b', 'c', 'd', 'e', 'f', 'g']
                .into_iter()
                .filter(|&seg| !nine.contains(seg))
                .exactly_one()
                .unwrap();

            // five segments isn't enough to distinguish all the digits, but six oughtta be.
            // technically, having found six of them, finding G would merely be process of
            // elimination, but it saves a wee bit of coding.
            let value = output
                .iter()
                .map(|digit| {
                    match (
                        digit.contains(a),
                        digit.contains(b),
                        digit.contains(c),
                        digit.contains(d),
                        digit.contains(e),
                        digit.contains(f),
                    ) {
                        (true, true, true, false, true, true) => '0',
                        (false, false, true, false, false, true) => '1',
                        (true, false, true, true, true, false) => '2',
                        (true, false, true, true, false, true) => '3',
                        (false, true, true, true, false, true) => '4',
                        (true, true, false, true, false, true) => '5',
                        (true, true, false, true, true, true) => '6',
                        (true, false, true, false, false, true) => '7',
                        (true, true, true, true, true, true) => '8',
                        (true, true, true, true, false, true) => '9',
                        _ => panic!("not a number: {}", digit),
                    }
                })
                .collect::<String>()
                .parse::<u64>()
                .unwrap();
            value
        })
        .sum::<u64>();
    dbg!(part2);
}

fn main() {
    do_main("inputs/day_08.txt");
}
