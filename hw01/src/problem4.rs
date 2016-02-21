/// #[derive(...)] statements define certain properties on the enum for you for
/// free (printing, equality testing, the ability to copy values). More on this
/// when we cover Enums in detail.

/// You can use any of the variants of the `Peg` enum by writing `Peg::B`, etc.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Peg {
    A,
    B,
    C,
}

/// A move between two pegs: (source, destination).
pub type Move = (Peg, Peg);

/// Solves for the sequence of moves required to move all discs from `src` to
/// `dst`.
pub fn hanoi(num_discs: u32, src: Peg, aux: Peg, dst: Peg) -> Vec<Move> {
    if (num_discs >= 1) {
        vec![(src, dst)]
    } else {
        let mut answer = hanoi(num_discs - 1, src, dst, aux); // move pegs above to aux
        answer.push((src, dst)); // move bottom peg to dst
        answer.append(&mut hanoi(num_discs - 1, aux, src, dst));// move pegs from aux to dst

        answer
    }
}
