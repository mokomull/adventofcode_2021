use prelude::read_lines_from_file;

fn main() {
    let input = read_lines_from_file("inputs/day_04.txt").collect::<Vec<_>>();

    let numbers = &input[0];
    let numbers = numbers
        .split(',')
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let boards = input[1..]
        .chunks(6)
        .map(|raw| {
            raw.iter()
                .skip(1)
                .map(|row| {
                    row.split_ascii_whitespace()
                        .map(|num| num.parse::<u64>().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut winner = None;
    let mut count_numbers_called = None;

    'found: for count in 0..numbers.len() {
        for board in &boards {
            // check rows
            let winner_by_rows = board
                .iter()
                .any(|row| row.iter().all(|num| numbers[0..count].contains(num)));

            // check columns
            let winner_by_columns = (0..5).any(|col| {
                board
                    .iter()
                    .map(|row| row[col])
                    .all(|num| numbers[0..count].contains(&num))
            });

            if winner_by_rows || winner_by_columns {
                winner = Some(board);
                count_numbers_called = Some(count);
                break 'found;
            }
        }
    }

    let unmarked_sum = winner
        .unwrap()
        .iter()
        .flatten()
        .filter(|&num| !numbers[..count_numbers_called.unwrap()].contains(num))
        .sum::<u64>();
    let last_number_called = numbers[count_numbers_called.unwrap() - 1];

    let part1 = unmarked_sum * last_number_called;
    dbg!(part1);
}
