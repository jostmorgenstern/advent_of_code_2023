use std::io;
use std::iter;

fn get_game_points(input_line: &str) -> u32 {
    let mut game_points = 0u32;

    let mut split_by_colon = input_line.split(':');
    split_by_colon.next();
    let after_colon = split_by_colon.next().unwrap();

    let mut split_by_pipe = after_colon.split('|');
    let before_pipe = split_by_pipe.next().unwrap();
    let after_pipe = split_by_pipe.next().unwrap();

    let mut winning_numbers = vec![];
    for num_str in before_pipe.split_whitespace() {
        let num = u32::from_str_radix(num_str, 10).unwrap();
        let pos = winning_numbers.binary_search(&num).unwrap_or_else(|e| e);
        winning_numbers.insert(pos, num);
    }
    for num_str in after_pipe.split_whitespace() {
        let guessed_num = u32::from_str_radix(num_str, 10).unwrap();
        if let Ok(_) = winning_numbers.binary_search(&guessed_num) {
            game_points += 1;
        }
    }

    return game_points;
}


fn main() {
    let stdin = io::stdin();

    let mut total_points = 0usize;
    let mut copies = vec![1usize];

    for (line_no, line) in stdin.lines().enumerate() {
        let game_points = get_game_points(line.unwrap().as_str()) as usize;
        if copies.len() < line_no + game_points + 1 {
            copies.extend(
                iter::repeat(1).take(line_no + game_points + 1 - copies.len())
            );
        }
        for i in line_no + 1 .. line_no + 1 + game_points {
            copies[i] += copies[line_no];
        }
        total_points += copies[line_no];
    }

    println!("{}", total_points);
}
