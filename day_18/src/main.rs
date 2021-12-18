use std::collections::VecDeque;

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
        let mut to_visit: VecDeque<_> = [(self, 0)].into();
        while !to_visit.is_empty() {
            let (this, depth) = to_visit.pop_front().unwrap();

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

                Snailfish::Pair(a, b) if depth >= 4 => {
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
                    to_visit.push_back((a, depth + 1));
                    to_visit.push_back((b, depth + 1));
                }
            }
        }

        // if and only if add_to_right was set, we experienced an explosion.  Use that as our
        // sentinel.
        add_to_right.is_some()
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use crate::Snailfish;

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
    }
}
