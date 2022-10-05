fn main() {
    use turing::TuringComplete;

    #[derive(Clone, PartialEq, Default)]
    enum Sigma {
        #[default]
        Zero,
        One,
    }

    use std::fmt;

    impl fmt::Debug for Sigma {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "{}",
                match self {
                    Self::One => "1",
                    Self::Zero => "0",
                }
            )
        }
    }

    use turing::Movement as M;
    use Sigma as S;

    let mut turing: turing::Turing<2, Sigma, u8> = turing::Turing::new(
        0,
        [
            ((0, S::Zero), (S::One, M::Right, 1)),
            ((1, S::Zero), (S::One, M::Left, 0)),
        ],
    );

    let mut string = String::new();

    loop {
        std::io::stdin().read_line(&mut string);
        turing.step();
        println!("{:?}", turing);
    }
}
