use nom::{branch::alt, bytes::complete::tag, character::complete::digit1, IResult};

#[derive(Clone, Debug, Eq, PartialEq)]
enum Snailfish {
    Normal(u64),
    Pair(Box<Snailfish>, Box<Snailfish>),
}

impl Snailfish {
    fn exploded(&mut self) -> bool {
        let mut left = None;
        let mut add_to_right = None;

        // Visit the nodes in-order, keeping track of the previously seen regular number, because
        // that is what the left element explodes into.
        let mut to_visit = vec![(self, 0)];
        while !to_visit.is_empty() {
            let (this, depth) = to_visit.pop().unwrap();

            match this {
                // if add_to_right has been set, then we have found an exploding snailfish number.
                // after we update `this`, we're completely done searching.
                Snailfish::Normal(ref mut inner) if add_to_right.is_some() => {
                    *inner += add_to_right.unwrap();
                    break;
                }

                Snailfish::Normal(_) => {
                    left = Some(this);
                }

                // explode a pair that is beyond depth 4, but if an explosion has already occurred,
                // we simply want to fall through into the next case to visit the next regular
                // number.
                Snailfish::Pair(a, b) if depth >= 4 && add_to_right.is_none() => {
                    // the flavortext says that exploding pairs will only have regular numbers as
                    // children.
                    if let Snailfish::Normal(a) = **a {
                        if let Snailfish::Normal(b) = **b {
                            if let Some(&mut Snailfish::Normal(ref mut left)) = left {
                                *left += a;
                            }
                            add_to_right = Some(b);
                            *this = Snailfish::Normal(0);
                            continue;
                        }
                    }
                    panic!(
                        "Found a pair of not-regular numbers that would explode: {:?}",
                        this
                    );
                }

                Snailfish::Pair(ref mut a, ref mut b) => {
                    to_visit.push((b, depth + 1));
                    to_visit.push((a, depth + 1));
                }
            }
        }

        // if and only if add_to_right was set, we experienced an explosion.  Use that as our
        // sentinel.
        add_to_right.is_some()
    }

    fn split(&mut self) -> bool {
        match self {
            Snailfish::Normal(value) if *value >= 10 => {
                *self = Snailfish::Pair(
                    Box::new(Snailfish::Normal(*value / 2)),
                    Box::new(Snailfish::Normal((*value + 1) / 2)),
                );
                true
            }

            Snailfish::Normal(_) => false,

            // take advantage of short-circuiting to split only the first thing that splits
            Snailfish::Pair(a, b) => a.split() || b.split(),
        }
    }

    fn reduce(&mut self) {
        while self.exploded() || self.split() {}
    }

    fn magnitude(&self) -> u64 {
        match self {
            Snailfish::Normal(value) => *value,
            Snailfish::Pair(lhs, rhs) => 3 * lhs.magnitude() + 2 * rhs.magnitude(),
        }
    }
}

impl std::ops::Add for Snailfish {
    type Output = Snailfish;

    fn add(mut self, mut rhs: Snailfish) -> Snailfish {
        self.reduce();
        rhs.reduce();

        let mut result = Snailfish::Pair(Box::new(self), Box::new(rhs));
        result.reduce();
        result
    }
}

