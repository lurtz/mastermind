use crate::evaluation::Evaluation;
use crate::mastermind_state::{MastermindState, Values, NUM_ELEMENTS};
use std::fmt::{Display, Error, Formatter};
use std::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GuessStatus {
    Success,
    Incorrect(Evaluation),
}

impl Display for GuessStatus {
    fn fmt(&self, format: &mut Formatter) -> Result<(), Error> {
        write!(format, "{:?}", self)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Mastermind {
    initial: MastermindState,
    guesses: Vec<MastermindState>,
}

impl Mastermind {
    pub fn new() -> Self {
        Mastermind {
            initial: MastermindState::new_random_state(),
            guesses: Vec::<MastermindState>::new(),
        }
    }

    pub fn new_with_state(values: Values) -> Self {
        Mastermind {
            initial: MastermindState::new_initial(values),
            guesses: Vec::<MastermindState>::new(),
        }
    }

    pub fn guess(&mut self, values: Values) -> GuessStatus {
        let mmstate = self.initial.new_diff_state(values);
        self.guesses.push(mmstate);
        println!("{}", mmstate);
        let diff = mmstate.get_evaluation();
        if diff.get_correct_match() as usize == NUM_ELEMENTS {
            GuessStatus::Success
        } else {
            GuessStatus::Incorrect(diff)
        }
    }

    pub fn get_initial(&self) -> MastermindState {
        self.initial
    }

    pub fn get_guesses(&self) -> Vec<MastermindState> {
        self.guesses.clone()
    }
}

impl Display for Mastermind {
    fn fmt(&self, format: &mut Formatter) -> Result<(), Error> {
        write!(format, "{}", self.initial)
    }
}

#[cfg(test)]
mod test {
    use crate::evaluation::Evaluation;
    use crate::mastermind::GuessStatus;
    use crate::mastermind::Mastermind;
    use crate::mastermind_state::MastermindState;

    #[test]
    fn guess_status_display() {
        assert_eq!("Success", format!("{}", GuessStatus::Success));
        assert_eq!(
            "Incorrect(Evaluation { correct_match: 2, color_present: 1 })",
            format!("{}", GuessStatus::Incorrect(Evaluation::new(2, 1)))
        );
    }

    #[test]
    fn new() {
        let mm = Mastermind::new();
        assert_eq!(0, mm.get_guesses().len());
        let initial = mm.get_initial();
        assert_eq!(0, initial.get_evaluation().get_color_present());
        assert_eq!(0, initial.get_evaluation().get_correct_match());
    }

    #[test]
    fn display() {
        let buffer = format!("{}", Mastermind::new());
        assert_eq!(64, buffer.len());
    }

    #[test]
    fn guess_with_success() {
        let mut mm = Mastermind::new();
        assert_eq!(
            GuessStatus::Success,
            mm.guess(mm.get_initial().get_values())
        );
    }

    #[test]
    fn guess_with_incorrect() {
        let mut mm = Mastermind::new();
        let mut state = MastermindState::new_random_state();
        while state == mm.get_initial() {
            state = MastermindState::new_random_state();
        }
        let status = mm.guess(state.get_values());
        let diff = mm.get_initial().new_diff_state(state.get_values());
        assert_eq!(GuessStatus::Incorrect(diff.get_evaluation()), status);
    }
}
