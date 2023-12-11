use std::io;

/*
fn get_num_digits(x: u64) -> u64 {
    x.checked_ilog10().unwrap_or(0) + 1
}
*/

fn main() {
    let stdin = io::stdin();
    let mut input_lines = stdin.lines();
    let time_line = input_lines.next().unwrap().unwrap();
    let distance_line = input_lines.next().unwrap().unwrap();


    let mut times = time_line.split_whitespace().skip(1);
    let mut distances = distance_line.split_whitespace().skip(1);

    let mut time = 0u64;
    let mut distance = 0u64;

    for (time_substr, distance_substr) in times.zip(distances) {
        let time_substr_int = u64::from_str_radix(time_substr, 10).unwrap();
        let distance_substr_int = u64::from_str_radix(distance_substr, 10).unwrap();
        time = time * 10u64.pow(time_substr.len() as u32) + time_substr_int;
        distance = distance * 10u64.pow(distance_substr.len() as u32) + distance_substr_int;
    }

    let mut num_ways = 0;
    for t_hold in 1..time-1 {
        if (time - t_hold) * t_hold > distance {
            num_ways += 1;
        }
    }

    println!("{}", num_ways);

}

