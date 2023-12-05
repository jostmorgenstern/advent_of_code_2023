use std::io;
use std::cmp::max;

fn main() {
    let stdin = io::stdin();

    let mut total_points = 0u64;

    for line in stdin.lines() {
        let mut game_points = 0;

        let line_str = line.unwrap();
        let mut split_by_colon = line_str.split(':');
        split_by_colon.next();
        let after_colon = split_by_colon.next().unwrap();

        let mut split_by_pipe = after_colon.split('|');
        let before_pipe = split_by_pipe.next().unwrap();
        let after_pipe = split_by_pipe.next().unwrap();
        //println!("{:?} , {:?}", before_pipe, after_pipe);

        let mut winning_numbers = vec![];
        for num_str in before_pipe.split_whitespace() {
            let num = u32::from_str_radix(num_str, 10).unwrap();
            let pos = winning_numbers.binary_search(&num).unwrap_or_else(|e| e);
            winning_numbers.insert(pos, num);
        }
        for num_str in after_pipe.split_whitespace() {
            let guessed_num = u32::from_str_radix(num_str, 10).unwrap();
            if let Ok(_) = winning_numbers.binary_search(&guessed_num) {
                game_points = max(1, game_points * 2);
            }
        }

        total_points += game_points;

        //println!("{:?}, {:?}", winning_numbers, my_numbers);

    }

    println!("{}", total_points);
}
