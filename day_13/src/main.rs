use prelude::*;

#[derive(Clone, Copy)]
enum Fold {
    X(i64),
    Y(i64),
}
use Fold::*;

fn fold(points: &mut HashSet<(i64, i64)>, fold: Fold) {
    let mut to_add = Vec::new();

    for point in points.iter() {
        match fold {
            X(x) => {
                if point.0 > x {
                    // x - (point - x)
                    to_add.push((2 * x - point.0, point.1));
                }
            }
            Y(y) => {
                if point.1 > y {
                    to_add.push((point.0, 2 * y - point.1));
                }
            }
        }
    }

    for point in to_add {
        points.insert(point);
    }

    match fold {
        X(x) => points.retain(|&point| point.0 < x),
        Y(y) => points.retain(|&point| point.1 < y),
    }
}

fn do_main(input: &str) {
    let mut input = read_lines_from_file(input);
    let mut points = HashSet::new();
    let mut folds = Vec::new();
    for line in &mut input {
        if line == "" {
            break;
        }
        let coords = line
            .split(',')
            .map(|s| s.parse::<i64>().unwrap())
            .collect_vec();
        points.insert((coords[0], coords[1]));
    }
    for line in &mut input {
        let pieces = line.split('=').collect_vec();
        let value = pieces[1].parse().unwrap();
        let fold = match pieces[0].bytes().last().unwrap() {
            b'x' => X(value),
            b'y' => Y(value),
            x => panic!("unknown axis {}", x),
        };
        folds.push(fold);
    }

    fold(&mut points, folds[0]);
    let part1 = points.len();
    dbg!(part1);

    for f in &folds[1..] {
        fold(&mut points, *f);
    }
    for y in 0..=points.iter().map(|&(_, y)| y).max().unwrap() {
        for x in 0..=points.iter().map(|&(x, _)| x).max().unwrap() {
            if points.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".")
            }
        }
        println!();
    }
}

fn main() {
    do_main("inputs/day_13.txt");
}
