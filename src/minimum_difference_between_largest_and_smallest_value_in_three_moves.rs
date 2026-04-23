fn min_difference(mut nums: Vec<i32>) -> i32 {
    let n = nums.len();
    if n < 5 {
        return 0;
    }

    nums.sort();
    let mut k = 3;
    let mut res = i32::MAX;
    for i in 0..=3 {
        for j in 0..k+1 {
            res = res.min((nums[n-i-1] - nums[j]).abs());
        }
        k = k.wrapping_sub(1);
    }

    res
}

pub fn main() {
    let nums = vec![1,5,0,10,14];
    println!("{}", min_difference(nums));
}
