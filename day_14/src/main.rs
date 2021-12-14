use prelude::*;

type Rules = HashMap<[char; 2], char>;

fn substitute(rules: &Rules, input: String) -> String {
    let to_insert = input
        .chars()
        .tuple_windows()
        .map(|(l, r)| rules.get(&[l, r]).cloned())
        .chain(std::iter::once(None));

    input
        .chars()
        .map(Option::Some)
        .interleave(to_insert)
        .filter_map(std::convert::identity)
        .collect::<String>()
}

fn min_max_count_by_char(template: &str) -> (usize, usize) {
    let mut elements = template.chars().collect_vec();
    elements.sort_unstable();
    let element_counts = elements
        .iter()
        .group_by(|&c| c)
        .into_iter()
        .map(|(&c, g)| (c, g.collect_vec().len()))
        .collect::<HashMap<char, usize>>();
    let (min, max) = element_counts
        .iter()
        .minmax_by_key(|&(_, count)| count)
        .into_option()
        .unwrap();
    (*min.1, *max.1)
}

fn do_main(input: &str) {
    let mut input = read_lines_from_file(input);
    let mut template = input.next().unwrap();
    input.next();
    let mut rules = Rules::new();
    for line in input {
        let parts = line.split(" -> ").collect_vec();
        let from = parts[0].chars().collect_vec().try_into().unwrap();
        let to = parts[1].chars().exactly_one().unwrap();

        rules.insert(from, to);
    }

    for _round in 0..10 {
        template = substitute(&rules, template);
    }
    let (min, max) = min_max_count_by_char(&template);
    let part1 = max - min;
    dbg!(part1);
}

fn main() {
    do_main("inputs/day_14.txt");
}

#[cfg(test)]
mod test {
    use crate::substitute;

    #[test]
    fn substitution() {
        let rules = [
            (['C', 'H'], 'B'),
            (['H', 'H'], 'N'),
            (['C', 'B'], 'H'),
            (['N', 'H'], 'C'),
            (['H', 'B'], 'C'),
            (['H', 'C'], 'B'),
            (['H', 'N'], 'C'),
            (['N', 'N'], 'C'),
            (['B', 'H'], 'H'),
            (['N', 'C'], 'B'),
            (['N', 'B'], 'B'),
            (['B', 'N'], 'B'),
            (['B', 'B'], 'N'),
            (['B', 'C'], 'B'),
            (['C', 'C'], 'N'),
            (['C', 'N'], 'C'),
        ]
        .into();

        let mut polymer = "NNCB".to_owned();
        polymer = substitute(&rules, polymer);
        assert_eq!(polymer, "NCNBCHB");
        polymer = substitute(&rules, polymer);
        debug_assert_eq!(polymer, "NBCCNBBBCBHCB");
    }
}
