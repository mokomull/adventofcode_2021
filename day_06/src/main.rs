use prelude::*;

fn do_main(input: &str) {
    let input = read_lines_from_file(input)
        .map(|line| {
            line.split(',')
                .map(|num| num.parse::<u64>().unwrap())
                .collect_vec()
        })
        .flatten()
        .collect_vec();

    let mut lanternfish = input.clone();
    for _round in 0..80 {
        for i in 0..lanternfish.len() {
            if lanternfish[i] == 0 {
                lanternfish[i] = 7;
                lanternfish.push(8);
            }
            lanternfish[i] -= 1;
        }
    }

    let part1 = lanternfish.len();
    dbg!(part1);
    assert_eq!(part1, 350917);

    let mut timers = [0; 9];
    for i in &input {
        timers[*i as usize] += 1;
    }

    for _round in 0..256 {
        let new_lanternfish = timers[0];

        for i in 1..timers.len() {
            timers[i - 1] = timers[i];
        }
        timers[6] += new_lanternfish;
        timers[8] = new_lanternfish;
    }

    let part2 = timers.iter().sum::<u64>();
    dbg!(part2);
    assert_eq!(part2, 1592918715629);
}

fn main() {
    do_main("inputs/day_06.txt");
}

#[cfg(test)]
mod test {
    #[test]
    fn main() {
        super::do_main("../inputs/day_06.txt");
    }
}
