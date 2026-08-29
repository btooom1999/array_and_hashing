fn min_bitwise_array(nums: Vec<i32>) -> Vec<i32> {
    let mut res = vec![-1; nums.len()];
    for (i, target) in nums.into_iter().enumerate() {
        for num in 1..target {
            if num | (num+1) == target {
                res[i] = num;
                break;
            }
        }
    }

    res
}

pub fn main() {
    let nums = [2,3,5,7].to_vec();
    println!("{:?}", min_bitwise_array(nums));
}
