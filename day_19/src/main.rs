use prelude::*;

#[derive(Clone, Copy, Debug)]
enum Axis {
    X,
    Y,
    Z,
}
use Axis::*;

#[derive(Clone, Copy, Debug)]
enum Transform {
    Pos(Axis),
    Neg(Axis),
    RotateCCW(u8),
}
use Transform::*;

impl Transform {
    fn transform(&self, coords: [i32; 3]) -> [i32; 3] {
        match self {
            // Transform needed to get scanner's "facing" axis rotated to +X.
            Pos(axis) => match axis {
                X => coords,
                Y => [coords[1], -coords[0], coords[2]],
                Z => [coords[2], coords[1], -coords[0]],
            },
            Neg(axis) => match axis {
                X => [-coords[0], -coords[1], coords[2]],
                Y => [-coords[1], coords[0], coords[2]],
                Z => [-coords[2], coords[1], coords[0]],
            },
            RotateCCW(count) => {
                // rotate about the X axis (i.e. in the YZ plane) because I've defined "facing
                // positive X" to be the identity transform.
                let mut output = coords;
                for _ in 0..*count {
                    // multiply  [1   0  0       [x
                    //            0   0  1   by   y
                    //            0  -1  0]       z]
                    output = [output[0], output[2], -output[1]]
                }
                output
            }
        }
    }

    fn apply(transforms: &[Self], coords: [i32; 3]) -> [i32; 3] {
        transforms
            .iter()
            .fold(coords, |old, xfrm| xfrm.transform(old))
    }

    fn apply_offset(transforms: &[Self], coords: [i32; 3], offset: [i32; 3]) -> [i32; 3] {
        let transformed = Transform::apply(transforms, coords);
        [
            transformed[0] + offset[0],
            transformed[1] + offset[1],
            transformed[2] + offset[2],
        ]
    }
}

fn do_main(input: &str) {
    let mut scanners = Vec::new();
    let mut this_scanner = Vec::new();

    for line in read_lines_from_file(input).chain(std::iter::once(String::new())) {
        if line.starts_with("---") {
            this_scanner = Vec::new();
            continue;
        }

        if line.is_empty() {
            scanners.push(this_scanner);
            this_scanner = Vec::new(); // one of these is unnecessary, but doesn't hurt.
            continue;
        }

        let coords: [i32; 3] = line
            .split(',')
            .map(|c| c.parse().unwrap())
            .collect_vec()
            .try_into()
            .unwrap();
        this_scanner.push(coords);
    }

    // define scanner 0 to be centered at 0, 0, 0, and right-side-up.

    // Maps scanner id -> (previous scanner id, transform to get from previous to this one, offset
    // to get from previous to this one)
    let mut locations: HashMap<usize, (usize, Vec<Transform>, [i32; 3])> =
        [(0, (0, vec![], [0, 0, 0]))].into();

    'scanner: for permutation in scanners.iter().enumerate().permutations(2) {
        let (origin_id, origin) = permutation[0];
        let (this_id, this_scanner) = permutation[1];

        if locations.contains_key(&this_id) {
            continue;
        }

        // try every possible facing direction and "up" direction
        for orientation in [Pos(X), Pos(Y), Pos(Z), Neg(X), Neg(Y), Neg(Z)] {
            for rotations in [0, 1, 2, 3] {
                let transforms = vec![orientation, RotateCCW(rotations)];

                // now let's pick all pairs of points between the origin scanner and this one,
                // pretend they're the same beacon, and see if the offset makes sense (i.e. at least
                // 12 beacons translated from scanner-n coordinates to the scanner-0-relative
                // coordinate system match)
                for origin_point in origin {
                    for this_point in this_scanner {
                        let rotated = Transform::apply(&transforms, *this_point);
                        let offset = [
                            origin_point[0] - rotated[0],
                            origin_point[1] - rotated[1],
                            origin_point[2] - rotated[2],
                        ];
                        // if I've done the math right this will be a no-op, but make sure that
                        // applying transforms and then offset to this point really does get us the
                        // canonical coordinate system.
                        assert_eq!(
                            Transform::apply_offset(&transforms, *this_point, offset),
                            *origin_point
                        );

                        let matching_beacons = this_scanner
                            .iter()
                            .filter(|&beacon| {
                                origin.contains(&Transform::apply_offset(
                                    &transforms,
                                    *beacon,
                                    offset,
                                ))
                            })
                            .count();

                        if matching_beacons >= 12 {
                            locations.insert(this_id, (origin_id, transforms, offset));
                            continue 'scanner;
                        }
                    }
                }
            }
        }
    }
    // double-check that we did indeed find an orientation/offset for each scanner
    assert_eq!(locations.len(), scanners.len());

    let mut beacons = HashSet::new();
    for (id, scanner) in scanners.iter().enumerate() {
        for point in scanner {
            let mut translated = *point;
            let mut translate_id = id;
            // translate the point to the reference scanner, until we reach 0
            while translate_id != 0 {
                let (next_id, transform, offset) = locations.get(&translate_id).unwrap();
                translated = Transform::apply_offset(transform, translated, *offset);
            }
            beacons.insert(translated);
        }
    }

    let part1 = beacons.len();
    dbg!(part1);
}

fn main() {
    do_main("inputs/day_19.txt");
}
