use std::io;

fn main() {
    let stdin = io::stdin();
    let mut sum = 0;
    for line in stdin.lines() {
        let mut first_digit: Option<u32> = None;
        let mut last_digit: Option<u32> = None;
        for cur_char in line.unwrap().chars() {
            if cur_char.is_digit(10) {
                if first_digit.is_none() {
                    first_digit = cur_char.to_digit(10);
                }
                last_digit = cur_char.to_digit(10);
            }
        }
        sum += first_digit.unwrap() * 10 + last_digit.unwrap();
    }
    println!("{}", sum);

}
