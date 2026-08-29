fn minimum_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
    let mut res = i32::MAX;

    for i in 0..nums.len() {
        let mut mask = 0;
        for j in i..nums.len() {
            if (j-i+1) as i32 >= res {
                break;
            }

            mask |= nums[j];
            if mask >= k {
                res = (j-i+1) as i32;
            }
        }
    }

    if res == i32::MAX { -1 } else { res }
}

pub fn main() {
    let nums = [1,2,3].to_vec();
    let k = 2;
    println!("{}", minimum_subarray_length(nums, k));
}
