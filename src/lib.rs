// [ a, a, a, b, b ]  |  [ 1, 1, 1, 2, 2 ]  |  [ Cell::Path(1), Cell::Path(1), Cell::End(1) , Cell::Path(2), Cell::End(2)  ]
// [ a, c, c, b, d ]  |  [ 1, 3, 3, 2, 4 ]  |  [ Cell::Path(1), Cell::Path(3), Cell::End(3) , Cell::Path(2), Cell::End(4)  ]
// [ a, c, b, b, d ]  |  [ 1, 3, 2, 2, 4 ]  |  [ Cell::Path(1), Cell::Path(3), Cell::End(2) , Cell::Path(2), Cell::Path(4) ]
// [ a, c, c, c, d ]  |  [ 1, 3, 3, 3, 4 ]  |  [ Cell::Path(1), Cell::Path(3), Cell::Path(3), Cell::End(3) , Cell::Path(4) ]
// [ a, a, d, d, d ]  |  [ 1, 1, 4, 4, 4 ]  |  [ Cell::Path(1), Cell::End(1) , Cell::End(4),  Cell::Path(4), Cell::Path(4) ]

pub mod print_board;

use rand::{prelude::*, distributions::Standard};

pub fn generate_with_solution(width: usize, height: usize) -> Vec<Vec<u8>> {
    let mut rng = thread_rng();
    let mut board = vec![vec![0u8; height]; width];

    for num in 1..=6 {
        let (mut x, mut y) = loop {
            let x = rng.gen_range(0..width);
            let y = rng.gen_range(0..height);
            if board[x][y] == 0 {
                board[x][y] = num;
                break (x, y);
            }
        };

        let mut finished = false;
        let mut length = 1;
        let mut max_iter = 0;
        
        while !finished {
            let dir = rng.gen::<Dir>();

            match dir {
                Dir::Right => if board_contains(&x, &y, &board, Dir::Right, num) {
                    x += 1;
                    length += 1;
                    board[x][y] = num;
                },
                Dir::Left => if board_contains(&x, &y, &board, Dir::Left, num) {
                    x -= 1;
                    length += 1;
                    board[x][y] = num;
                },
                Dir::Up => if board_contains(&x, &y, &board, Dir::Up, num) {
                    y += 1;
                    length += 1;
                    board[x][y] = num;
                },
                Dir::Down => if board_contains(&x, &y, &board, Dir::Down, num) {
                    y -= 1;
                    length += 1;
                    board[x][y] = num;
                },
            }

            if length > 2 {
                finished = rng.gen_bool(0.1);
            } else {
                if checked_add_bounds(&x, &board.len()) && board[x + 1][y] != 0 || x.checked_add(1).is_none() {
                    println!("Couln't move right!");
                    if checked_sub_bounds(&x, &board.len()) && board[x - 1][y] != 0 || x.checked_sub(1).is_none() {
                        println!("Couln't move left!");
                        println!("Y: {y}, {}", checked_add_bounds(&y, &board[0].len()));
                        if checked_add_bounds(&y, &board[0].len()) && board[x][y + 1] != 0 || y.checked_add(1).is_none() {
                            println!("Couln't move up!");
                            if checked_sub_bounds(&y, &board[0].len()) && board[x][y - 1] != 0 || y.checked_sub(1).is_none() {
                                println!("Couln't move down!");
                                finished = true;
                            }
                        }
                    }
                }
            }

            if max_iter > 1000 {
                println!("Max iter!");
                return board;
            }
            max_iter += 1;
        }
    }

    board
}

fn checked_add_bounds(v: &usize, bound: &usize) -> bool {
    if let Some(v) = v.checked_add(1) {
        (0..*bound).contains(&v)
    } else {
        false
    }
}

fn checked_sub_bounds(v: &usize, bound: &usize) -> bool {
    if let Some(v) = v.checked_sub(1) {
        (0..*bound).contains(&v)
    } else {
        false
    }
}

fn board_contains(x: &usize, y: &usize, board: &Vec<Vec<u8>>, dir: Dir, current_num: u8) -> bool {
    match dir {
        Dir::Right => checked_add_bounds(&x, &board.len()) && board[x + 1][*y] == 0 && no_adjacent(&(x + 1), y, board, dir, current_num),
        Dir::Left => checked_sub_bounds(&x, &board.len()) && board[x - 1][*y] == 0 && no_adjacent(&(x - 1), y, board, dir, current_num),
        Dir::Up => checked_add_bounds(&y, &board[0].len()) && board[*x][y + 1] == 0 && no_adjacent(x, &(y + 1), board, dir, current_num),
        Dir::Down => checked_sub_bounds(&y, &board[0].len()) && board[*x][y - 1] == 0 && no_adjacent(x, &(y - 1), board, dir, current_num),
    }
}

fn no_adjacent(x: &usize, y: &usize, board: &Vec<Vec<u8>>, dir: Dir, current_num: u8) -> bool {
    // ...
    // ox.
    // ...
    if dir != Dir::Right {
        if checked_sub_bounds(x, &board.len()) && board[x - 1][*y] == current_num {
            return false;
        }
    }
    // ...
    // .xo
    // ...
    if dir != Dir::Left {
        if checked_add_bounds(x, &board.len()) && board[x + 1][*y] == current_num {
            return false;
        }
    }
    // .o.
    // .x.
    // ...
    if dir != Dir::Up {
        if checked_sub_bounds(y, &board[0].len()) && board[*x][y - 1] == current_num {
            return false;
        }
    }
    // .o.
    // .x.
    // ...
    if dir != Dir::Down {
        if checked_add_bounds(y, &board[0].len()) && board[*x][y + 1] == current_num {
            return false;
        }
    }

    true
}


#[derive(Debug, PartialEq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Distribution<Dir> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Dir {
        match rng.gen_range(0..=3) {
            0 => Dir::Up,
            1 => Dir::Down,
            2 => Dir::Left,
            3 => Dir::Right,
            _ => unreachable!()
        }
    }
}