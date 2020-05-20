use std::convert::From;
use std::fs::File;
use std::io::{Read, stdin};
use std::fmt::{Display, Formatter, Error};
use std::vec::Vec;

const BLACK: &str = "\x1B[30m";
const RED: &str = "\x1B[31m";
const GREEN: &str = "\x1B[32m";
const YELLOW: &str = "\x1B[33m";
const BLUE: &str = "\x1B[34m";
const WHITE: &str = "\x1B[37m";
const RESET: &str = "\x1B[0m";
// moves one line up
const CURSOR_UP: &str = "\x1B[1A";

const CHAR: &str = "▉";

fn get_random_number() -> u64 {
    let mut f = File::open("/dev/urandom").unwrap();
    let mut buf = [0u8; 8];
    f.read_exact(&mut buf).unwrap();
    let mut num: u64 = 0;
    for i in 0..7 {
        num |= u64::from(buf[i]) << 8*(7 - i);
    }
    num
}

fn get_random_number_u8(max: u8) -> u8 {
    (get_random_number() % u64::from(max)) as u8
}

#[derive(Debug,Clone,Copy,PartialEq)]
enum Colors {
    Red,
    Green,
    Blue,
    Yellow,
    White,
    Black,
}

const NUM_COLORS: u8 = 6;

impl Display for Colors {
    fn fmt(&self, format: &mut Formatter) -> Result<(), Error> {
        let color = match self {
            &Colors::Red => RED,
            &Colors::Green => GREEN,
            &Colors::Blue => BLUE,
            &Colors::Yellow => YELLOW,
            &Colors::White => WHITE,
            &Colors::Black => BLACK,
        };
        write!(format, "{}{}{}", color, CHAR, RESET)
    }
}

impl From<u8> for Colors {
    fn from(num: u8) -> Self {
        match num {
            0 => return Colors::Red,
            1 => return Colors::Green,
            2 => return Colors::Blue,
            3 => return Colors::Yellow,
            4 => return Colors::White,
            _ => return Colors::Black,
        }
    }
}

impl Colors {
    fn show_number_mapping() {
        for i in 0..NUM_COLORS {
            print!("{}{} ", Colors::from(i), i);
        }
        println!();
    }
}

#[derive(Debug,Clone,Copy,PartialEq)]
struct Evaluation {
    correct_match: u8,
    color_present: u8,
}

impl Evaluation {
    fn new(correct_matches: u8, color_present: u8) -> Self {
        Evaluation{correct_match: correct_matches, color_present: color_present}
    }

    fn get_correct_match(&self) -> u8 { self.correct_match }
    fn get_color_present(&self) -> u8 { self.color_present }
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

const NUM_ELEMENTS: usize = 4;
type Values = [Colors; NUM_ELEMENTS];

#[derive(Debug,Clone,Copy,PartialEq)]
struct MastermindState {
    values: Values,
    eval: Evaluation,
}

impl MastermindState {
    fn new_random_state() -> Self {
        let mut values: Values =
            [Colors::Black, Colors::Black, Colors::Black, Colors::Black];
        for val in values.iter_mut() {
            *val = Colors::from(get_random_number_u8(NUM_COLORS + 1));
        }
        MastermindState::new(values, Evaluation::new(0, 0))
    }

    fn new(values: Values, eval: Evaluation) -> Self {
        MastermindState{values: values, eval: eval}
    }

    fn new_diff_state(&self, values: Values) -> MastermindState {
        let eval = MastermindState::diff(self, &values);
        MastermindState::new(values, eval)
    }

    fn diff(&self, guess: &Values) -> Evaluation {
        let mut correct_matches: u8 = 0;
        let mut color_present: u8 = 0;
        let mut used_slots_truth: [bool; NUM_ELEMENTS]= [false, false, false, false];
        let mut used_slots_guess: [bool; NUM_ELEMENTS]= [false, false, false, false];

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

    fn get_evaluation(&self) -> Evaluation { self.eval }
}

impl Display for MastermindState {
    fn fmt(&self, format: &mut Formatter) -> Result<(), Error> {
        for v in &self.values {
            write!(format, "{}", v)?;
        }
        write!(format, "  {}", self.eval)
    }
}

#[derive(Debug,Clone,Copy,PartialEq)]
enum GuessStatus {
    Success,
    Incorrect(Evaluation),
}

impl Display for GuessStatus {
    fn fmt(&self, format: &mut Formatter) -> Result<(), Error> {
        write!(format, "{:?}", self)
    }
}

#[derive(Debug,Clone,PartialEq)]
struct Mastermind {
    initial: MastermindState,
    guesses: Vec<MastermindState>,
}

impl Mastermind {
    fn new() -> Self {
        Mastermind{
            initial: MastermindState::new_random_state(),
            guesses: Vec::<MastermindState>::new()}
    }

    fn guess(&mut self, values: Values) -> GuessStatus {
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
}

impl Display for Mastermind {
    fn fmt(&self, format: &mut Formatter) -> Result<(), Error> {
        write!(format, "{}", self.initial)
    }
}

fn get_guess() -> Result<Values, std::io::Error> {
    let mut buf = String::new();
    stdin().read_line(&mut buf)?;
    let mut result = [Colors::Blue, Colors::Black, Colors::Green, Colors::Yellow];
    let mut i = 0;
    for c in buf.as_bytes() {
        if *c >= ('0' as u8) && *c <= (NUM_COLORS - 1 + '0' as u8) && i < NUM_ELEMENTS{
            result[i] = Colors::from(*c - ('0' as u8));
            i += 1;
        }
    }
    Ok(result)
}

fn main() {
    Colors::show_number_mapping();
    let mut mm = Mastermind::new();
    println!("{}", mm);

    let mut solved = false;
    while !solved {
        let guess: Values;
        match get_guess() {
            Ok(g) => guess = g,
            Err(e) => {println!("input error: {}", e); return},
        };
        print!("{}", CURSOR_UP);
        let status = mm.guess(guess);
        if GuessStatus::Success == status {
            solved = true;
            println!("{}", status);
        }
    }
}

#[cfg(test)]
mod test {
    use crate::get_random_number_u8;
    use crate::MastermindState;
    use crate::Colors;
    use crate::Evaluation;

    #[test]
    fn random_number_generator_u8_with_valid_limits() {
        for upper_limit in 1..20 {
            let x = get_random_number_u8(upper_limit);
            assert!(x < upper_limit);
        }
    }

    #[test]
    #[should_panic]
    fn random_number_generator_u8_with_invalid_limit() {
        get_random_number_u8(0);
    }

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
        let colors = [Colors::Black, Colors::Black, Colors::Black, Colors::Black];
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
