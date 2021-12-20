use prelude::*;

enum Image {
    DefaultDark(HashSet<(i32, i32)>),
    DefaultLight(HashSet<(i32, i32)>),
}
use Image::*;

fn enhance(image: &Image, algorithm: &[bool]) -> Image {
    let indices_in_image = match image {
        DefaultDark(lit) => lit,
        DefaultLight(dark) => dark,
    };
    let (min_row, max_row) = indices_in_image
        .iter()
        .map(|&(row, _)| row)
        .minmax()
        .into_option()
        .unwrap();
    let (min_col, max_col) = indices_in_image
        .iter()
        .map(|&(_, col)| col)
        .minmax()
        .into_option()
        .unwrap();
    // if 000_000_000 is lit, then the infinite field will become lit.  Let's store the list of
    // *dark* pixels in that case.
    let invert_result = algorithm[0] && !algorithm[511];
    let mut result = match image {
        DefaultDark(_) => {
            if invert_result {
                DefaultLight(HashSet::new())
            } else {
                DefaultDark(HashSet::new())
            }
        }
        DefaultLight(_) => {
            if invert_result {
                DefaultDark(HashSet::new())
            } else {
                DefaultLight(HashSet::new())
            }
        }
    };

    for row in (min_row - 3)..=(max_row + 3) {
        for col in (min_col - 3)..=(max_col + 3) {
            let index = (row - 1..=row + 1)
                .cartesian_product(col - 1..=col + 1)
                .map(|(i, j)| match image {
                    DefaultDark(lit) if lit.contains(&(i, j)) => 1,
                    DefaultDark(_) => 0,
                    DefaultLight(dark) if dark.contains(&(i, j)) => 0,
                    DefaultLight(_) => 1,
                })
                .reduce(|value, x| 2 * value + x)
                .unwrap();
            match &mut result {
                DefaultDark(lit) if algorithm[index] => {
                    lit.insert((row, col));
                }
                DefaultLight(dark) if !algorithm[index] => {
                    dark.insert((row, col));
                }
                _ => (),
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
    let mut image = DefaultDark(
        input
            .enumerate()
            .map(|(i, line)| {
                line.into_bytes().into_iter().enumerate().filter_map(
                    move |(j, pixel)| match pixel {
                        b'#' => Some((i as i32, j as i32)),
                        _ => None,
                    },
                )
            })
            .flatten()
            .collect::<HashSet<_>>(),
    );

    for _ in 0..2 {
        image = enhance(&image, &algorithm);
    }
    if let DefaultDark(lit) = &image {
        let part1 = lit.len();
        dbg!(part1);
    }

    for _ in 2..50 {
        image = enhance(&image, &algorithm);
    }
    if let DefaultDark(lit) = &image {
        let part2 = lit.len();
        dbg!(part2);
    }
}

fn main() {
    do_main("inputs/day_20.txt");
}
