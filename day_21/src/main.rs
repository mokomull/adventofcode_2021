use prelude::*;

fn do_main(input: &str) {
    let mut spaces = read_lines_from_file(input)
        .map(|line| {
            let value = line.split(": ").nth(1).unwrap();
            // subtract one so that I can use simple modular arithmetic
            value.parse::<u32>().unwrap() - 1
        })
        .collect_vec();

    let mut die = 1;
    let mut roll = || -> u32 {
        let next = if die == 100 { 1 } else { die + 1 };
        std::mem::replace(&mut die, next)
    };

    let mut scores = vec![0; spaces.len()];
    let mut player = 0;
    let mut rolls = 0;

    let part1 = loop {
        spaces[player] += roll() + roll() + roll();
        spaces[player] %= 10;
        rolls += 3;

        scores[player] += spaces[player] + 1; // add that subtraction back in

        let next_player = (player + 1) % spaces.len();

        if scores[player] > 1000 {
            break scores[next_player] * rolls;
        }

        player = next_player;
    };
    dbg!(part1);
}

fn main() {
    do_main("inputs/day_21.txt");
}
