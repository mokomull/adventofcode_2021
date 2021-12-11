use prelude::*;

fn do_single_step(energy: &mut Vec<Vec<u8>>) -> usize {
    let mut flashes = 0;

    energy
        .iter_mut()
        .for_each(|row| row.iter_mut().for_each(|octopus| *octopus += 1));

    while energy.iter().flatten().any(|&octopus| octopus > 9) {
        for i in 0..energy.len() {
            for j in 0..energy[0].len() {
                if energy[i][j] > 9 {
                    for m in i.saturating_sub(1)..=((i + 1).min(energy.len() - 1)) {
                        for n in j.saturating_sub(1)..=((j + 1).min(energy[0].len() - 1)) {
                            if energy[m][n] == 0 {
                                // this octopus has already flashed, and will be reset to 0.  It
                                // won't flash again, so ignore it.
                                continue;
                            }
                            energy[m][n] += 1;
                        }
                    }
                    energy[i][j] = 0;
                    flashes += 1;
                }
            }
        }
    }

    flashes
}

fn do_main(input: &str) {
    let mut input = read_lines_from_file(input)
        .map(|line| line.bytes().map(|b| b - b'0').collect_vec())
        .collect_vec();

    let mut part1 = 0;

    for _step in 0..100 {
        part1 += do_single_step(&mut input);
    }
    dbg!(part1);

    for step in 100.. {
        if input.iter().flatten().all(|&octopus| octopus == 0) {
            let part2 = step;
            dbg!(part2);
            break;
        }
    }
}

fn main() {
    do_main("inputs/day_11.txt");
}
