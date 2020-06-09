use std::fmt::{Display, Formatter, Error};
use std::slice::Iter;

use crate::util::{CHAR, RESET, BLUE, BLACK, GREEN, WHITE, YELLOW, RED};

#[derive(Debug,Clone,Copy,PartialEq)]
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
    Colors::Black];

impl Display for Colors {
    fn fmt(&self, format: &mut Formatter) -> Result<(), Error> {
        write!(format, "{}{}{}", self.to_shell_escape(), CHAR, RESET)
    }
}

impl From<u8> for Colors {
    fn from(num: u8) -> Self {
        *Colors::iterator().nth(num as usize).unwrap_or(&Colors::Black)
    }
}

impl Colors {
    fn to_shell_escape(&self) -> &str {
        match self {
            &Colors::Red => RED,
            &Colors::Green => GREEN,
            &Colors::Blue => BLUE,
            &Colors::Yellow => YELLOW,
            &Colors::White => WHITE,
            &Colors::Black => BLACK,
        }
    }

    pub fn show_number_mapping() {
        for (pos, col) in Colors::iterator().enumerate() {
            print!("{}{} ", pos, col);
        }
        println!();
    }

    pub fn iterator() -> Iter<'static, Colors> {
        COLORS.iter()
    }

    pub fn last() -> Colors {
        *COLORS.last().unwrap()
    }

    pub fn len() -> u8 { COLORS.len() as u8 }
}
