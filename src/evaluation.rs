use crate::util::{BLACK, DOTS, RESET, WHITE};
use std::fmt::{Display, Error, Formatter};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Evaluation {
    correct_match: u8,
    color_present: u8,
}

impl Evaluation {
    pub fn new(correct_match: u8, color_present: u8) -> Self {
        Evaluation {
            correct_match,
            color_present,
        }
    }

    pub fn get_correct_match(&self) -> u8 {
        self.correct_match
    }
    pub fn get_color_present(&self) -> u8 {
        self.color_present
    }
}

impl Display for Evaluation {
    fn fmt(&self, format: &mut Formatter) -> Result<(), Error> {
        // each square seems to consume 3 bytes
        let correct_dots = &DOTS[0..3 * (self.get_correct_match() as usize)];
        let present_dots = &DOTS[0..3 * (self.get_color_present() as usize)];
        write!(
            format,
            "{}{}{}{}{}",
            BLACK, correct_dots, WHITE, present_dots, RESET
        )
    }
}

#[cfg(test)]
mod test {
    use crate::evaluation::Evaluation;

    #[test]
    fn new() {
        let x = 4;
        let y = 5;
        let eval = Evaluation::new(x, y);
        assert_eq!(x, eval.get_correct_match());
        assert_eq!(y, eval.get_color_present());
    }

    #[test]
    fn display() {
        assert_eq!(
            "\u{1b}[30m▉▉▉\u{1b}[37m▉▉▉▉\u{1b}[0m",
            format!("{}", Evaluation::new(3, 4))
        );
    }

    #[test]
    #[should_panic]
    fn display_out_of_bounds() {
        format!("{}", Evaluation::new(5, 4));
    }
}
