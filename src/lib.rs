#[derive(Clone)]
pub enum Movement {
    Left,
    Stop,
    Right,
}

impl Movement {
    fn unsigned_even_odd(self, index: usize) -> Option<usize> {
        Some(if index == 0 {
            // Position zero
            match self {
                Movement::Left => 1,
                Movement::Right => 2,
                _ => return None,
            }
        } else if index % 2 == 0 {
            // Even position == positive
            match self {
                Movement::Left => index - 2,
                Movement::Right => index + 2,
                _ => return None,
            }
        } else if index == 1 {
            // Position one
            match self {
                Movement::Left => 3,
                Movement::Right => 0,
                _ => return None,
            }
        } else {
            // Uneven position not one or zero == negative position
            match self {
                Movement::Left => index + 2,
                Movement::Right => index - 2,
                _ => return None,
            }
        })
    }
}

pub trait TuringComplete {
    type Alphabet: Clone + PartialEq + Default;
    type State: Clone + PartialEq;

    fn delta(
        &self,
        state: Self::State,
        character: Self::Alphabet,
    ) -> Option<(Self::Alphabet, Movement, Self::State)>;

    fn step(&mut self);
}

pub struct Turing<
    const INST: usize,
    Alphabet: Clone + PartialEq + Default,
    State: Clone + PartialEq,
> {
    head: usize,
    // `[0, -1, 1, -2, 3, -3, 4, -4, …]`
    mem: Vec<Alphabet>,
    state: State,
    quint: [((State, Alphabet), (Alphabet, Movement, State)); INST],
}

impl<const INST: usize, Alphabet: Clone + PartialEq + Default, State: Clone + PartialEq>
    Turing<INST, Alphabet, State>
{
    pub fn new(
        state: State,
        quint: [((State, Alphabet), (Alphabet, Movement, State)); INST],
    ) -> Self {
        Self {
            head: 0,
            mem: vec![Alphabet::default()],
            state,
            quint,
        }
    }

    fn movement(&self, movement: Movement) -> Option<usize> {
        movement.unsigned_even_odd(self.head)
    }
}

use std::fmt;

impl<
        const INST: usize,
        Alphabet: Clone + PartialEq + Default + fmt::Display,
        State: Clone + PartialEq + fmt::Display,
    > Turing<INST, Alphabet, State>
{
    fn fmt_state_mem(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}| ", self.state)?;
        for i in (1..self.mem.len()).filter(|i| i % 2 == 1).rev() {
            write!(f, "{} ", self.mem[i])?;
        }
        for i in (0..self.mem.len()).filter(|i| i % 2 == 0) {
            write!(f, "{} ", self.mem[i])?;
        }
        writeln!(f)
    }

    fn fmt_index(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "-| ")?;
        for i in (1..self.mem.len())
            .filter(|i| i % 2 == 1)
            .rev()
            .chain((0..self.mem.len()).filter(|i| i % 2 == 0))
        {
            if i == self.head {
                write!(f, "^ ")?;
            } else if i == 0 {
                write!(f, "⁰ ")?;
            } else {
                write!(f, "  ")?;
            }
        }

        Ok(())
    }
}

impl<
        const INST: usize,
        Alphabet: Clone + PartialEq + Default + fmt::Display,
        State: Clone + PartialEq + fmt::Display,
    > fmt::Display for Turing<INST, Alphabet, State>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_state_mem(f)?;
        self.fmt_index(f)
    }
}

impl<const INST: usize, Alphabet: Clone + PartialEq + Default, State: Clone + PartialEq>
    TuringComplete for Turing<INST, Alphabet, State>
{
    type Alphabet = Alphabet;
    type State = State;

    fn delta(
        &self,
        state: Self::State,
        character: Self::Alphabet,
    ) -> Option<(Self::Alphabet, Movement, Self::State)> {
        for inst in &self.quint {
            if inst.0 .0 == state && inst.0 .1 == character {
                return Some(inst.1.clone());
            }
        }

        None
    }

    fn step(&mut self) {
        if let Some((character, movement, state)) =
            self.delta(self.state.clone(), self.mem[self.head].clone())
        {
            self.mem[self.head] = character;
            if let Some(head) = self.movement(movement) {
                self.head = head;

                if self.head >= self.mem.len() {
                    for i in 0..(1 + self.head - self.mem.len()) {
                        self.mem.push(Alphabet::default())
                    }
                }
            }
            self.state = state;
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn movement() {
        use super::Movement as Mov;

        fn movs(start: usize, len: usize, dir: Mov, expected: Vec<usize>) {
            assert_eq!(
                expected,
                (0..len)
                    .scan(start, |index, _| {
                        *index = dir.clone().unsigned_even_odd(index.clone()).unwrap();
                        Some(index.clone())
                    })
                    .collect::<Vec<usize>>()
            )
        }

        movs(0, 4, Mov::Right, vec![2, 4, 6, 8]);
        movs(0, 4, Mov::Left, vec![1, 3, 5, 7]);
        movs(4, 4, Mov::Left, vec![2, 0, 1, 3]);
    }
}
