mod board;
mod consts;




fn main() {
    println!("Hello, world!");

    let nyt_easy_starting_board = board::Board::new(crate::consts::nyt_easy_map());
    
    let _solved = board::solve(nyt_easy_starting_board);
}


