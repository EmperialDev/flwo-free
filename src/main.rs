use flwo_free::{generate_with_solution, print_board::print_board, };

fn main()  {
   let board = generate_with_solution(7, 5);

   //println!("board: {board:?}");
   print_board(7, 5, board);
}