use std::io;

fn all_zero(nums: &Vec<i64>) -> bool {
    nums.iter().fold(true, | acc, cur | acc && (*cur == 0i64) )
}

fn main() {
    let stdin = io::stdin();
    let mut sum = 0;

    for line in stdin.lines() {
        let mut nums: Vec<i64> = line.unwrap().split_whitespace().map(| num_str | i64::from_str_radix(num_str, 10).unwrap() ).collect();
        let mut first_elems = vec![];
        let mut agg = 0;
        while !all_zero(&nums) {
            first_elems.push(nums[0]);
            for i in 0..nums.len()-1 {
                nums[i] = nums[i + 1] - nums[i];
            }
            nums.remove(nums.len() - 1);
        }
        for n in first_elems.iter().rev() {
            agg = n - agg;
        }
        sum += agg;
    }
    println!("{}", sum);
}
