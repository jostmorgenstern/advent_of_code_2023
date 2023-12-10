use std::io;

#[derive(Debug)]
struct Seed {
    value: i64,
    category_in_progress: bool,
}

fn parse_mapping(input_line: &str) -> (i64, i64, i64) {
    let mut iter = input_line.split_whitespace().map( | num_str | i64::from_str_radix(num_str, 10).unwrap() );
    return ( iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap() );
}

fn main() {
    let stdin = io::stdin();
    let mut lines_iter = stdin.lines();

    // ugliest code ever
    let seeds_line = lines_iter.next().unwrap().unwrap();
    let mut seeds: Vec<Seed> = seeds_line.split(':').nth(1).unwrap().split_whitespace().map( | num_str | { 
        Seed { value: i64::from_str_radix(num_str, 10).unwrap(), category_in_progress: true }
    }).collect();

    // it gets better from here
    lines_iter.next();
    lines_iter.next();

    for line in lines_iter {
        let line_str = line.unwrap();
        if line_str.len() == 0 {
            continue;
        }
        if !line_str.starts_with( |c: char| c.is_digit(10) ) {
            for seed in &mut seeds {
                seed.category_in_progress = true;
            }
            continue;
        }
        let (dst_range_start, src_range_start, range_len) = parse_mapping(&line_str);
        for seed in &mut seeds {
            if seed.category_in_progress {
                if src_range_start <= seed.value && seed.value < src_range_start + range_len {
                    seed.value += dst_range_start - src_range_start;
                    seed.category_in_progress = false;
                }
            }
        }
    }
    // quite ugly again
    println!("{:?}", seeds.iter().min_by( | x, y | x.value.cmp(&y.value) ).unwrap().value);
}

