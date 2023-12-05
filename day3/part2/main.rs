use std::io;

#[derive(Debug)]
struct Gear {
    adjacent_number_count: u32,
    ratio: u32,
    pos: u32,
}

#[derive(Debug)]
struct Number {
    value: u32,
    start_pos: u32,
}

fn get_gears_and_numbers(input_line: &str) -> (Vec<Gear>, Vec<Number>) {
    let mut numbers = vec![];
    let mut gears = vec![];

    let mut cur_number: Option<Number> = None;

    for (i, char) in input_line.char_indices() {
        if char.is_digit(10) {
            match cur_number {
                None => { 
                    cur_number = Some( Number { value: char.to_digit(10).unwrap(), start_pos: i as u32 } ); 
                },
                Some( Number { value: cur_value, start_pos: cur_start_pos } ) => {
                    cur_number = Some( Number { value: cur_value * 10 + char.to_digit(10).unwrap(), start_pos: cur_start_pos } );
                }
            }
        } else {
            if let Some(x) = cur_number {
                numbers.push(x);
                cur_number = None;
            }
            if char == '*' {
                gears.push( Gear { adjacent_number_count: 0, ratio: 1, pos: i as u32 } );
            }
        }
    }
    if let Some(x) = cur_number {
        numbers.push(x);
    }

    return (gears, numbers);
}

fn decimal_digit_count(x: &u32) -> u32 {
    x.checked_ilog10().unwrap_or(0) + 1
}


fn add_adjacent_numbers_to_gears(gears: &mut Vec<Gear>, numbers: &[Number]){
    for gear in gears {
        for number in numbers {
            let digit_count = decimal_digit_count(&number.value);

            if gear.pos + 1 >= number.start_pos && gear.pos <= number.start_pos + digit_count {
                gear.ratio *= number.value;
                gear.adjacent_number_count += 1;
            }

            if gear.pos < number.start_pos + 1 {
                break;
            }
        }
    }
}

fn main() {
    let stdin = io::stdin();

    let mut prev_numbers = vec![];
    let mut prev_gears = vec![];

    let mut cur_numbers;
    let mut cur_gears;

    let mut sum = 0u32;

    for line in stdin.lines() {
        (cur_gears, cur_numbers) = get_gears_and_numbers(line.unwrap().as_str());

        add_adjacent_numbers_to_gears(&mut cur_gears, cur_numbers.as_slice());
        add_adjacent_numbers_to_gears(&mut prev_gears, cur_numbers.as_slice());
        add_adjacent_numbers_to_gears(&mut cur_gears, prev_numbers.as_slice());

        for gear in prev_gears {
            if gear.adjacent_number_count == 2 {
                sum += gear.ratio;
            }
        }
        prev_gears = cur_gears;
        prev_numbers = cur_numbers;

    }
    println!("{}", sum);
}
