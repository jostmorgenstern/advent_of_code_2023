use std::io;
use std::collections::HashMap;

fn gcd(first: usize, second: usize) -> usize {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

fn lcm(first: usize, second: usize) -> usize {
    first * second / gcd(first, second)
}

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();
    let mut lines = input.lines();

    let lr_line = lines.next().unwrap();

    lines.next();

    let mut map = HashMap::new();
    let mut cur_nodes = vec![];
    for line in lines {
        let key = &line[..3];
        let left = &line[7..10];
        let right = &line[12..15];
        if key.as_bytes()[2] as char == 'A' {
            cur_nodes.push(key);
        }
        map.insert(key, [left, right]);
    }

    let mut num_steps_per_node = vec![];
    for node in &cur_nodes {
        let mut cur_node = *node;
        let mut num_steps = 0usize;
        let mut i = 0;

        while cur_node.as_bytes()[2] as char != 'Z' {
            if lr_line.as_bytes()[i] as char == 'L' {
                cur_node = map.get(cur_node).unwrap()[0];
            } else {
                cur_node = map.get(cur_node).unwrap()[1];
            }
            num_steps += 1;
            i = (i + 1) % lr_line.len();
        }
        num_steps_per_node.push(num_steps);
    }

    println!("{}", num_steps_per_node.into_iter().reduce(| acc, cur | lcm(acc, cur)).unwrap());
}
