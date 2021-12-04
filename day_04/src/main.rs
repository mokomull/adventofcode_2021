use prelude::*;

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

    let mut first_winner = None;
    let mut last_winner = None;
    let mut has_won = HashSet::new();

    for count in 0..numbers.len() {
        for board in &boards {
            if has_won.contains(board) {
                continue;
            }

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
                if first_winner.is_none() {
                    first_winner = Some((board, count))
                }

                has_won.insert(board);
                last_winner = Some((board, count));
            }
        }
    }

    let (first_winning_board, first_winning_index) = first_winner.unwrap();
    let unmarked_sum = first_winning_board
        .iter()
        .flatten()
        .filter(|&num| !numbers[..first_winning_index].contains(num))
        .sum::<u64>();
    let last_number_called = numbers[first_winning_index - 1];

    let part1 = unmarked_sum * last_number_called;
    dbg!(part1);

    let (last_winning_board, last_winning_index) = last_winner.unwrap();
    let unmarked_sum = last_winning_board
        .iter()
        .flatten()
        .filter(|&num| !numbers[..last_winning_index].contains(num))
        .sum::<u64>();
    let last_number_called = numbers[last_winning_index - 1];

    let part2 = unmarked_sum * last_number_called;
    dbg!(part2);
}
