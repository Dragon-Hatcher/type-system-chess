#![allow(clippy::type_complexity)]
#![recursion_limit = "256"]

mod board_rep;
mod move_gen;
mod state;
#[cfg(test)]
mod test;
mod ui;
mod util;
mod values;

fn main() {
    use ui::prelude::*;

    // NOTE: When the move chain gets long it can take a very long time to compile.
    // Sometimes on my computer the compiler runs out of memory and crashes.
    // If that happens you may need to spell out an intermediate state instead
    // of computing it. See the test directory for how to do that.

    // (MakeEM stands for MakeEasyMove since you only specify the start and end sq.)

    // On my computer it takes ~2min to compile the following program.
    // Running `cargo test` can take over 9min.

    // Fool's mate
    type S1 = StartState;
    type S2 = MakeEM<S1, F2, F3>;
    type S3 = MakeEM<S2, E7, E6>;
    type S4 = MakeEM<S3, G2, G4>;
    type S5 = MakeEM<S4, D8, H4>;
    println!("{}", S5::reify()); // You can print the other states if you like.

    // Promotion (may crash the compiler)
    // type S1 = StartState;
    // type S2 = MakeEM<S1, F2, F4>;
    // type S3 = MakeEM<S2, A7, A6>;
    // type S4 = MakeEM<S3, F4, F5>;
    // type S5 = MakeEM<S4, A6, A5>;
    // type S6 = MakeEM<S5, F5, F6>;
    // type S7 = MakeEM<S6, A5, A6>;
    // type S8 = MakeEM<S7, F6, F7>;
    // type S9 = MakeEM<S8, A4, A3>;
    // type S10 = MakeEM<S9, G7, H8, Queen>;
    // println!("{}", S10::reify());
}
