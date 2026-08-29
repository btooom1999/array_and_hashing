fn next_permutation(nums: &mut [i32]) {
    let n = nums.len();
    for i in (1..n).rev() {
        if nums[i-1] < nums[i] {
            let mut j = n-1;
            while nums[j] <= nums[i-1] {
                j -= 1;
            }
            (nums[i-1], nums[j]) = (nums[j], nums[i-1]);
            nums[i..].sort();
            return;
        }
    }

    nums.reverse();
}

pub fn main() {
    let mut nums = [1,1,3,2,1].to_vec();
    next_permutation(&mut nums);
    println!("{:?}", nums);
}
