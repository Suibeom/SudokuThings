use std::collections::HashMap;
use std::collections::HashSet;
use std::mem::{self, MaybeUninit};

#[macro_export]
macro_rules! set {
    ( $( $x:expr ),* ) => {  // Match zero or more comma delimited items
        {
            let mut temp_set = HashSet::<u8>::new();  // Create a mutable HashSet
            $(
                temp_set.insert($x); // Insert each item matched into the HashSet
            )*
            temp_set // Return the populated HashSet
        }
    };
}

fn iter_to_set<'a, A, B>(iter: A) -> HashSet<B>
where
    A: Iterator<Item = &'a B>,
    B: std::cmp::Eq + std::hash::Hash + Clone + Copy + 'a,
{
    let mut outside = HashSet::<B>::new();
    for i in iter {
        outside.insert(*i);
    }
    return outside;
}

#[derive(Clone)]
pub struct Board {
    digits: [Option<u8>; 81],
    pencilmarks: [HashSet<u8>; 81],
    boxes: [bool; 9],
    last_box: u8,
}

fn one_to_nine() -> HashSet<u8> {
    set!(1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8, 9u8)
}

impl Board {
    fn get_unsolved_box(&mut self) -> Option<SudokuBox> {
        let mut some_unsolved = false;
        for i in 1..9{
            some_unsolved = some_unsolved | self.boxes[i as usize];
        }
        for i in self.last_box + 1..9 {
            if !self.boxes[i as usize] {
                self.last_box = i as u8;
                return Some(self.get_box(i as u8));
            }
        }
        if some_unsolved {
            self.last_box = 0;
        }
        return None;
    }
    fn get_box(&self, bx: u8) -> SudokuBox {
        let y = (bx / 3) * 3;
        let x = (bx % 3) * 3;
        let cols = set!(y, y + 1, y + 2);
        let rows = set!(x, x + 1, x + 2);
        return SudokuBox::new(rows, cols, bx as usize);
    }
    fn left_to_place(&self, bx: &SudokuBox) -> HashSet<u8> {
        let mut placed = HashSet::<u8>::new();
        for i in &bx.inds {
            if let Some(j) = self.digits[*i as usize] {
                placed.insert(j);
            }
        }
        let left_to_place = iter_to_set(one_to_nine().difference(&placed));
        return left_to_place;
    }
    fn pencil_out(&self, idx: usize, pencilmarks: &HashSet<u8>) -> Board {
        let mut new_marks = self.pencilmarks[idx].clone();
        for i in pencilmarks {
            new_marks.remove(&i);
        }
        let mut new_board_marks = self.pencilmarks.clone();
        new_board_marks[idx] = new_marks;
        return Board {
            digits: self.digits,
            pencilmarks: new_board_marks,
            boxes: self.boxes,
            last_box: 0,
        };
    }
    fn pencil_out_mut(&mut self, idx: usize, pencilmarks: &HashSet<u8>) {
        for i in pencilmarks {
            self.pencilmarks[idx].remove(&i);
        }
    }
    fn place(&self, idx: usize, digit: u8) -> Board {
        let mut digits = self.digits;
        digits[idx] = Some(digit);
        let mut pencilmarks = self.pencilmarks.clone();
        pencilmarks[idx] = HashSet::<u8>::new();
        return Board {
            digits,
            pencilmarks,
            boxes: self.boxes,
            last_box: 0
        };
    }
    fn place_mut(&mut self, idx: usize, digit: u8) {
        self.digits[idx] = Some(digit);
        self.pencilmarks[idx] = HashSet::<u8>::new();
    }
    pub fn new(digits: HashMap<usize, u8>) -> Board {
        let mut board = Board::empty_board();
        for (key, val) in digits {
            board.place_mut(key, val);
        }
        return board;
    }
    fn empty_board() -> Board {
        // It's a good thing this heads of a fuckton of bugs, because it's a royal pain in the ass.
        let mut pencilmarks: [MaybeUninit<HashSet<u8>>; 81] =
            unsafe { MaybeUninit::uninit().assume_init() };
        for i in 0..81 {
            pencilmarks[i] = MaybeUninit::new(HashSet::<u8>::new());
        }
        let pencilmarks = unsafe { mem::transmute::<_, [HashSet<u8>; 81]>(pencilmarks) };
        return Board {
            digits: [None; 81],
            pencilmarks,
            boxes: [false; 9],
            last_box: 0
        };
    }
    fn mark_if_finished(&mut self, bx: SudokuBox){
        if self.left_to_place(&bx).len()==0{
            self.boxes[bx.idx] = true;
        }
    }
}

// #[derive(Clone)]
// struct Set {
//     digits: HashSet<u8>,
// }
// impl Set {
//     fn intersect(&self, other: &Set) -> Set{
//         let digits = iter_to_set(self.digits.intersection(&other.digits));
//         return Set{digits};
//     }
//     fn  difference(&self, other: &Set) -> Set{
//         let digits = iter_to_set(self.digits.difference(&other.digits));
//         return Set{digits};
//     }
// }

