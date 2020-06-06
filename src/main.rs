mod util;
mod colors;
mod evaluation;
mod mastermind_state;
mod mastermind;
mod single_digit_solver;
mod multi_digit_solver;
mod solver;
mod manual_solver;

use colors::Colors;
use mastermind::Mastermind;
use std::env;

enum Modus {
    Manual,
    Single,
    Multi
}

fn parse_args() -> Modus {
    let args: Vec<String> = env::args().collect();
    println!("args = {:?}", args);

    let modus;
    if 2 > args.len() {
        modus = Modus::Manual;
    }
    else if "single" == args[1] {
        modus = Modus::Single;
    }
    else if "multi" == args[1] {
        modus = Modus::Multi;
    } else {
        modus = Modus::Manual;
    }
    modus
}

fn main() {
    let modus = parse_args();
    Colors::show_number_mapping();
    let mut mm = Mastermind::new();

    let solution = match modus {
        Modus::Manual => manual_solver::solve(&mut mm),
        Modus::Single => single_digit_solver::solve(&mut mm),
        Modus::Multi => multi_digit_solver::solve(&mut mm),
    };

    if mm.get_initial().are_values_equal(&solution) {
        println!("Game solved in {} steps", mm.get_guesses().len());
    }
}
