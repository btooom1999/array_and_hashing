fn get_maximum_xor(nums: Vec<i32>, maximum_bit: i32) -> Vec<i32> {
    let mut sum = 0;
    for &num in &nums {
        sum ^= num;
    }

    let mut res = vec![0; nums.len()];
    for (i, num) in nums.into_iter().rev().enumerate() {
        res[i] = sum ^ (2i32.pow(maximum_bit as u32)-1);
        sum ^= num;
    }

    res
}

pub fn main() {
    let nums = [0,1,1,3].to_vec();
    let maximum_bit = 2;
    println!("{:?}", get_maximum_xor(nums, maximum_bit));
}
