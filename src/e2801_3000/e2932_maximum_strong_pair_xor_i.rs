fn maximum_strong_pair_xor(nums: Vec<i32>) -> i32 {
    let mut res = 0;
    for i in 0..nums.len() {
        for j in i..nums.len() {
            if (nums[i]-nums[j]).abs() <= nums[i].min(nums[j]) {
                res = res.max(nums[i] ^ nums[j]);
            }
        }
    }

    res
}

pub fn main() {
    let nums = [1,2,3,4,5].to_vec();
    println!("{:?}", maximum_strong_pair_xor(nums));
}
