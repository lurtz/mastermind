use crate::manual_solver;
use crate::mastermind::Mastermind;
use crate::mastermind_state::Values;
use crate::multi_digit_solver;
use crate::single_digit_solver;

pub type SolverFn = fn(&mut Mastermind) -> Values;

pub fn parse_args(args: Vec<String>) -> SolverFn {
    let solver: SolverFn;
    if 2 > args.len() {
        solver = manual_solver::solve;
    } else if "single" == args[1] {
        solver = single_digit_solver::solve;
    } else if "multi" == args[1] {
        solver = multi_digit_solver::solve;
    } else {
        solver = manual_solver::solve;
    }
    solver
}


#[cfg(test)]
pub mod test_utils {
    use crate::mastermind::Mastermind;
    use crate::mastermind_state::{MastermindState, Values};

    pub fn check_solution(values: &Values, mm: &Mastermind, solution: &Values) {
        let pattern = mm.get_initial();
        assert!(pattern.are_values_equal(&solution));
        assert!(MastermindState::new_initial(*values).are_values_equal(&solution));
    }
}

#[cfg(test)]
mod test {
    use crate::colors::Colors;
    use crate::mastermind::Mastermind;
    use crate::{multi_digit_solver, single_digit_solver, manual_solver};
    use crate::solver::{parse_args, SolverFn};
    use std::string::String;
    use crate::solver::test_utils::check_solution;

    #[test]
    fn empty_args_results_in_manual_solver() {
        parse_args(vec![]);
    }

    fn parse_args_tests(args: Vec<String>, func: SolverFn) {
        assert!(parse_args(args) as *const SolverFn == func as *const SolverFn);
    }

    #[test]
    fn no_args_results_in_manual_solver() {
        parse_args_tests(vec![String::from("bla")], manual_solver::solve);
    }

    #[test]
    fn single_results_in_single_digit_solver() {
        parse_args_tests(vec![String::from("bla"), String::from("single")],
                single_digit_solver::solve);
    }

    #[test]
    fn multi_results_in_multi_digit_solver() {
        parse_args_tests(vec![String::from("bla"), String::from("multi")],
                multi_digit_solver::solve);
    }

    #[test]
    fn unknown_string_results_in_manual_solver() {
        parse_args_tests(vec![String::from("bla"), String::from("fdjafda")],
                manual_solver::solve);
    }

    macro_rules! solver_tests {($solvers:expr; $($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let values = $value;
                for name_solver in $solvers.iter() {
                    let (solver_name, solver) = name_solver;
                    let mut mm = Mastermind::new_with_state(values);
                    let solution = solver(&mut mm);
                    check_solution(&values, &mm, &solution);
                    println!("{} solved in {} steps", solver_name, mm.get_guesses().len());
                }
            }
        )*
    }}

    solver_tests! {
        [("single_digit_solver", single_digit_solver::solve as SolverFn), ("multi_digit_solver", multi_digit_solver::solve as SolverFn)];
        solve_with_red_state_solves_the_game: [Colors::Red; 4],
        solve_with_green_state_solves_the_game: [Colors::Green; 4],
        solve_with_white_state_solves_the_game: [Colors::White; 4],
        solve_with_yellow_state_solves_the_game: [Colors::Yellow; 4],
        solve_with_blue_state_solves_the_game: [Colors::Blue; 4],
        solve_with_black_state_solves_the_game: [Colors::Black; 4],
        solve_with_mixed_state_solves_the_game0: [Colors::Blue, Colors::White, Colors::Green, Colors::Yellow],
        solve_with_mixed_state_solves_the_game1: [Colors::Red, Colors::White, Colors::Black, Colors::Yellow],
        solve_with_mixed_state_solves_the_game2: [Colors::Red, Colors::Red, Colors::Black, Colors::Black],
        solve_with_mixed_state_solves_the_game3: [Colors::Red, Colors::Black, Colors::Black, Colors::Red],
        solve_with_mixed_state_solves_the_game4: [Colors::White, Colors::Blue, Colors::Blue, Colors::White],
        solve_with_mixed_state_solves_the_game5: [Colors::Green, Colors::Yellow, Colors::Green, Colors::Yellow],
        solve_with_mixed_state_solves_the_game6: [Colors::Black, Colors::White, Colors::Yellow, Colors::Blue],
        solve_with_mixed_state_solves_the_game7: [Colors::White, Colors::Yellow, Colors::Blue, Colors::Black],
        solve_with_mixed_state_solves_the_game8: [Colors::White, Colors::Blue, Colors::Yellow, Colors::Black],
        solve_with_mixed_state_solves_the_game9: [Colors::Black, Colors::Blue, Colors::Yellow, Colors::White],
    }
}
