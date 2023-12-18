use std::io;
use std::cmp::{max, min};

fn main() {
    let stdin = io::stdin();
    let mut col_widths = vec![];
    let mut line_widths = vec![];
    let mut galaxies = vec![];

    for (line_no, line) in stdin.lines().enumerate() {
        let line_str = line.unwrap();
        if line_no == 0 {
            col_widths = vec![1000000u64; line_str.len()];
        }
        let mut cur_line_galaxies: Vec<(usize, usize)> = line_str.match_indices('#').map( | (j, pat) | (line_no, j) ).collect();
        for &(line_no, i) in &cur_line_galaxies {
            col_widths[i] = 1;
        }
        if cur_line_galaxies.len() == 0 {
            line_widths.push(1000000u64);
        } else {
            line_widths.push(1);
        }
        galaxies.extend(cur_line_galaxies);
    }

    let mut dist_sum = 0u64;
    for i in 0..galaxies.len() {
        for j in i+1..galaxies.len() {
            let vertical_max = max(galaxies[i].0, galaxies[j].0);
            let vertical_min = min(galaxies[i].0, galaxies[j].0);
            let horizontal_max = max(galaxies[i].1, galaxies[j].1);
            let horizontal_min = min(galaxies[i].1, galaxies[j].1);

            let mut vertical_dist = 0;
            let mut horizontal_dist = 0;

            for k in vertical_min+1..vertical_max+1 {
                vertical_dist += line_widths[k];
            }
            for k in horizontal_min+1..horizontal_max+1 {
                horizontal_dist += col_widths[k];
            }
            let dist = vertical_dist + horizontal_dist;
            dist_sum += dist;
        }
    }

    println!("{}", dist_sum);

}
