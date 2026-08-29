fn rotate_elements(mut nums: Vec<i32>, mut k: i32) -> Vec<i32> {
    let mut negatives = Vec::new();
    for &num in &nums {
        if num >= 0 {
            negatives.push(num);
        }
    }

    if negatives.is_empty() {
        return nums;
    }

    let n = negatives.len();
    k %= negatives.len() as i32;
    negatives.rotate_left(n.min(k as usize));

    let mut i = 0;
    for num in nums.iter_mut() {
        if *num >= 0 {
            *num = negatives[i];
            i += 1;
        }
    }

    nums
}

pub fn main() {
    let nums = [1,-2,3,-4,0].to_vec();
    let k = 1;
    println!("{:?}", rotate_elements(nums, k));
}
