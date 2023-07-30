use colored::*;

pub fn print_board(width: usize, height: usize, board: Vec<Vec<u8>>) {
    for y in 0..height {
        print!("[ ");
        for x in 0..width {
            let num = board[x][y];
            print!("{}, ", num.to_string().color(num_to_color(&num)));
        }
        println!("]");
    }
}

fn num_to_color(num: &u8) -> Color {
    match num {
        0 => Color::BrightBlack,
        1 => Color::Red,
        2 => Color::Green,
        3 => Color::Yellow,
        4 => Color::Blue,
        5 => Color::Magenta,
        6 => Color::Cyan,
        7 => Color::BrightRed,
        8 => Color::BrightGreen,
        9 => Color::BrightYellow,
        10 => Color::BrightBlue,
        11 => Color::BrightMagenta,
        12 => Color::BrightCyan,
        _ => Color::White,
    }
}