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

fn do_main(input: &str) {
    let mut input = read_lines_from_file(input);
    let template = input.next().unwrap();
    input.next();
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
