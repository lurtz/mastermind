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

    fn reduce_colors(&mut self, values: &Values, eval: &Evaluation) {
        if 0 == eval.get_correct_match() {
            for i in 0..values.len() {
                self.colors[i].remove(&values[i]);
            }
        }
    }

    fn sort<'a>(values: &'a Values, eval: &'a Evaluation, old_values: &'a Values, old_eval: &'a Evaluation) -> (&'a Values, &'a Evaluation, &'a Values, &'a Evaluation) {
        if eval.get_correct_match() < old_eval.get_correct_match() {
            (old_values, old_eval, values, eval)
        } else {
            (values, eval, old_values, old_eval)
        }
    }

    fn reduce_colors_with_previous_state(&mut self, values: &Values, eval: &Evaluation, old_values: &Values, old_eval: &Evaluation) {
        self.reduce_colors(values, eval);
        // TODO reduce colors for two slots if matches went down by 2
        let (&better_values, &better_eval, &worse_values, &worse_eval) = PossibleColors::sort(values, eval, old_values, old_eval);
        if worse_eval.get_correct_match() + 2 == better_eval.get_correct_match() {
            for i in 0..values.len() {
                if worse_values[i] != better_values[i] {
                    self.colors[i].clear();
                    self.colors[i].insert(better_values[i]);
                }
            }
        }
        if worse_eval.get_correct_match() + 1 == better_eval.get_correct_match() {
            for i in 0..values.len() {
                if worse_values[i] != better_values[i] {
                    self.colors[i].remove(&worse_values[i]);
                }
            }
        }
        else if (better_eval.get_correct_match() == worse_eval.get_correct_match()) && 4 == self.get_num_colors() {
            for i in 0..values.len() {
                if better_values[i] != worse_values[i] {
                    self.colors[i].remove(&better_values[i]);
                    self.colors[i].remove(&worse_values[i]);
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

    fn get_num_colors(&self) -> usize {
        let mut used_colors = HashSet::<Colors>::new();
        for colors in self.colors.iter() {
            for c in colors {
                used_colors.insert(*c);
            }
        }
        used_colors.len()
    }
}

pub fn solve(mm: &mut Mastermind) -> Values {
    let mut result = solve_colors(mm);
    if are_all_colors_equal(&result) {
        return result;
    }
    let mut eval = Evaluation::new(0,0);
    let mut tried_patterns = HashSet::new();
    let mut possible_colors = PossibleColors::new(&result);
    let mut shift_loop = true;
    while shift_loop {
        match mm.guess(result) {
            GuessStatus::Success => return result,
            GuessStatus::Incorrect(e) => {
                eval = e;
                possible_colors.reduce_colors(&result, &eval);
                tried_patterns.insert(result);
                if !possible_colors.are_colors_ok(&result) {
                    result.rotate_right(1);
                } else {
                    shift_loop = false;
                }
            },
        }
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
