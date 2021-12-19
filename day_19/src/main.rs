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
    Flip(Axis),
    RotateCCW(u8),
}
use Transform::*;

impl Transform {
    fn transform(&self, coords: [i32; 3]) -> [i32; 3] {
        match self {
            Flip(axis) => {
                let mut output = coords;
                match axis {
                    X => output[0] *= -1,
                    Y => output[1] *= -1,
                    Z => output[2] *= -1,
                }
                output
            }
            RotateCCW(count) => {
                // rotate about the Z axis (i.e. in the XY plane)
                let mut output = coords;
                for _ in 0..*count {
                    // multiply [x       [0  -1  0
                    //           y   by   1   0  0
                    //           z]       0   0  1]
                    output = [output[1], -output[0], output[2]];
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
    let mut locations = vec![(vec![], [0, 0, 0])];

    'scanner: for scanner in &scanners[1..] {
        // try every possible axis flip
        for flips in [Flip(X), Flip(Y), Flip(Z)].into_iter().powerset() {
            for rotations in [0, 1, 2, 3] {
                let mut transforms = flips.clone();
                transforms.push(RotateCCW(rotations));
                let transforms = transforms;

                // now let's pick all pairs of points between scanner 0 and this one, pretend
                // they're the same beacon, and see if the offset makes sense (i.e. at least 12
                // beacons translated from scanner-n coordinates to the scanner-0-relative
                // coordinate system match)
                for canonical in &scanners[0] {
                    for this in scanner {
                        let rotated = Transform::apply(&transforms, *this);
                        let offset = [
                            canonical[0] - rotated[0],
                            canonical[1] - rotated[1],
                            canonical[2] - rotated[2],
                        ];
                        // if I've done the math right this will be a no-op, but make sure that
                        // applying transforms and then offset to this point really does get us the
                        // canonical coordinate system.
                        assert_eq!(
                            Transform::apply_offset(&transforms, *this, offset),
                            *canonical
                        );

                        let matching_beacons = scanner
                            .iter()
                            .filter(|&beacon| {
                                scanners[0].contains(&Transform::apply_offset(
                                    &transforms,
                                    *beacon,
                                    offset,
                                ))
                            })
                            .count();

                        if matching_beacons >= 12 {
                            locations.push((transforms, offset));
                            continue 'scanner;
                        }
                    }
                }
            }
        }
    }
    // double-check that we did indeed find an orientation/offset for each scanner
    assert_eq!(locations.len(), scanners.len());
}

fn main() {
    do_main("inputs/day_19.txt");
}
