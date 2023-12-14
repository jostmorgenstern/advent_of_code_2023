use std::io;

fn all_zero(nums: &Vec<i64>) -> bool {
    nums.iter().fold(true, | acc, cur | acc && (*cur == 0i64) )
}

fn main() {
    let stdin = io::stdin();
    let mut sum = 0;

    for line in stdin.lines() {
        let mut nums: Vec<i64> = line.unwrap().split_whitespace().map(| num_str | i64::from_str_radix(num_str, 10).unwrap() ).collect();
        let mut last_elems = vec![];
        while !all_zero(&nums) {
            last_elems.push(nums[nums.len() - 1]);
            for i in 0..nums.len()-1 {
                nums[i] = nums[i + 1] - nums[i];
            }
            nums.remove(nums.len() - 1);
        }
        sum += last_elems.iter().sum::<i64>();
    }
    println!("{}", sum);
}
