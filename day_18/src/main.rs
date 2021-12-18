#[derive(Clone, Debug, Eq, PartialEq)]
enum Snailfish {
    Normal(u64),
    Pair(Box<Snailfish>, Box<Snailfish>),
}

impl Snailfish {
    fn exploded(&mut self) -> bool {
        unimplemented!()
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
