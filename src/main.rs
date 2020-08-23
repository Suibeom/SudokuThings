mod board;
mod consts;

use serde_json;


fn main() {
    println!("Hello, world!");

    let nyt_easy_starting_board = board::Board::new(crate::consts::nyt_easy_map());
    
    let serialized_board = serde_json::to_string(&nyt_easy_starting_board);

    println!("Serialized board state: {}", serialized_board.unwrap());

    let _solved = board::solve(nyt_easy_starting_board);

}


