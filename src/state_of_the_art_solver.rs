use crate::colors::Colors;
use crate::evaluation::Evaluation;
use crate::mastermind::{GuessStatus, Mastermind};
use crate::mastermind_state::{MastermindState, Values};

struct AllStates {
    states: Vec<Values>,
}

impl AllStates {
    fn new() -> AllStates {
        let mut states = Vec::new();
        for c0 in Colors::iter() {
            for c1 in Colors::iter() {
                for c2 in Colors::iter() {
                    for c3 in Colors::iter() {
                        states.push([*c0, *c1, *c2, *c3]);
                    }
                }
            }
        }
        AllStates { states }
    }

    fn reduce(&mut self, values: &Values, eval: &Evaluation) -> Values {
        let state = MastermindState::new_initial(*values);
        self.states
            .retain(|possible_state| state.diff(possible_state) == *eval);
        self.new_pick()
    }

    fn new_pick(&self) -> Values {
        self.states[self.states.len() / 2]
    }
}

pub fn solve(mm: &mut Mastermind) -> Values {
    let mut states = AllStates::new();
    let mut colors: Values = [Colors::Red, Colors::Red, Colors::Green, Colors::Green];
    while let GuessStatus::Incorrect(e) = mm.guess(colors) {
        colors = states.reduce(&colors, &e);
    }
    colors
}

#[cfg(test)]
mod test {
    use crate::solver::test_utils::check_solution;
    use crate::solver::SolverFn;
    use crate::state_of_the_art_solver::solve;
    use crate::Mastermind;

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
