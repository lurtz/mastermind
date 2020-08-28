use crate::colors::Colors;
use crate::mastermind::{GuessStatus, Mastermind};
use crate::mastermind_state::{get_guess_from_string, Values};
use crate::util::CURSOR_UP;
use std::io::stdin;

type InputFn = fn() -> Result<Values, std::io::Error>;

fn solve_with_input(mm: &mut Mastermind, input: InputFn) -> Values {
    Colors::show_number_mapping();
    let mut guess = input().unwrap();
    let mut solved = false;
    while !solved {
        print!("{}", CURSOR_UP);
        let status = mm.guess(guess);
        if GuessStatus::Success == status {
            solved = true;
        } else {
            guess = input().unwrap();
        }
    }
    guess
}

fn get_guess() -> Result<Values, std::io::Error> {
    let mut buf = String::new();
    stdin().read_line(&mut buf)?;
    Ok(get_guess_from_string(buf))
}

pub fn solve(mm: &mut Mastermind) -> Values {
    solve_with_input(mm, get_guess)
}

#[cfg(test)]
mod test {
    use crate::colors::Colors;
    use crate::manual_solver::solve;
    use crate::manual_solver::solve_with_input;
    use crate::mastermind::Mastermind;
    use crate::mastermind_state::{get_guess_from_string, MastermindState, Values};
    use crate::solver::SolverFn;
    use std::io::{Error, ErrorKind};

    #[test]
    fn solve_has_correct_type() {
        let _solvefn: SolverFn = solve;
    }

    #[test]
    fn solve_with_correct_guess() {
        let return_black = || -> Result<Values, Error> {
            Ok(get_guess_from_string(String::from("5555")))
        };
        let values = [Colors::Black; 4];
        let mut mm = Mastermind::new_with_state(values);
        let solution = solve_with_input(&mut mm, return_black);
        let pattern = mm.get_initial();
        assert!(pattern.are_values_equal(&solution));
        assert!(MastermindState::new_initial(values).are_values_equal(&solution));
    }

    fn get_blue_and_black_guess() -> Result<Values, Error> {
        static mut NUM_QUERIES: u8 = 0;
        unsafe {
            NUM_QUERIES += 1;
            if 1 == NUM_QUERIES {
                Ok(get_guess_from_string(String::from("4444")))
            } else {
                Ok(get_guess_from_string(String::from("5555")))
            }
        }
    }

    #[test]
    fn solve_with_incorrect_and_correct_guess() {
        let values = [Colors::Black; 4];
        let mut mm = Mastermind::new_with_state(values);
        let solution = solve_with_input(&mut mm, get_blue_and_black_guess);
        let pattern = mm.get_initial();
        assert!(pattern.are_values_equal(&solution));
        assert!(MastermindState::new_initial(values).are_values_equal(&solution));
    }

    #[test]
    #[should_panic]
    fn solve_with_erroring_input_panics() {
        let return_black = || -> Result<Values, Error> {
            Err(Error::from(ErrorKind::InvalidData))
        };
        let values = [Colors::Black; 4];
        let mut mm = Mastermind::new_with_state(values);
        let _ = solve_with_input(&mut mm, return_black);
    }

}
