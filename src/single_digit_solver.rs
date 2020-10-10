use crate::colors::Colors;
use crate::mastermind::{GuessStatus, Mastermind};
use crate::mastermind_state::{Values, NUM_ELEMENTS};

// solves mastermind in <= 24 turns
pub fn solve(mm: &mut Mastermind) -> Values {
    let mut guess: Values = [Colors::Red; NUM_ELEMENTS];
    let mut eval;
    match mm.guess(guess) {
        GuessStatus::Success => return guess,
        GuessStatus::Incorrect(e) => eval = e,
    }

    'guess_loop: for i in 0..guess.len() {
        let mut current_guess = guess;
        'colors_loop: for c in Colors::iter().skip(1) {
            current_guess[i] = *c;
            match mm.guess(current_guess) {
                GuessStatus::Success => {
                    guess = current_guess;
                    break 'guess_loop;
                }
                GuessStatus::Incorrect(e) => {
                    if e.get_correct_match() > eval.get_correct_match() {
                        eval = e;
                        guess = current_guess;
                        break 'colors_loop;
                    }
                }
            }
        }
    }
    guess
}

#[cfg(test)]
mod test {
    use crate::single_digit_solver::solve;
    use crate::solver::SolverFn;
    use crate::Mastermind;
    use crate::solver::test_utils::check_solution;

    #[test]
    fn solve_has_correct_type() {
        let _solvefn: SolverFn = solve;
    }

    #[test]
    fn solve_solves_the_game() {
        let mut mm = Mastermind::new();
        let solution = solve(&mut mm);
        check_solution(&mm.get_initial().get_values(), &mm, &solution);
    }
}
