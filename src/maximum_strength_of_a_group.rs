fn max_strength(nums: Vec<i32>) -> i64 {
    let (mut min, mut max) = (nums[0] as i64, nums[0] as i64);

    for num in nums.into_iter().skip(1) {
        let (temp_min, temp_max, num) = (min, max, num as i64);
        max = num.max(max).max(temp_max * num).max(temp_min * num);
        min = num.min(min).min(temp_min * num).min(temp_max * num);
    }

    max
}

pub fn main() {
    let nums = [-1,-2,-3,0].to_vec();
    println!("{}", max_strength(nums));
}
