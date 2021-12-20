use prelude::*;

fn enhance(image: &HashSet<(i32, i32)>, algorithm: &[bool]) -> HashSet<(i32, i32)> {
    let (min_row, max_row) = image
        .iter()
        .map(|&(row, _)| row)
        .minmax()
        .into_option()
        .unwrap();
    let (min_col, max_col) = image
        .iter()
        .map(|&(_, col)| col)
        .minmax()
        .into_option()
        .unwrap();
    let mut result = HashSet::new();

    for row in (min_row - 3)..=(max_row + 3) {
        for col in (min_col - 3)..=(max_col + 3) {
            let index = (row - 1..=row + 1)
                .cartesian_product(col - 1..=col + 1)
                .map(|(i, j)| if image.contains(&(i, j)) { 1 } else { 0 })
                .reduce(|value, x| 2 * value + x)
                .unwrap();
            if algorithm[index] {
                result.insert((row, col));
            }
        }
    }
    result
}

fn do_main(input: &str) {
    let mut input = read_lines_from_file(input);
    let algorithm = input
        .next()
        .unwrap()
        .bytes()
        .map(|b| b == b'#')
        .collect_vec();
    input.next(); // skip the empty line
    let mut image = input
        .enumerate()
        .map(|(i, line)| {
            line.into_bytes()
                .into_iter()
                .enumerate()
                .filter_map(move |(j, pixel)| match pixel {
                    b'#' => Some((i as i32, j as i32)),
                    _ => None,
                })
        })
        .flatten()
        .collect::<HashSet<_>>();

    for _ in 0..2 {
        image = enhance(&image, &algorithm);
    }
    let part1 = image.len();
    dbg!(part1);
}

fn main() {
    do_main("inputs/day_20.txt");
}
