#[derive(Clone)]
struct DigitOptions {
    opts: u16,
}

impl DigitOptions {
    fn check(&self, digit: u16) -> bool {
        return (&self.opts >> (digit as i8 - 1)) & 1 == 1;
    }
    fn count(&self) -> u16 {
        return self.opts.count_ones() as u16;
    }

    fn new(digits: Vec<u16>) -> DigitOptions {
        let powers: Vec<u16> = digits.into_iter().map(|x| 1 << (x as i8 - 1)).collect();
        let sum = powers.into_iter().fold(0, |acc, x| acc + x) as u16;
        return DigitOptions { opts: sum as u16 };
    }
    fn exclude(&self, digits: &DigitOptions) -> DigitOptions{
        return DigitOptions{opts: &self.opts & !digits.opts};
    }
    fn subtract_options(&self, digits:&[&DigitOptions]) -> DigitOptions{
        let mut result = self.clone();
        for i in 1..9 {
            if digits[i].count() == 1{
                result = result.exclude(digits[i]);
            }
        }
        return result;
    }
}

fn available(digits: &[&DigitOptions; 9])-> DigitOptions{
    let mut result = DigitOptions{opts: 0};
    for i in 1..9{
        if digits[i].count() == 1 {
            result = result.exclude(digits[i]);
        }
    }
    return result
}



struct SudokuBoard {
    options: [DigitOptions; 81],
    placed: [bool; 81],
    board_state: [u8; 81],
}

const BOX_COORDS: [usize; 9] = [0, 1, 2, 9, 10, 11, 18, 19, 20];
const BOX_OFFSETS: [usize; 9] = [0, 3, 6, 27, 30, 33, 54, 57, 60];
const COL_COORDS: [usize; 9] = [0, 9, 18, 27, 36, 45, 54, 63, 72];

impl SudokuBoard {
    fn get_row(&self, num: u8) -> [Option<&DigitOptions>; 9] {
        let mut pointers: [Option<&DigitOptions>; 9] = [None; 9];
        let offset = num as usize * 9;
        for i in 0.. {
            pointers[i] = Some(&self.options[i + offset]);
        }
        return pointers;
    }
    fn get_col(&self, num: u8) -> [Option<&DigitOptions>; 9] {
        let mut pointers: [Option<&DigitOptions>; 9] = [None; 9];
        let offset = num as usize;
        for i in 0..9 {
            pointers[i] = Some(&self.options[COL_COORDS[i] + offset]);
        }
        return pointers;
    }
    fn get_box(&self, num: u8) -> [Option<&DigitOptions>; 9] {
        let mut pointers: [Option<&DigitOptions>; 9] = [None; 9];
        let offset = BOX_OFFSETS[num as usize];
        for i in 0..9 {
            pointers[i] = Some(&self.options[BOX_COORDS[i] + offset]);
        }
        return pointers;
    }
}




fn row_xy((x, y): (u8, u8)) -> u8 {
    return y;
}
fn col_xy((x, y): (u8, u8)) -> u8 {
    return x;
}
fn box_xy((x, y): (u8, u8)) -> u8 {
    return 3 * (y / 3) + (x / 3);
}

fn row_from_idx(idx: u8) -> u8 {
    return row_xy(xy(idx));
}
fn col_from_idx(idx: u8) -> u8 {
    return col_xy(xy(idx));
}
fn box_from_idx(idx: u8) -> u8 {
    return box_xy(xy(idx));
}
fn xy(idx: u8) -> (u8, u8) {
    return ((idx % 9) + 1, (idx / 9) + 1);
}
fn idx(x: u8, y: u8) -> u8 {
    let ind = 9 * (y as i8 - 1) + (x as i8 - 1);
    return ind as u8;
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_things() {
        let test_digit_options = crate::board::DigitOptions { opts: 3 };
        assert_eq!(test_digit_options.check(1), true);
        assert_eq!(test_digit_options.check(2), true);
        assert_eq!(test_digit_options.check(3), false);

        let test_digit_options = crate::board::DigitOptions::new(vec![1, 2, 5, 9]);
        let digits = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let check_results: Vec<bool> = digits
            .into_iter()
            .map(|x| test_digit_options.check(x))
            .collect();
        let expected_results = vec![true, true, false, false, true, false, false, false, true];

        assert_eq!(check_results, expected_results);
    }
    #[test]
    fn test_idx() {
        assert_eq!(crate::board::idx(1, 1), 0);
        assert_eq!(crate::board::idx(1, 2), 9);
        assert_eq!(crate::board::idx(5, 1), 4);
        assert_eq!(crate::board::idx(9, 9), 80);
        let (x, y) = crate::board::xy(33);
        assert_eq!(crate::board::idx(x, y), 33);
    }
}