fn parse_snailfish(input: &[u8]) -> IResult<&[u8], Snailfish> {
    fn pair(input: &[u8]) -> IResult<&[u8], Snailfish> {
        let (input, _) = tag(b"[")(input)?;
        let (input, left) = parse_snailfish(input)?;
        let (input, _) = tag(b",")(input)?;
        let (input, right) = parse_snailfish(input)?;
        let (input, _) = tag(b"]")(input)?;
        Ok((input, Snailfish::Pair(Box::new(left), Box::new(right))))
    }
    fn integer(input: &[u8]) -> IResult<&[u8], Snailfish> {
        let (input, value) = digit1(input)?;
        Ok((
            input,
            Snailfish::Normal(std::str::from_utf8(value).unwrap().parse().unwrap()),
        ))
    }

    alt((pair, integer))(input)
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn explode() {
        use super::Snailfish::Normal as N;
        use super::Snailfish::Pair as P;
        let bx = Box::<Snailfish>::new;

        let mut testcase = P(
            bx(P(
                bx(P(bx(P(bx(P(bx(N(9)), bx(N(8)))), bx(N(1)))), bx(N(2)))),
                bx(N(3)),
            )),
            bx(N(4)),
        );

        assert!(testcase.exploded());
        assert_eq!(
            testcase,
            P(
                bx(P(bx(P(bx(P(bx(N(0)), bx(N(9)))), bx(N(2)))), bx(N(3)))),
                bx(N(4))
            )
        );

        fn test_explode_parsed(input: &[u8], output: &[u8]) {
            let mut testcase = parse_snailfish(input).unwrap().1;
            assert!(testcase.exploded());
            assert_eq!(testcase, parse_snailfish(output).unwrap().1);
        }

        test_explode_parsed(b"[7,[6,[5,[4,[3,2]]]]]", b"[7,[6,[5,[7,0]]]]");
        test_explode_parsed(b"[[6,[5,[4,[3,2]]]],1]", b"[[6,[5,[7,0]]],3]");
        test_explode_parsed(
            b"[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",
            b"[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
        );
        test_explode_parsed(
            b"[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
            b"[[3,[2,[8,0]]],[9,[5,[7,0]]]]",
        );

        // from manually evaluating [[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]] +
        // [7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
        test_explode_parsed(
            b"[[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]],[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]]",
            b"[[[[4,0],[5,0]],[[[4,5],[2,6]],[9,5]]],[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]]",
        );
        test_explode_parsed(
            b"[[[[4,0],[5,0]],[[[4,5],[2,6]],[9,5]]],[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]]",
            b"[[[[4,0],[5,4]],[[0,[7,6]],[9,5]]],[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]]",
        );
    }

    #[test]
    fn split() {
        fn test_split_parsed(input: &[u8], output: &[u8]) {
            let mut testcase = parse_snailfish(input).unwrap().1;
            assert!(testcase.split());
            assert_eq!(testcase, parse_snailfish(output).unwrap().1);
        }

        test_split_parsed(
            b"[[[[0,7],4],[15,[0,13]]],[1,1]]",
            b"[[[[0,7],4],[[7,8],[0,13]]],[1,1]]",
        );
        test_split_parsed(
            b"[[[[0,7],4],[[7,8],[0,13]]],[1,1]]",
            b"[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]",
        );
    }

    #[test]
    fn reduce() {
        fn test_reduce_parsed(input: &[u8], output: &[u8]) {
            let mut testcase = parse_snailfish(input).unwrap().1;
            testcase.reduce();
            assert_eq!(testcase, parse_snailfish(output).unwrap().1);
        }

        test_reduce_parsed(
            b"[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]",
            b"[[[[0,7],4],[[7,8],[6,0]]],[8,1]]",
        );
    }

    #[test]
    fn add() {
        fn test_add(lhs: &[u8], rhs: &[u8], output: &[u8]) {
            let lhs = parse_snailfish(lhs).unwrap().1;
            let rhs = parse_snailfish(rhs).unwrap().1;
            assert_eq!(lhs + rhs, parse_snailfish(output).unwrap().1);
        }

        test_add(
            b"[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]",
            b"[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]",
            b"[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]",
        );
        test_add(
            b"[[[[6,6],[7,7]],[[0,7],[7,7]]],[[[5,5],[5,6]],9]]",
            b"[1,[[[9,3],9],[[9,0],[0,7]]]]",
            b"[[[[7,8],[6,7]],[[6,8],[0,8]]],[[[7,7],[5,0]],[[5,5],[5,6]]]]",
        );
    }

    #[test]
    fn magnitude() {
        fn test_magnitude(input: &[u8], output: u64) {
            assert_eq!(parse_snailfish(input).unwrap().1.magnitude(), output);
        }

        test_magnitude(b"[[1,2],[[3,4],5]]", 143);
        test_magnitude(b"[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", 1384);
        test_magnitude(b"[[[[1,1],[2,2]],[3,3]],[4,4]]", 445);
        test_magnitude(b"[[[[3,0],[5,3]],[4,4]],[5,5]]", 791);
        test_magnitude(b"[[[[5,0],[7,4]],[5,5]],[6,6]]", 1137);
        test_magnitude(
            b"[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
            3488,
        );
    }
}
