fn min_moves2(mut nums: Vec<i32>) -> i32 {
    nums.sort_unstable();

    let delta = nums[(nums.len()-1) / 2];
    nums.into_iter().fold(0, |acc, num| acc + (num - delta).abs())
}

pub fn main() {
    // let nums = [1,2,3].to_vec();
    let nums = [-1000000000,0,1000000000].to_vec();
    println!("{}", min_moves2(nums));
}
