use std::collections::HashMap;

const NYT_EASY_BOARD: [(usize, u8); 38] = [
    (0, 4),
    (1, 8),
    (3, 1),
    (6, 6),
    (8, 9),
    (11, 7),
    (13, 8),
    (15, 1),
    (17, 5),
    (18, 5),
    (20, 6),
    (22, 4),
    (23, 7),
    (24, 2),
    (29, 8),
    (32, 1),
    (36, 6),
    (38, 2),
    (39, 8),
    (40, 7),
    (41, 3),
    (42, 5),
    (45, 7),
    (49, 6),
    (51, 9),
    (53, 3),
    (55, 7),
    (56, 4),
    (57, 6),
    (59, 8),
    (61, 9),
    (63, 8),
    (64, 6),
    (66, 7),
    (68, 2),
    (73, 3),
    (79, 6),
    (80, 7),
];

const NYT_HARD_BOARD: [(usize, u8); 24] = [
    (14, 1),
    (15, 2),
    (16, 6),
    (17, 9),
    (18, 2),
    (22, 5),
    (26, 1),
    (31, 8),
    (32, 6),
    (33, 9),
    (37, 5),
    (40, 4),
    (41, 9),
    (52, 7),
    (55, 3),
    (56, 8),
    (58, 7),
    (60, 6),
    (65, 5),
    (70, 9),
    (71, 7),
    (73, 9),
    (77, 5),
    (80, 4),
];

fn nyt_easy_vec() -> Vec<(usize, u8)> {
    return NYT_EASY_BOARD.to_vec();
}
pub fn nyt_easy_map() -> HashMap<usize, u8> {
    let mut board_map = HashMap::<usize, u8>::new();
    let vec = nyt_easy_vec();
    for (j, k) in vec {
        board_map.insert(j, k);
    }
    return board_map;
}

fn nyt_hard_vec() -> Vec<(usize, u8)> {
    return NYT_HARD_BOARD.to_vec();
}
pub fn nyt_hard_map() -> HashMap<usize, u8> {
    let mut board_map = HashMap::<usize, u8>::new();
    let vec = nyt_hard_vec();
    for (j, k) in vec {
        board_map.insert(j, k);
    }
    return board_map;
}
