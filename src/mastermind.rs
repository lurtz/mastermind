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

    pub fn guess(&mut self, values: Values) -> GuessStatus {
        self.guesses.push(self.initial.new_diff_state(values));
        let mmstate = self.guesses.last().unwrap();
        println!("{}", mmstate);
        let diff = self.guesses.last().unwrap().get_evaluation();
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
