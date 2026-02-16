fn min_increment_for_unique(mut nums: Vec<i32>) -> i32 {
    nums.sort();

    let mut res = 0;
    let mut need = nums[0];
    for &num in &nums {
        if num < need {
            res += need - num;
        } else {
            need = num;
        }
        need += 1;
    }

    res
}

pub fn main() {
    let nums = [3,2,1,2,1,7].to_vec();
    println!("{}", min_increment_for_unique(nums));
}
