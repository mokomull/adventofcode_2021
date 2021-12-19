use prelude::*;

// Since every substitution inserts a letter into the middle, it transforms one letter-pair to two
// letter-pairs.  e.g. AB -> C turns into AC and CB.  Since the beginning and end are unchanged, we
// never create "surprise" letter-pairs that don't exist in either the input or the ruleset.
type Pair = [u8; 2];
type Rules = HashMap<Pair, [Pair; 2]>;
// represent the current polymer by simply the number of each letter-pair it contains.
type Count = HashMap<Pair, u64>;

fn substitute(rules: &Rules, input: Count) -> Count {
    let mut result = input.clone();

    for (pair, count) in input {
        if let Some(new) = rules.get(&pair) {
            // we no longer have the original pairs
            *result.get_mut(&pair).unwrap() -= count;

            // but we do have the new ones
            for i in new {
                *result.entry(*i).or_default() += count;
            }
        }
    }

    result
}

// count each element by the trailing-letter of the letter pair, *plus* the beginning character
// (i.e. AHA is {AH -> 1, HA -> 1}, so count one H, one A, plus the first A).
fn min_max_count_by_char(counts: &Count, first_element: u8) -> (u64, u64) {
    let mut element_counts: HashMap<u8, u64> = HashMap::new();

    for (pair, count) in counts.iter().chain([(&[0, first_element], &1)].into_iter()) {
        *element_counts.entry(pair[1]).or_default() += count;
    }

    let (min, max) = element_counts.values().minmax().into_option().unwrap();
    (*min, *max)
}

fn do_main(input: &str) {
    let mut input = read_lines_from_file(input);
    let template = input.next().unwrap();
    input.next();
    let mut rules = Rules::new();
    for line in input {
        let parts = line.split(" -> ").collect_vec();
        let from = parts[0].bytes().collect_vec().try_into().unwrap();
        let middle = parts[1].bytes().exactly_one().unwrap();

        rules.insert(from, [[from[0], middle], [middle, from[1]]]);
    }

    let mut pair_counts = Count::new();
    for (l, r) in template.bytes().tuple_windows() {
        *pair_counts.entry([l, r]).or_default() += 1;
    }

    for _round in 0..10 {
        pair_counts = substitute(&rules, pair_counts);
    }
    let (min, max) = min_max_count_by_char(&pair_counts, template.as_bytes()[0]);
    let part1 = max - min;
    dbg!(part1);

    for _round in 10..40 {
        pair_counts = substitute(&rules, pair_counts);
    }
    let (min, max) = min_max_count_by_char(&pair_counts, template.as_bytes()[0]);
    let part2 = max - min;
    dbg!(part2);
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
            ([b'C', b'H'], [[b'C', b'B'], [b'B', b'H']]),
            ([b'H', b'H'], [[b'H', b'N'], [b'N', b'H']]),
            ([b'C', b'B'], [[b'C', b'H'], [b'H', b'B']]),
            ([b'N', b'H'], [[b'N', b'C'], [b'C', b'H']]),
            ([b'H', b'B'], [[b'H', b'C'], [b'C', b'B']]),
            ([b'H', b'C'], [[b'H', b'B'], [b'B', b'C']]),
            ([b'H', b'N'], [[b'H', b'C'], [b'C', b'N']]),
            ([b'N', b'N'], [[b'N', b'C'], [b'C', b'N']]),
            ([b'B', b'H'], [[b'B', b'H'], [b'H', b'H']]),
            ([b'N', b'C'], [[b'N', b'B'], [b'B', b'C']]),
            ([b'N', b'B'], [[b'N', b'B'], [b'B', b'B']]),
            ([b'B', b'N'], [[b'B', b'B'], [b'B', b'N']]),
            ([b'B', b'B'], [[b'B', b'N'], [b'N', b'B']]),
            ([b'B', b'C'], [[b'B', b'B'], [b'B', b'C']]),
            ([b'C', b'C'], [[b'C', b'N'], [b'N', b'C']]),
            ([b'C', b'N'], [[b'C', b'C'], [b'C', b'N']]),
        ]
        .into();

        // NNCB
        let mut polymer = [([b'N', b'N'], 1), ([b'N', b'C'], 1), ([b'C', b'B'], 1)].into();
        polymer = substitute(&rules, polymer);
        // zeroes are left in as a benign implementation detail, but remove them before comparing
        // for testing
        polymer.retain(|_, &mut count| count != 0);
        assert_eq!(
            polymer,
            crate::Count::from([
                // NCNBCHB
                ([b'N', b'C'], 1),
                ([b'C', b'N'], 1),
                ([b'N', b'B'], 1),
                ([b'B', b'C'], 1),
                ([b'C', b'H'], 1),
                ([b'H', b'B'], 1),
            ])
        );
        polymer = substitute(&rules, polymer);
        polymer.retain(|_, &mut count| count != 0);
        debug_assert_eq!(
            polymer,
            crate::Count::from([
                // NBCCNBBBCBHCB, split into pairs and piped through sort|uniq -c
                ([b'B', b'B'], 2),
                ([b'B', b'C'], 2),
                ([b'B', b'H'], 1),
                ([b'C', b'B'], 2),
                ([b'C', b'C'], 1),
                ([b'C', b'N'], 1),
                ([b'H', b'C'], 1),
                ([b'N', b'B'], 2),
            ])
        );
    }
}
