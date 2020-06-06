use crate::colors::Colors;
use crate::mastermind_state::{Values, NUM_ELEMENTS};
use crate::mastermind::{Mastermind, GuessStatus};

// solves mastermind in <= 24 turns
pub fn solve(mm: &mut Mastermind) -> Values {
    let mut guess: Values = [Colors::Red; NUM_ELEMENTS];
    let mut eval;
    match mm.guess(guess){
        GuessStatus::Success => return guess,
        GuessStatus::Incorrect(e) => eval = e,
    }

    for i in 0..guess.len() {
        let mut current_guess = guess;
        'colors_loop: for c in Colors::iterator() {
            current_guess[i] = *c;
            match mm.guess(current_guess){
                GuessStatus::Success => return current_guess,
                GuessStatus::Incorrect(e) => {
                    if e.get_correct_match() > eval.get_correct_match() {
                        eval = e;
                        guess = current_guess;
                        break 'colors_loop;
                    }
                    if e.get_correct_match() < eval.get_correct_match() {
                        break 'colors_loop;
                    }
                },
            }
        }
    }
    guess
}

#[cfg(test)]
mod test {
    use crate::Mastermind;
    use crate::solver::SolverFn;
    use crate::single_digit_solver::solve;

    #[test]
    fn solve_has_correct_type() {
        let _solvefn: SolverFn = solve;
    }

    #[test]
    fn solve2_solves_the_game() {
        let mut mm = Mastermind::new();
        let solution = solve(&mut mm);
        let pattern =  mm.get_initial();
        assert!(pattern.are_values_equal(&solution));
    }
}
