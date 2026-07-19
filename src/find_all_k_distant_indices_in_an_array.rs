fn find_k_distant_indices(nums: Vec<i32>, key: i32, k: i32) -> Vec<i32> {
    let mut indices = Vec::new();
    let n = nums.len();
    for i in 0..n {
        if nums[i] == key { indices.push(i as i32) };
    }

    let mut res = Vec::new();
    for i in 0..n {
        if indices.iter().any(|&num| (i as i32 - num).abs() <= k) {
            res.push(i as i32);
        }
    }

    res
}

pub fn main() {
    let nums = [3,4,9,1,3,9,5].to_vec();
    let key = 9;
    let k = 1;
    println!("{:?}", find_k_distant_indices(nums, key, k));
}
