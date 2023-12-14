use std::io;
use std::collections::HashMap;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lines();
    let lr_line = lines.next().unwrap().unwrap();

    lines.next();

    let mut map = HashMap::new();
    for line in lines {
        let line_str = line.unwrap();
        let key = line_str[..3].to_string();
        let left = line_str[7..10].to_string();
        let right = line_str[12..15].to_string();
        println!("{}", line_str);
        //println!("{:?}, {:?}, {:?}", key, left, right);
        map.insert(key, [left, right]);
    }

    println!();
    let mut cur = "AAA";
    let mut i = 0;
    let mut num_steps = 0;
    while cur != "ZZZ" {
        println!("{}", cur);
        if lr_line.as_bytes()[i] as char == 'L' {
            cur = &map.get(cur).unwrap()[0];
        } else {
            cur = &map.get(cur).unwrap()[1];
        }
        num_steps += 1;
        i = (i + 1) % lr_line.len();
    }


    println!("{}", num_steps);

}
