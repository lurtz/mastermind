use crate::colors::Colors;
use crate::mastermind::{GuessStatus, Mastermind};
use crate::mastermind_state::{get_guess_from_string, Values};
use crate::util::CURSOR_UP;
use std::io::stdin;

fn get_guess() -> Result<Values, std::io::Error> {
    let mut buf = String::new();
    stdin().read_line(&mut buf)?;
    Ok(get_guess_from_string(buf))
}

pub fn solve(mm: &mut Mastermind) -> Values {
    Colors::show_number_mapping();
    let mut guess = get_guess().unwrap();
    let mut solved = false;
    while !solved {
        print!("{}", CURSOR_UP);
        let status = mm.guess(guess);
        if GuessStatus::Success == status {
            solved = true;
        } else {
            guess = get_guess().unwrap();
        }
    }
    guess
}

#[cfg(test)]
mod test {
    use crate::manual_solver::solve;
    use crate::solver::SolverFn;

    #[test]
    fn solve_has_correct_type() {
        let _solvefn: SolverFn = solve;
    }
}
