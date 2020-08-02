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
mod test {
    use crate::colors::Colors;
    use crate::mastermind::Mastermind;
    use crate::mastermind_state::MastermindState;
    use crate::multi_digit_solver;
    use crate::single_digit_solver;
    use crate::solver::parse_args;
    use std::string::String;

    #[test]
    fn empty_args_results_in_manual_solver() {
        parse_args(vec![]);
    }

    #[test]
    fn no_args_results_in_manual_solver() {
        parse_args(vec![String::from("bla")]);
    }

    #[test]
    fn single_results_in_single_digit_solver() {
        parse_args(vec![String::from("bla"), String::from("single")]);
    }

    #[test]
    fn multi_results_in_multi_digit_solver() {
        parse_args(vec![String::from("bla"), String::from("multi")]);
    }

    #[test]
    fn unknown_string_results_in_manual_solver() {
        parse_args(vec![String::from("bla"), String::from("fdjafda")]);
    }

    macro_rules! solver_tests {($solvers:expr; $($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let values = $value;
                for solver in $solvers.iter() {
                    let mut mm = Mastermind::new_with_state(values);
                    let solution =solver(&mut mm);
                    let pattern = mm.get_initial();
                    assert!(pattern.are_values_equal(&solution));
                    assert!(MastermindState::new_initial(values).are_values_equal(&solution));
                }
            }
        )*
    }}

    solver_tests! {
        [multi_digit_solver::solve, single_digit_solver::solve];
        solve_with_red_state_solves_the_game_fast: [Colors::Red; 4],
        solve_with_green_state_solves_the_game_fast: [Colors::Green; 4],
        solve_with_white_state_solves_the_game_fast: [Colors::White; 4],
        solve_with_yellow_state_solves_the_game_fast: [Colors::Yellow; 4],
        solve_with_blue_state_solves_the_game_fast: [Colors::Blue; 4],
        solve_with_black_state_solves_the_game_slow: [Colors::Black; 4],
        solve_with_mixed_state_solves_the_game0: [Colors::Blue, Colors::White, Colors::Green, Colors::Yellow],
        solve_with_mixed_state_solves_the_game1: [Colors::Red, Colors::White, Colors::Black, Colors::Yellow],
        solve_with_mixed_state_solves_the_game2: [Colors::Red, Colors::Red, Colors::Black, Colors::Black],
        solve_with_mixed_state_solves_the_game3: [Colors::Red, Colors::Black, Colors::Black, Colors::Red],
        solve_with_mixed_state_solves_the_game4: [Colors::White, Colors::Blue, Colors::Blue, Colors::White],
        solve_with_mixed_state_solves_the_game5: [Colors::Green, Colors::Yellow, Colors::Green, Colors::Yellow],
    }
}
