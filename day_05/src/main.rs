use prelude::*;

fn do_main(input: &str) {
    let input = read_lines_from_file(input)
        .map(|line| {
            let mut coords = line.split(" -> ");
            let from: Vec<u64> = coords
                .next()
                .unwrap()
                .split(',')
                .map(|s| s.parse().unwrap())
                .collect();
            let to: Vec<u64> = coords
                .next()
                .unwrap()
                .split(',')
                .map(|s| s.parse().unwrap())
                .collect();

            ((from[0], from[1]), (to[0], to[1]))
        })
        .collect_vec();

    let mut map: HashMap<(u64, u64), usize> = HashMap::new();
    for ((x1, y1), (x2, y2)) in &input {
        if x1 == x2 {
            for y in *y1.min(y2)..=*y1.max(y2) {
                *map.entry((*x1, y)).or_default() += 1;
            }
        }

        if y1 == y2 {
            for x in *x1.min(x2)..=*x1.max(x2) {
                *map.entry((x, *y1)).or_default() += 1;
            }
        }
    }

    let part1 = map.values().filter(|&&v| v > 1).count();
    dbg!(part1);

    // now add only the 45-degree lines
    for ((x1, y1), (x2, y2)) in &input {
        if x1 != x2 && y1 != y2 {
            let (leftx, lefty) = (x1, y1).min((x2, y2));
            let (rightx, righty) = (x1, y1).max((x2, y2));

            let xs = *leftx..=*rightx;

            let ys: Box<dyn Iterator<Item = u64>> = if righty > lefty {
                Box::new(*lefty..=*righty)
            } else {
                Box::new((*righty..=*lefty).rev())
            };

            for (x, y) in xs.zip(ys) {
                *map.entry((x, y)).or_default() += 1;
            }
        }
    }

    let part2 = map.values().filter(|&&v| v > 1).count();
    dbg!(part2);
}

fn main() {
    do_main("inputs/day_05.txt")
}
