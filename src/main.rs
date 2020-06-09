mod colors;
mod evaluation;
mod manual_solver;
mod mastermind;
mod mastermind_state;
mod multi_digit_solver;
mod single_digit_solver;
mod solver;
mod util;

use mastermind::Mastermind;
use solver::parse_args;
use std::env;

fn main() {
    let mut mm = Mastermind::new();

    let solver = parse_args(env::args().collect());
    let solution = solver(&mut mm);

    if mm.get_initial().are_values_equal(&solution) {
        println!("Game solved in {} steps", mm.get_guesses().len());
    }
}
