use std::fmt::{Display, Formatter, Error};
use crate::util::{CHAR, RESET, BLACK, WHITE};

#[derive(Debug,Clone,Copy,PartialEq)]
pub struct Evaluation {
    correct_match: u8,
    color_present: u8,
}

impl Evaluation {
    pub fn new(correct_matches: u8, color_present: u8) -> Self {
        Evaluation{correct_match: correct_matches, color_present: color_present}
    }

    pub fn get_correct_match(&self) -> u8 { self.correct_match }
    pub fn get_color_present(&self) -> u8 { self.color_present }
}

impl Display for Evaluation {
    fn fmt(&self, format: &mut Formatter) -> Result<(), Error> {
        let mut correct_dots = String::new();
        for _ in 0..self.get_correct_match() {
            correct_dots.push_str(CHAR);
        }
        let mut present_dots = String::new();
        for _ in 0..self.get_color_present() {
            present_dots.push_str(CHAR);
        }
        write!(format, "{}{}{}{}{}", BLACK, correct_dots, WHITE, present_dots, RESET)
    }
}
