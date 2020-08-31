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
        println!(
            "Lef to place in box {}: {}",
            bx.idx,
            self.board.left_to_place(&bx).len()
        );
        if self.board.left_to_place(&bx).len() == 0 {
            self.boxes[bx.idx] = true;
        }
    }
    fn get_unsolved_box(&mut self) -> Option<SudokuBox> {
        let mut all_solved = true;
        for i in 0..9 {
            all_solved = all_solved & self.boxes[i as usize];
        }
        if !all_solved {
            let mut idx = self.last_box;
            idx += 1;
            if idx > 8 {
                idx = 0;
            }
            while self.boxes[idx as usize] {
                if idx < 8 {
                    idx += 1;
                } else {
                    idx = 0;
                }
            }
            self.last_box = idx;
            return Some(self.board.get_box(idx as u8));
        };
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

fn get_box_index(idx: usize) -> u8 {
    let row = idx % 9;
    let col = idx / 9;
    let block_x = row / 3;
    let block_y = col / 3;
    let bx = 3 * block_x + block_y;
    return bx as u8;
}

impl Board {
    fn get_box(&self, bx: u8) -> SudokuBox {
        let y = (bx / 3) * 3;
        let x = (bx % 3) * 3;
        let cols = set!(y, y + 1, y + 2);
        let rows = set!(x, x + 1, x + 2);
        return SudokuBox::new(rows, cols, bx as usize);
    }
    fn placed(&self, bx: &SudokuBox) -> HashSet<u8> {
        let mut placed = HashSet::<u8>::new();
        for i in &bx.inds {
            if let Some(j) = self.digits[*i as usize] {
                placed.insert(j);
            }
        }
        return placed;
    }
    fn left_to_place(&self, bx: &SudokuBox) -> HashSet<u8> {
        let left_to_place = iter_to_set(one_to_nine().difference(&self.placed(bx)));
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
            col + 54,
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
            col + 54,
            col + 63,
            col + 72
        );
        let intersect = iter_to_set(col_inds.intersection(&self.inds));
        return intersect;
    }
}

