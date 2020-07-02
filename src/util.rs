use std::fs::File;
use std::io::Read;

pub const BLACK: &str = "\x1B[30m";
pub const RED: &str = "\x1B[31m";
pub const GREEN: &str = "\x1B[32m";
pub const YELLOW: &str = "\x1B[33m";
pub const BLUE: &str = "\x1B[34m";
pub const WHITE: &str = "\x1B[37m";
pub const RESET: &str = "\x1B[0m";
// moves one line up
pub const CURSOR_UP: &str = "\x1B[1A";

pub const DOTS: &str = "▉▉▉▉";
pub const CHAR: &str = "▉";

fn get_random_number() -> u64 {
    let mut f = File::open("/dev/urandom").unwrap();
    let mut buf = [0u8; 8];
    f.read_exact(&mut buf).unwrap();
    let mut num: u64 = 0;
    for (i, val) in buf.iter().enumerate() {
        num |= u64::from(*val) << (8 * (7 - i));
    }
    num
}

pub fn get_random_number_u8(max: u8) -> u8 {
    (get_random_number() % u64::from(max)) as u8
}

#[cfg(test)]
mod test {
    use crate::util::get_random_number_u8;

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
    fn random_number_generator_generates_complete_range() {
        for upper_limit in 1..20 {
            let mut generated = Vec::<bool>::with_capacity(upper_limit);
            generated.resize(upper_limit, false);
            while generated.iter().any(|x| *x == false) {
                let x = get_random_number_u8(upper_limit as u8);
                assert!(x < upper_limit as u8);
                generated[x as usize] = true;
            }
        }
    }
}
