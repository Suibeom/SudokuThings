use serde::ser::{SerializeStruct, SerializeTuple, Serializer};
use serde::{Deserialize, Serialize};
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
}

#[derive(Clone)]
pub struct Solver {
    board: Board,
    boxes: [bool; 9],
    last_box: u8,
}

impl Solver {
    fn mark_if_finished(&mut self, bx: SudokuBox) {
        if self.board.left_to_place(&bx).len() == 0 {
            self.boxes[bx.idx] = true;
        }
    }
    fn get_unsolved_box(&mut self) -> Option<SudokuBox> {
        let mut some_unsolved = false;
        for i in 1..9 {
            some_unsolved = some_unsolved | self.boxes[i as usize];
        }
        for i in (self.last_box + 1)..9 {
            if !self.boxes[i as usize] {
                self.last_box = i;
                return Some(self.board.get_box(i as u8));
            }
        }
        if some_unsolved {
            self.last_box = 0;
            return self.get_unsolved_box();
        }
        return None;
    }
    pub fn init_with_board(mut board: Board) -> Solver {
        // if we're just initializing the board assume that any empty squares without pencilmarks are
        // actually untouched, not that they represent a contradiction.
        for i in 0..81 {
            if let None = board.digits[i] {
                if board.pencilmarks[i].len() == 0 {
                    board.pencilmarks[i] = one_to_nine();
                }
            }
        }
        let mut new_solver = Solver {
            board,
            boxes: [false; 9],
            last_box: 0,
        };
        for j in 0..9 {
            new_solver.mark_if_finished(new_solver.board.get_box(j));
        }
        return new_solver;
    }
    pub fn get_board(&self) -> Board {
        return self.board.clone();
    }
}

fn one_to_nine() -> HashSet<u8> {
    set!(1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8, 9u8)
}

impl Board {
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
    fn from_square_state_vec(squares: Vec<SquareState>) -> Board {
        let mut board = Board::empty_board();
        for i in 0..squares.len() {
            match squares[i].contents {
                None => {}
                Some(j) => {
                    board.place_mut(i, j);
                    continue;
                }
            }
            board.pencilmarks[i] = squares[i].pencilmarks.clone();
        }
        return board;
    }
    pub fn from_str(str: String) -> Board {
        let square_state_vec: Vec<SquareState> = serde_json::from_str(&str).unwrap();
        return Board::from_square_state_vec(square_state_vec);
    }

    fn empty_board() -> Board {
        // It's a good thing this heads off a fuckton of bugs, because it's a royal pain in the ass.
        let mut pencilmarks: [MaybeUninit<HashSet<u8>>; 81] =
            unsafe { MaybeUninit::uninit().assume_init() };
        for i in 0..81 {
            pencilmarks[i] = MaybeUninit::new(one_to_nine());
        }
        let pencilmarks = unsafe { mem::transmute::<_, [HashSet<u8>; 81]>(pencilmarks) };
        return Board {
            digits: [None; 81],
            pencilmarks,
        };
    }
}

impl Serialize for Board {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut squarestates = serializer.serialize_tuple(81)?;
        for i in 0..81 {
            let current_square: SquareState = SquareState {
                contents: self.digits[i],
                pencilmarks: self.pencilmarks[i].clone(),
            };
            squarestates.serialize_element(&current_square)?;
        }
        squarestates.end()
    }
}

#[derive(Deserialize)]
pub struct SquareState {
    contents: Option<u8>,
    pencilmarks: HashSet<u8>,
}

impl Serialize for SquareState {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("SquareState", 2)?;
        state.serialize_field("contents", &self.contents)?;
        state.serialize_field("pencilmarks", &self.pencilmarks)?;
        state.end()
    }
}

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
        return SudokuBox {
            rows,
            cols,
            inds,
            idx,
        };
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

    for i in board.left_to_place(current_box) {
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
                next_board.place_mut(j as usize, i);
                next_board.pencilmarks[j as usize] = HashSet::<u8>::new();
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

fn pencil_and_place(board: Solver, current_box: SudokuBox) -> Solver {
    let mut next_board = board.clone();

    next_board.board = place_pencilmarks(next_board.board, &current_box);

    next_board.board = collapse_pencilmarks(next_board.board, &current_box);

    next_board.mark_if_finished(current_box);

    return next_board;
}

pub fn work_one_box(solver: Solver, box_index: Option<u8>) -> Solver {
    let mut new_solver = solver;
    let working_box = match box_index {
        None => match new_solver.get_unsolved_box() {
            Some(sudoku_box) => sudoku_box,
            None => return new_solver,
        },
        Some(box_index) => new_solver.board.get_box(box_index),
    };
    return pencil_and_place(new_solver, working_box);
}

pub fn solve(board: Board) -> Board {
    let mut unsolved = true;
    let mut solver = Solver::init_with_board(board.clone());
    while unsolved {
        // get an unsolved box
        let current_box = match solver.get_unsolved_box() {
            Some(bx) => bx,
            None => {
                unsolved = false;
                continue;
            }
        };
        solver = pencil_and_place(solver, current_box);
    }
    return solver.board;
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_things() {}
    #[test]
    fn test_idx() {}
}
