use crate::colors::Colors;
use crate::mastermind_state::{Values, NUM_ELEMENTS};
use crate::mastermind::{Mastermind, GuessStatus};

fn solve_colors(mm: &mut Mastermind) -> Values {
    let mut colors: Values = [Colors::Blue; NUM_ELEMENTS];
    let mut colors_iter = colors.iter_mut();

    for c in Colors::iterator().take_while(|x| **x != Colors::last()) {
        let guess: Values = [*c; NUM_ELEMENTS];
        let status = mm.guess(guess);
        match status {
            GuessStatus::Success => {
                return [*c; NUM_ELEMENTS];
            },
            GuessStatus::Incorrect(s) => {
                for _ in 0..(s.get_correct_match() + s.get_color_present()) {
                    *colors_iter.next().unwrap() = *c;
                }
            },
        }
    }
    for i in colors_iter {
        *i = Colors::last();
    }
    colors
}

fn are_all_colors_equal(values: Values) -> bool {
    if values.len() == 0 {
        return true;
    }
    let first = values[0];
    return values.iter().all(|x| *x == first);
}

fn switch(values: &mut Values, a: usize, b: usize) {
    let tmp = values[a];
    values[a] = values[b];
    values[b] = tmp;
}

pub fn solve(mm: &mut Mastermind) -> Values {
    let mut result = solve_colors(mm);
    if are_all_colors_equal(result) {
        return result;
    }
    let mut eval;
    match mm.guess(result) {
        GuessStatus::Success => return result,
        GuessStatus::Incorrect(e) => eval = e,
    }
    for i in 0..result.len() {
        'second_pos: for j in 0..result.len() {
            let mut current_guess = result;
            switch(&mut current_guess, i, j);
            match mm.guess(current_guess) {
                GuessStatus::Success => return current_guess,
                GuessStatus::Incorrect(e) => {
                    if eval.get_correct_match() < e.get_correct_match() {
                        result = current_guess;
                        eval = e;
                        break 'second_pos;
                    }
                },
            }
        }
    }
    result
}

#[cfg(test)]
mod test {
    use crate::Mastermind;
    use crate::multi_digit_solver::solve;
    use crate::solver::SolverFn;

    #[test]
    fn solve_has_correct_type() {
        let _solvefn: SolverFn = solve;
    }

    #[test]
    fn solve_solves_the_game() {
        let mut mm = Mastermind::new();
        let solution = solve(&mut mm);
        let pattern =  mm.get_initial();
        assert!(pattern.are_values_equal(&solution));
    }
}
