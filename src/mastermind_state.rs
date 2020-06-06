use std::fmt::{Display, Formatter, Error};
use crate::util::get_random_number_u8;
use crate::evaluation::Evaluation;
use crate::colors::Colors;

pub const NUM_ELEMENTS: usize = 4;
pub type Values = [Colors; NUM_ELEMENTS];

pub fn get_guess_from_string(buf: String) -> Values {
    let mut result = [Colors::Blue; NUM_ELEMENTS];
    let zero_char = '0' as u8;

    let mut result2 = buf.as_bytes().iter()
        .take(NUM_ELEMENTS)
        .filter(|c| **c >= zero_char && **c <= (Colors::len() - 1 + zero_char))
        .map(|c| Colors::from(c - zero_char))
        .collect::<Vec<Colors>>();
    while result2.len() < NUM_ELEMENTS {
        result2.push(Colors::Blue);
    }
    result.copy_from_slice(result2.as_slice());

    result
}

#[derive(Debug,Clone,Copy,PartialEq)]
pub struct MastermindState {
    values: Values,
    eval: Evaluation,
}

impl MastermindState {
    pub fn new_random_state() -> Self {
        let mut values: Values =
            [Colors::Black, Colors::Black, Colors::Black, Colors::Black];
        for val in values.iter_mut() {
            *val = Colors::from(get_random_number_u8(Colors::len()));
        }
        MastermindState::new(values, Evaluation::new(0, 0))
    }

    pub fn new(values: Values, eval: Evaluation) -> Self {
        MastermindState{values: values, eval: eval}
    }

    pub fn new_diff_state(&self, values: Values) -> MastermindState {
        let eval = MastermindState::diff(self, &values);
        MastermindState::new(values, eval)
    }

    pub fn are_values_equal(&self, rhs: &Values) -> bool {
        return self.values == *rhs
    }

    pub fn diff(&self, guess: &Values) -> Evaluation {
        let mut correct_matches: u8 = 0;
        let mut color_present: u8 = 0;
        let mut used_slots_truth: [bool; NUM_ELEMENTS]= [false; NUM_ELEMENTS];
        let mut used_slots_guess: [bool; NUM_ELEMENTS]= [false; NUM_ELEMENTS];

        // correct matches need to be done first
        for i in 0..guess.len() {
            let c = &guess[i];
            if *c == self.values[i] {
                correct_matches += 1;
                used_slots_truth[i] = true;
                used_slots_guess[i] = true;
            }
        }

        // check if at least correct color was picked
        for i in 0..guess.len() {
            if used_slots_guess[i] {
                continue;
            }

            let c = &guess[i];
            for j in 0..self.values.len() {
                if *c == self.values[j] && !used_slots_truth[j] {
                    color_present += 1;
                    used_slots_truth[j] = true;
                    used_slots_guess[i] = true;
                    break;
                }
            }
        }

        assert!((correct_matches + color_present) as usize <= self.values.len());
        Evaluation::new(correct_matches, color_present)
    }

    pub fn get_evaluation(&self) -> Evaluation { self.eval }
}

impl Display for MastermindState {
    fn fmt(&self, format: &mut Formatter) -> Result<(), Error> {
        for v in &self.values {
            write!(format, "{}", v)?;
        }
        write!(format, "  {}", self.eval)
    }
}

#[cfg(test)]
mod test {
    use crate::mastermind_state::{MastermindState, NUM_ELEMENTS};
    use crate::colors::Colors;
    use crate::evaluation::Evaluation;

    #[test]
    fn create_mastermind_state() {
        let state = MastermindState::new_random_state();
        println!("{}", state);
        //assert_eq!(1,2);
    }

    #[test]
    fn diff_with_solution() {
        let colors = [Colors::Black, Colors::Blue, Colors::Green, Colors::Red];
        let mms = MastermindState::new(colors, Evaluation::new(0, 0));
        let diff = mms.diff(&colors);
        assert_eq!(Evaluation::new(4, 0), diff);
    }

    #[test]
    fn diff_correct_colors() {
        let colors = [Colors::Black, Colors::Blue, Colors::Green, Colors::Red];
        let mms = MastermindState::new(colors, Evaluation::new(0, 0));
        let diff = mms.diff(&[Colors::Blue, Colors::Green, Colors::Red, Colors::Black]);
        assert_eq!(Evaluation::new(0, 4), diff);
    }

