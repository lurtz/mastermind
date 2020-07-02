use std::fmt::{Display, Error, Formatter};
use std::slice::Iter;

use crate::util::{BLACK, BLUE, CHAR, GREEN, RED, RESET, WHITE, YELLOW};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Colors {
    Red,
    Green,
    Blue,
    Yellow,
    White,
    Black,
}

const COLORS: [Colors; 6] = [
    Colors::Red,
    Colors::Green,
    Colors::Blue,
    Colors::Yellow,
    Colors::White,
    Colors::Black,
];

impl Display for Colors {
    fn fmt(&self, format: &mut Formatter) -> Result<(), Error> {
        write!(format, "{}{}{}", self.to_shell_escape(), CHAR, RESET)
    }
}

impl From<u8> for Colors {
    fn from(num: u8) -> Self {
        *Colors::iter().nth(num as usize).unwrap_or(&Colors::last())
    }
}

impl Colors {
    fn to_shell_escape(&self) -> &str {
        match *self {
            Colors::Red => RED,
            Colors::Green => GREEN,
            Colors::Blue => BLUE,
            Colors::Yellow => YELLOW,
            Colors::White => WHITE,
            Colors::Black => BLACK,
        }
    }

    pub fn show_number_mapping() {
        for (pos, col) in Colors::iter().enumerate() {
            print!("{}{} ", pos, col);
        }
        println!();
    }

    pub fn iter() -> Iter<'static, Colors> {
        COLORS.iter()
    }

    pub fn last() -> Colors {
        *COLORS.last().unwrap()
    }

    pub fn len() -> u8 {
        COLORS.len() as u8
    }
}

#[cfg(test)]
mod test {
    use crate::colors::Colors;
    use crate::colors::COLORS;

    #[test]
    fn converting_number_to_color() {
        for (i, c) in Colors::iter().enumerate() {
            assert!(*c == Colors::from(i as u8));
        }
    }

    #[test]
    fn converting_out_of_bounds_number_produces_back() {
        assert!(Colors::last() == Colors::from(Colors::len() as u8));
    }

    #[test]
    fn last_color_is_last() {
        assert!(Colors::last() == *COLORS.last().unwrap());
    }

    #[test]
    fn display_displays_color() {
        assert_eq!("\u{1b}[30m▉\u{1b}[0m", format!("{}", Colors::Black));
        assert_eq!("\u{1b}[31m▉\u{1b}[0m", format!("{}", Colors::Red));
        assert_eq!("\u{1b}[32m▉\u{1b}[0m", format!("{}", Colors::Green));
        assert_eq!("\u{1b}[33m▉\u{1b}[0m", format!("{}", Colors::Yellow));
        assert_eq!("\u{1b}[34m▉\u{1b}[0m", format!("{}", Colors::Blue));
        assert_eq!("\u{1b}[37m▉\u{1b}[0m", format!("{}", Colors::White));
    }
}
