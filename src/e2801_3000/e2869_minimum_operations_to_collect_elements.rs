fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
    let target = (1u64 << k as u64) - 1;
    let mut mask = 0;
    let mut ops = 0;
    for i in (0..nums.len()).rev() {
        ops += 1;
        if nums[i] <= k {
            mask |= 1 << (nums[i] as u64 -1);
        }
        if target == mask {
            return ops;
        }
    }

    ops
}

pub fn main() {
    // let nums = [3,1,5,4,2].to_vec();
    let nums = [18,31,18,22,14,27,19,28,4,33,33,17,44,16,20,8,40,5,42,4,26,19,23,15,24,32,29,1,2,30,22,46,10,12,6,32,35,13,30,21,46,11,25,7,3,11,34,9,15,28].to_vec();
    let k = 35;
    println!("{}", min_operations(nums, k));
}
