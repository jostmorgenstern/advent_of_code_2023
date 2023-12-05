use std::io;

// returns:
// a vector of every number in an input line with its input index
// another vector of every symbol position in the input line
fn get_numbers_and_symbol_positions(input_line: &str) -> (Vec<(u32, u32)>, Vec<u32>) {
    let mut numbers = Vec::new();
    let mut symbol_positions = Vec::new();

    let mut cur_number: Option<(u32, u32)> = None;

    for (i, char) in input_line.char_indices() {
        if char.is_digit(10) {
            match cur_number {
                None => { 
                    cur_number = Some( (i as u32, char.to_digit(10).unwrap()) ); 
                },
                Some( (start_idx, cur_agg) ) => { cur_number = Some( (start_idx, cur_agg * 10 + char.to_digit(10).unwrap()) ); }
            }
        } else {
            // symbol
            if char != '.' {
                if let Some( (start_idx, cur_agg) ) = cur_number {
                    numbers.push( (start_idx, cur_agg) );
                    cur_number = None;
                }
                symbol_positions.push(i as u32);
            // period
            } else {
                if let Some( (start_idx, cur_agg) ) = cur_number {
                    numbers.push( (start_idx, cur_agg) );
                    cur_number = None;
                }
            }
        }
    }
    if let Some( (start_idx, cur_agg) ) = cur_number {
        numbers.push( (start_idx, cur_agg) );
    }

    return (numbers, symbol_positions);
}

fn decimal_digit_count(x: &u32) -> u32 {
    x.checked_ilog10().unwrap_or(0) + 1
}

fn add_numbers_with_adjacent_symbol_in_same_line(numbers: &mut Vec<(u32, u32)>, symbols: &[u32]) -> u32 {
    let mut sum = 0u32;

    numbers.retain( | &(num_start_pos, num) | {
        let digit_count = decimal_digit_count(&num);
        for &symbol_pos in symbols {
            // e.g. ...#420...
            if num_start_pos > 0 && symbol_pos == num_start_pos - 1 {
                sum += num;
                return false;
            }
            // e.g. ...420#...
            if symbol_pos == num_start_pos + digit_count {
                sum += num;
                return false;
            }
            // e.g. ...420.#...
            if symbol_pos > num_start_pos + digit_count {
                return true;
            }
        }
        return true;
    });
    
    return sum;
}

fn add_numbers_with_adjacent_symbol_in_adjacent_line(numbers: &mut Vec<(u32, u32)>, symbols: &[u32]) -> u32 {
    let mut sum = 0u32;

    numbers.retain( | &(num_start_pos, num) | {
        let digit_count = decimal_digit_count(&num);
        for &symbol_pos in symbols {
            // e.g.
            //..#......
            //...420...
            if num_start_pos > 0 && symbol_pos == num_start_pos - 1 {
                sum += num;
                return false;
            }
            // e.g.
            //...#.....
            //...420...
            // or
            //......#..
            //...420...
            if symbol_pos >= num_start_pos && symbol_pos <= num_start_pos + digit_count {
                sum += num;
                return false;
            }
            //.......#.
            //...420...
            if symbol_pos > num_start_pos + digit_count {
                return true;
            }

        }
        return true;
    });
    
    return sum;
}


fn main() {
    let stdin = io::stdin();

    let mut prev_line_numbers: Vec<(u32, u32)> = Vec::new();
    let mut cur_line_numbers: Vec<(u32, u32)>;
    
    let mut prev_line_symbol_positions: Vec<u32> = Vec::new();
    let mut cur_line_symbol_positions: Vec<u32>;

    let mut total_sum = 0u32;

    for line in stdin.lines() {
        (cur_line_numbers, cur_line_symbol_positions) = get_numbers_and_symbol_positions(line.unwrap().as_str());


        total_sum += add_numbers_with_adjacent_symbol_in_same_line(&mut cur_line_numbers, cur_line_symbol_positions.as_slice());
        total_sum += add_numbers_with_adjacent_symbol_in_adjacent_line(&mut cur_line_numbers, prev_line_symbol_positions.as_slice());
        total_sum += add_numbers_with_adjacent_symbol_in_adjacent_line(&mut prev_line_numbers, cur_line_symbol_positions.as_slice());


        prev_line_numbers = cur_line_numbers;
        prev_line_symbol_positions = cur_line_symbol_positions;


    }
    println!("{}", total_sum);
}