struct SudokuBox {
    rows: HashSet<u8>,
    cols: HashSet<u8>,
    inds: HashSet<u8>,
    idx: usize,
}
impl SudokuBox {
    fn new(rows: HashSet<u8>, cols: HashSet<u8>, idx: usize) -> SudokuBox {
        let mut inds = HashSet::<u8>::new();
        for i in &rows {
            for j in &cols {
                inds.insert(9 * i + j);
            }
        }
        return SudokuBox { rows, cols, inds , idx};
    }
    fn row_inds_outside_box(&self, row: u8) -> HashSet<u8> {
        let row_inds = set!(
            9 * row,
            1 + 9 * row,
            2 + 9 * row,
            3 + 9 * row,
            4 + 9 * row,
            5 + 9 * row,
            6 + 9 * row,
            7 + 9 * row,
            8 + 9 * row
        );
        let diff = iter_to_set(row_inds.difference(&self.inds));
        return diff;
    }
    fn row_inds_inside_box(&self, row: u8) -> HashSet<u8> {
        let row_inds = set!(
            9 * row,
            1 + 9 * row,
            2 + 9 * row,
            3 + 9 * row,
            4 + 9 * row,
            5 + 9 * row,
            6 + 9 * row,
            7 + 9 * row,
            8 + 9 * row
        );
        let intersect = iter_to_set(row_inds.intersection(&self.inds));
        return intersect;
    }
    fn col_inds_outside_box(&self, col: u8) -> HashSet<u8> {
        let col_inds = set!(
            col,
            col + 9,
            col + 18,
            col + 27,
            col + 36,
            col + 45,
            col + 63,
            col + 72
        );
        let diff = iter_to_set(col_inds.difference(&self.inds));
        return diff;
    }
    fn col_inds_inside_box(&self, col: u8) -> HashSet<u8> {
        let col_inds = set!(
            col,
            col + 9,
            col + 18,
            col + 27,
            col + 36,
            col + 45,
            col + 63,
            col + 72
        );
        let intersect = iter_to_set(col_inds.intersection(&self.inds));
        return intersect;
    }
}

fn collapse_pencilmarks(board: Board, current_box: &SudokuBox) -> Board {
    let mut next_board = board.clone();
    // This step is basically laser-finding Naked Singles
    // so I don't think I want it in for 'humanistic' reasons.
    //
    // for i in current_box.inds.clone() {
    //     if board.pencilmarks[i as usize].len() == 1 {
    //         next_board.place(
    //             i as usize,
    //             *board.pencilmarks[i as usize].iter().next().unwrap(),
    //         );
    //         next_board.pencilmarks[i as usize] = HashSet::<u8>::new();
    //     }
    // }

    for i in one_to_nine() {
        let mut count = 0;
        let mut last_index: Option<usize> = None;
        for j in current_box.inds.clone() {
            if board.pencilmarks[j as usize].contains(&i) {
                count += 1;
                last_index.replace(j as usize);
            }
        }
        if count == 1 {
            if let Some(j) = last_index {
                next_board.place(j as usize, i);
                next_board.pencilmarks[i as usize] = HashSet::<u8>::new();
            }
        }
    }

    return next_board;
}

fn place_pencilmarks(board: Board, current_box: &SudokuBox) -> Board {
    let mut next_board = board.clone();

    //rows first
    for i in current_box.rows.clone() {
        let mut digits_seen = HashSet::<u8>::new();
        for j in current_box.row_inds_outside_box(i) {
            if let Some(i) = board.digits[j as usize] {
                digits_seen.insert(i);
            }
        }
        for k in current_box.row_inds_inside_box(i) {
            next_board.pencil_out_mut(k as usize, &digits_seen);
        }
    }

    //then cols
    for i in current_box.cols.clone() {
        let mut digits_seen = HashSet::<u8>::new();
        for j in current_box.col_inds_outside_box(i) {
            if let Some(i) = board.digits[j as usize] {
                digits_seen.insert(i);
            }
        }
        for k in current_box.col_inds_inside_box(i) {
            next_board.pencil_out_mut(k as usize, &digits_seen);
        }
    }
    return next_board;
}

fn pencil_and_place(board: Board, current_box: SudokuBox) -> Board {
    let mut next_board = board.clone();

    next_board = place_pencilmarks(next_board, &current_box);

    next_board = collapse_pencilmarks(next_board, &current_box);

    next_board.mark_if_finished(current_box);

    return next_board;
}

pub fn solve(board: Board) -> Board{
    let mut unsolved = true;
    let mut working_board = board.clone();
    while unsolved {
        // get an unsolved box
        let current_box = match working_board.get_unsolved_box() {
            Some(bx) => bx,
            None => {
                unsolved = false;
                continue;
            }
        };
        working_board = pencil_and_place(working_board, current_box);
    }
    return working_board;
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_things() {}
    #[test]
    fn test_idx() {}
}
