mod util;
mod colors;
mod evaluation;
mod mastermind_state;
mod mastermind;
mod single_digit_solver;
mod multi_digit_solver;
mod solver;
mod manual_solver;

use mastermind::Mastermind;
use std::env;
use solver::parse_args;

fn main() {
    let mut mm = Mastermind::new();

    let solver = parse_args(env::args().collect());
    let solution = solver(&mut mm);

    if mm.get_initial().are_values_equal(&solution) {
        println!("Game solved in {} steps", mm.get_guesses().len());
    }
}
