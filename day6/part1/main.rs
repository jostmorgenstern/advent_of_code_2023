use std::io;


fn main() {
    let stdin = io::stdin();
    let mut input_lines = stdin.lines();
    let times_line = input_lines.next().unwrap().unwrap();
    let distances_line = input_lines.next().unwrap().unwrap();


    let mut times = times_line.split_whitespace().skip(1).map( | num_str | u64::from_str_radix(num_str, 10).unwrap() );
    let mut distances = distances_line.split_whitespace().skip(1).map( | num_str | u64::from_str_radix(num_str, 10).unwrap() );

    let mut product = 1;

    for (time, distance) in times.zip(distances) {
        println!("{}, {}", time, distance);
        let mut num_ways = 0;
        for t_hold in 1..time-1 {
            if (time - t_hold) * t_hold > distance {
                num_ways += 1;
            }
        }
        product *= num_ways;
    }

    println!("{}", product);
}

