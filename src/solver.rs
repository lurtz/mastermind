use crate::mastermind::Mastermind;
use crate::mastermind_state::Values;

use crate::single_digit_solver;
use crate::multi_digit_solver;
use crate::manual_solver;

pub type SolverFn = fn(&mut Mastermind) -> Values;

pub fn parse_args(args: Vec<String>) -> SolverFn {
    let solver: SolverFn;
    if 2 > args.len() {
        solver = manual_solver::solve;
    }
    else if "single" == args[1] {
        solver = single_digit_solver::solve;
    }
    else if "multi" == args[1] {
        solver = multi_digit_solver::solve;
    } else {
        solver = manual_solver::solve;
    }
    solver
}

