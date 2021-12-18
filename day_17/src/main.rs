use prelude::*;

fn highest_reached(
    mut xvel: i64,
    mut yvel: i64,
    x1: i64,
    x2: i64,
    y1: i64,
    y2: i64,
) -> Option<i64> {
    let corners = [(x1, y1), (x1, y2), (x2, y1), (x2, y2)];
    let bottom_left = corners.iter().min().cloned().unwrap();
    let top_right = corners.iter().max().cloned().unwrap();

    let (mut x, mut y) = (0, 0);
    let mut highest_y = 0;

    loop {
        if (bottom_left.0..=top_right.0).contains(&x) && (bottom_left.1..=top_right.1).contains(&y)
        {
            return Some(highest_y);
        }

        // if we're descending below the block, we've already missed
        if yvel < 0 && y < bottom_left.1 {
            return None;
        }

        x += xvel;
        y += yvel;

        if xvel > 0 {
            xvel -= 1;
        } else if xvel < 0 {
            xvel += 1;
        }

        yvel -= 1;
    }
}

fn do_main(input: &str) {
    let format =
        Regex::new("target area: x=(-?[0-9]+)..(-?[0-9]+), y=(-?[0-9]+)..(-?[0-9]+)").unwrap();
    let input = read_lines_from_file(input).next().unwrap();
    let captures = format.captures(&input).unwrap();
    dbg!(&captures[0]);
    let x1 = captures[1].parse::<i64>().unwrap();
    let x2 = captures[2].parse::<i64>().unwrap();
    let y1 = captures[3].parse::<i64>().unwrap();
    let y2 = captures[4].parse::<i64>().unwrap();

    let mut part1 = 0;
    for xvel in -50..50 {
        for yvel in 0..1000 {
            if let Some(y) = highest_reached(xvel, yvel, x1, x2, y1, y2) {
                if y > part1 {
                    part1 = y
                }
            }
        }
    }
    dbg!(part1);
}

fn main() {
    do_main("inputs/day_17.txt");
}