//Conventional digit placement.
fn type_1_pencilmark_collapse(board: Board, current_box: &SudokuBox) -> Board {
    let mut next_board = board.clone();
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

//Determine 'naked singles'
fn type_2_pencilmark_collapse(board: Board, current_box: &SudokuBox) -> Board {
    let mut next_board = board.clone();

    for i in current_box.inds.clone() {
        if board.pencilmarks[i as usize].len() == 1 {
            let digits = board.pencilmarks[i as usize].clone();
            println!("Digits: {:?}", digits);
            for digit in digits {
                println!("Digit:{}", digit);
                next_board.place_mut(i as usize, digit);
            }
            next_board.pencilmarks[i as usize] = HashSet::<u8>::new();
        }
    }

    // for i in board.left_to_place(current_box) {
    //     let mut count = 0;
    //     let mut last_index: Option<usize> = None;
    //     for j in current_box.inds.clone() {
    //         if board.pencilmarks[j as usize].contains(&i) {
    //             count += 1;
    //             last_index.replace(j as usize);
    //         }
    //     }
    //     if count == 1 {
    //         if let Some(j) = last_index {
    //             next_board.place_mut(j as usize, i);
    //             next_board.pencilmarks[j as usize] = HashSet::<u8>::new();
    //         }
    //     }
    // }

    return next_board;
}

fn place_simple_pencilmarks(board: Board, current_box: &SudokuBox) -> Board {
    let mut next_board = board.clone();

    //rows first
    for i in current_box.rows.clone() {
        let mut digits_seen = HashSet::<u8>::new();
        for j in current_box.row_inds_outside_box(i) {
            if let Some(l) = board.digits[j as usize] {
                digits_seen.insert(l);
                // println!("Seen digit {} in row {}", l, i);
            }
        }
        for k in current_box.row_inds_inside_box(i) {
            // println!("Penciling out options from row {}, box {}", i, k);
            next_board.pencil_out_mut(k as usize, &board.placed(current_box));
            next_board.pencil_out_mut(k as usize, &digits_seen);
        }
    }

    //then cols
    for i in current_box.cols.clone() {
        let mut digits_seen = HashSet::<u8>::new();
        for j in current_box.col_inds_outside_box(i) {
            if let Some(l) = board.digits[j as usize] {
                digits_seen.insert(l);
                // println!("Seen digit {} in column {}", l, i);
            }
        }
        for k in current_box.col_inds_inside_box(i) {
            // println!("Penciling out options from column {}, box {}", i, k);
            next_board.pencil_out_mut(k as usize, &board.placed(current_box));
            next_board.pencil_out_mut(k as usize, &digits_seen);
        }
    }
    return next_board;
}

// If a digit is only possible in a certain row or column outside of the current box
// it can be penciled out of that row or column in the current box.

fn place_derived_pencilmarks(board: Board, current_box: &SudokuBox) -> Board {
    let mut next_board = board.clone();

    //rows first
    for i in current_box.rows.clone() {
        let mut other_rows = current_box.rows.clone();
        other_rows.remove(&i);
        let mut box_a_pencilmarks_in_row = HashSet::<u8>::new();
        let mut box_a_pencilmarks_in_other_rows = HashSet::<u8>::new();
        let mut box_b_pencilmarks_in_row = HashSet::<u8>::new();
        let mut box_b_pencilmarks_in_other_rows = HashSet::<u8>::new();
        let mut box_a_idx: u8 = 10;
        for j in current_box.row_inds_outside_box(i) {
            if box_a_idx == 10 {
                box_a_idx = get_box_index(j as usize);
            }
            if box_a_idx == get_box_index(j as usize) {
                box_a_pencilmarks_in_row =
                    iter_to_set(box_a_pencilmarks_in_row.union(&board.pencilmarks[j as usize]));
            } else {
                box_b_pencilmarks_in_row =
                    iter_to_set(box_b_pencilmarks_in_row.union(&board.pencilmarks[j as usize]));
            }
        }
        for h in other_rows {
            for j in current_box.row_inds_outside_box(h) {
                if box_a_idx == get_box_index(j as usize) {
                    box_a_pencilmarks_in_other_rows = iter_to_set(
                        box_a_pencilmarks_in_other_rows.union(&board.pencilmarks[j as usize]),
                    );
                } else {
                    box_b_pencilmarks_in_other_rows = iter_to_set(
                        box_b_pencilmarks_in_other_rows.union(&board.pencilmarks[j as usize]),
                    );
                }
            }
        }

        let box_a_elidable_pencilmarks =
            iter_to_set(box_a_pencilmarks_in_row.difference(&box_a_pencilmarks_in_other_rows));
        let box_b_elidable_pencilmarks =
            iter_to_set(box_b_pencilmarks_in_row.difference(&box_b_pencilmarks_in_other_rows));
        let elidable_pencilmarks =
            iter_to_set(box_a_elidable_pencilmarks.union(&box_b_elidable_pencilmarks));
        for k in current_box.row_inds_inside_box(i) {
            // println!("Penciling out options from row {}, box {}", i, k);
            next_board.pencil_out_mut(k as usize, &elidable_pencilmarks);
        }
    }

    //then cols
    for i in current_box.cols.clone() {
        let mut other_cols = current_box.cols.clone();
        other_cols.remove(&i);
        let mut pencilmarks_in_col = HashSet::<u8>::new();
        let mut pencilmarks_in_other_cols = HashSet::<u8>::new();
        for j in current_box.col_inds_outside_box(i) {
            pencilmarks_in_col =
                iter_to_set(pencilmarks_in_col.union(&board.pencilmarks[j as usize]));
        }
        for h in other_cols {
            for j in current_box.col_inds_outside_box(h) {
                pencilmarks_in_other_cols =
                    iter_to_set(pencilmarks_in_other_cols.union(&board.pencilmarks[j as usize]));
            }
        }
        let elidable_pencilmarks =
            iter_to_set(pencilmarks_in_col.difference(&pencilmarks_in_other_cols));
        for k in current_box.row_inds_inside_box(i) {
            next_board.pencil_out_mut(k as usize, &elidable_pencilmarks);
        }
    }

    return next_board;
}

// If you have n cells whose pencilmarks are all a subset of a subset of size n,
// You can eliminate those pencilmarks from all other cells.

// For naturalism reasons, this will only look at sets already occuring in the box.
fn resolve_subset_pencilmarks(board: Board, current_box: &SudokuBox) -> Board {
    let mut next_board = board.clone();

    for i in current_box.inds.clone() {
        let current_marks = board.pencilmarks[i as usize].clone();
        let mut similar_boxes = HashSet::<u8>::new();
        //find the squares whose pencilmarks are a subset of this one
        for j in current_box.inds.clone() {
            if let Some(_) = board.digits[j as usize] {
                continue;
            }
            if board.pencilmarks[j as usize].is_subset(&current_marks) {
                similar_boxes.insert(j);
            }
        }
        //If the number of squares matches the size of the subset, those
        // numbers are all constrained to these squares so pencil them out!
        if similar_boxes.len() == current_marks.len() {
            for j in iter_to_set(current_box.inds.clone().difference(&similar_boxes)) {
                next_board.pencil_out_mut(j as usize, &current_marks);
            }
        }
    }

    return next_board;
}

fn resolve_cycle_pencilmarks(board: Board, current_box: &SudokuBox) -> Board {
    let mut next_board = board.clone();
    let mut already_visited = HashSet::<u8>::new();
    for i in current_box.inds.clone() {
        if already_visited.contains(&i) {
            continue;
        }
        if board.pencilmarks[i as usize].len() != 2 {
            continue;
        }
        let mut cycle_boxes = HashSet::<u8>::new();
        cycle_boxes.insert(i);

        let cycle_start = board.pencilmarks[i as usize].clone();
        let mut cycle_current = cycle_start.clone();
        let mut digits_seen = cycle_start.clone();
        let mut closed: bool = false;
        let mut finished: bool = false;
        while finished == false {
            finished = true;
            for j in current_box.inds.clone() {
                if already_visited.contains(&j) {
                    continue;
                }
                if board.pencilmarks[j as usize].len() != 2 {
                    continue;
                }
                // cycle closed, break and sort it all out.
                if iter_to_set(cycle_start.intersection(&board.pencilmarks[j as usize])).len() == 1
                {
                    cycle_boxes.insert(j);
                    closed = true;
                    finished = true;
                    break;
                }
                //cycle continued!
                if iter_to_set(cycle_current.intersection(&board.pencilmarks[j as usize])).len()
                    == 1
                {
                    cycle_current = board.pencilmarks[j as usize].clone();
                    digits_seen = iter_to_set(digits_seen.union(&cycle_current));
                    cycle_boxes.insert(j);
                    finished = false;
                    break;
                }
            }
        }
        already_visited = iter_to_set(already_visited.union(&cycle_boxes));
        if closed {
            for j in iter_to_set(current_box.inds.clone().difference(&cycle_boxes)) {
                next_board.pencil_out_mut(j as usize, &digits_seen);
            }
        }
    }

    return next_board;
}

fn pencil_and_place_simple(board: Solver, current_box: SudokuBox) -> Solver {
    let mut next_board = board.clone();

    next_board.board = place_simple_pencilmarks(next_board.board, &current_box);

    next_board.board = type_1_pencilmark_collapse(next_board.board, &current_box);

    next_board.mark_if_finished(current_box);

    return next_board;
}

fn pencil_and_place_complex(board: Solver, current_box: SudokuBox) -> Solver {
    let mut next_board = board.clone();

    next_board.board = place_simple_pencilmarks(next_board.board, &current_box);

    // next_board.board = place_derived_pencilmarks(next_board.board, &current_box);

    next_board.board = resolve_cycle_pencilmarks(next_board.board, &current_box);

    // next_board.board = resolve_subset_pencilmarks(next_board.board, &current_box);

    next_board.board = type_1_pencilmark_collapse(next_board.board, &current_box);

    next_board.board = type_2_pencilmark_collapse(next_board.board, &current_box);

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
    return pencil_and_place_complex(new_solver, working_box);
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
        solver = pencil_and_place_simple(solver, current_box);
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
