use crate::colors::Colors;
use crate::evaluation::Evaluation;
use crate::mastermind::{GuessStatus, Mastermind};
use crate::mastermind_state::{Values, NUM_ELEMENTS};
use std::collections::HashSet;

fn solve_colors(mm: &mut Mastermind) -> Values {
    let mut colors: Values = [Colors::Blue; NUM_ELEMENTS];
    let mut colors_iter = colors.iter_mut().peekable();

    for c in Colors::iter().take_while(|x| **x != Colors::last()) {
        let guess: Values = [*c; NUM_ELEMENTS];
        let status = mm.guess(guess);
        match status {
            GuessStatus::Success => {
                return [*c; NUM_ELEMENTS];
            }
            GuessStatus::Incorrect(s) => {
                for _ in 0..(s.get_correct_match() + s.get_color_present()) {
                    *colors_iter.next().unwrap() = *c;
                }
                // quit early if all colors have been found
                if colors_iter.peek().is_none() {
                    return colors;
                }
            }
        }
    }
    for i in colors_iter {
        *i = Colors::last();
    }
    colors
}

fn are_all_colors_equal(values: &Values) -> bool {
    assert!(!values.is_empty());
    let first = values[0];
    values.iter().all(|x| *x == first)
}

struct PossibleColors {
    colors: [HashSet<Colors>; NUM_ELEMENTS],
}

impl PossibleColors {
    fn new(values: &Values) -> PossibleColors {
        let mut colors = HashSet::<Colors>::new();
        for v in values {
            colors.insert(*v);
        }
        let mut result: [HashSet<Colors>; NUM_ELEMENTS] = Default::default();
        for r in result.iter_mut() {
            *r = colors.clone();
        }
        PossibleColors{colors: result}
    }

    fn new_with_eval(values: &Values, eval: &Evaluation) -> PossibleColors {
        let mut pc = PossibleColors::new(values);
        pc.reduce_colors(values, eval);
        pc
    }

    fn reduce_colors(&mut self, values: &Values, eval: &Evaluation) {
        if 0 == eval.get_correct_match() {
            for i in 0..values.len() {
                self.colors[i].remove(&values[i]);
            }
        }
    }

    fn reduce_colors_with_previous_state(&mut self, values: &Values, eval: &Evaluation, old_values: &Values, old_eval: &Evaluation) {
        self.reduce_colors(values, eval);
        if eval.get_correct_match() < old_eval.get_correct_match() {
            for i in 0..values.len() {
                if values[i] != old_values[i] {
                    self.colors[i].remove(&values[i]);
                }
            }
        }
    }

    fn are_colors_ok(&self, values: &Values) -> bool {
        let mut result = true;
        for i in 0..values.len() {
            result &= self.colors[i].contains(&values[i]);
        }
        result
    }
}

pub fn solve(mm: &mut Mastermind) -> Values {
    let mut result = solve_colors(mm);
    if are_all_colors_equal(&result) {
        return result;
    }
    let mut eval;
    match mm.guess(result) {
        GuessStatus::Success => return result,
        GuessStatus::Incorrect(e) => eval = e,
    }
    let mut tried_patterns = HashSet::new();
    tried_patterns.insert(result);
    let mut possible_colors = PossibleColors::new_with_eval(&result, &eval);
    if !possible_colors.are_colors_ok(&result) {
        result.rotate_right(1);
    }
    for i in 0..result.len() {
        'second_pos: for j in 0..result.len() {
            let mut current_guess = result;
            if current_guess[i] == current_guess[j] {
                continue 'second_pos;
            }
            current_guess.swap(i, j);
            if !tried_patterns.insert(current_guess) {
                continue 'second_pos;
            }
            if !possible_colors.are_colors_ok(&current_guess) {
                continue 'second_pos;
            }
            match mm.guess(current_guess) {
                GuessStatus::Success => return current_guess,
                GuessStatus::Incorrect(e) => {
                    possible_colors.reduce_colors_with_previous_state(&current_guess, &e, &result, &eval);
                    if eval.get_correct_match() < e.get_correct_match() {
                        result = current_guess;
                        eval = e;
                        break 'second_pos;
                    }
                }
            }
        }
    }
    result
}

#[cfg(test)]
mod test {
    use crate::multi_digit_solver::solve;
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