    #[test]
    fn diff_two_colors_correct() {
        let colors = [Colors::Black, Colors::Blue, Colors::Green, Colors::Red];
        let mms = MastermindState::new(colors, Evaluation::new(0, 0));
        let diff = mms.diff(&[Colors::Yellow, Colors::Yellow, Colors::Red, Colors::Black]);
        assert_eq!(Evaluation::new(0, 2), diff);
    }

    #[test]
    fn diff_no_color_correct() {
        let colors = [Colors::Black, Colors::Blue, Colors::Green, Colors::Red];
        let mms = MastermindState::new(colors, Evaluation::new(0, 0));
        let diff = mms.diff(&[Colors::Yellow, Colors::Yellow, Colors::Yellow, Colors::Yellow]);
        assert_eq!(Evaluation::new(0, 0), diff);
    }

    #[test]
    fn diff_duplicate_color0() {
        let colors = [Colors::Black, Colors::Black, Colors::Green, Colors::Red];
        let mms = MastermindState::new(colors, Evaluation::new(0, 0));
        let diff = mms.diff(&[Colors::Black, Colors::Yellow, Colors::Yellow, Colors::Yellow]);
        assert_eq!(Evaluation::new(1, 0), diff);
    }

    #[test]
    fn diff_duplicate_color1() {
        let colors = [Colors::Black, Colors::Black, Colors::Green, Colors::Red];
        let mms = MastermindState::new(colors, Evaluation::new(0, 0));
        let diff = mms.diff(&[Colors::Yellow, Colors::Black, Colors::Yellow, Colors::Yellow]);
        assert_eq!(Evaluation::new(1, 0), diff);
    }

    #[test]
    fn diff_duplicate_color2() {
        let colors = [Colors::Black, Colors::Black, Colors::Green, Colors::Red];
        let mms = MastermindState::new(colors, Evaluation::new(0, 0));
        let diff = mms.diff(&[Colors::Black, Colors::Black, Colors::Yellow, Colors::Yellow]);
        assert_eq!(Evaluation::new(2, 0), diff);
    }

    #[test]
    fn diff_duplicate_color3() {
        let colors = [Colors::Black, Colors::Black, Colors::Green, Colors::Red];
        let mms = MastermindState::new(colors, Evaluation::new(0, 0));
        let diff = mms.diff(&[Colors::Yellow, Colors::Yellow, Colors::Black, Colors::Black]);
        assert_eq!(Evaluation::new(0, 2), diff);
    }

    #[test]
    fn diff_only_one_color() {
        let colors = [Colors::Black; NUM_ELEMENTS];
        let mms = MastermindState::new(colors, Evaluation::new(0, 0));
        let diff = mms.diff(&[Colors::Yellow, Colors::Yellow, Colors::Black, Colors::Black]);
        assert_eq!(Evaluation::new(2, 0), diff);
    }

    #[test]
    fn diff_only_one_color_except_one() {
        let colors = [Colors::Black, Colors::Black, Colors::Black, Colors::Blue];
        let mms = MastermindState::new(colors, Evaluation::new(0, 0));
        let diff = mms.diff(&[Colors::Yellow, Colors::Yellow, Colors::Black, Colors::Black]);
        assert_eq!(Evaluation::new(1, 1), diff);
    }

    #[test]
    fn diff_only_one_color_except_one_with_color_fit() {
        let colors = [Colors::Black, Colors::Black, Colors::Black, Colors::Blue];
        let mms = MastermindState::new(colors, Evaluation::new(0, 0));
        let diff = mms.diff(&[Colors::Yellow, Colors::Yellow, Colors::Yellow, Colors::Black]);
        assert_eq!(Evaluation::new(0, 1), diff);
    }

    #[test]
    fn diff_only_one_color_except_one_bla() {
        let colors = [Colors::Black, Colors::Black, Colors::Black, Colors::Blue];
        let mms = MastermindState::new(colors, Evaluation::new(0, 0));
        let diff = mms.diff(&[Colors::Black, Colors::Yellow, Colors::Yellow, Colors::Black]);
        assert_eq!(Evaluation::new(1, 1), diff);
    }

    #[test]
    fn diff_two_colors() {
        let colors = [Colors::Black, Colors::Black, Colors::Blue, Colors::Blue];
        let mms = MastermindState::new(colors, Evaluation::new(0, 0));
        let diff = mms.diff(&[Colors::Blue, Colors::Blue, Colors::Black, Colors::Black]);
        assert_eq!(Evaluation::new(0, 4), diff);
    }
}
