fn find_min(nums: Vec<i32>) -> i32 {
    let n = nums.len() - 1;
    let mut l = 0;
    let mut r = n;

    while l < r {
        let m = (l + r) / 2;
        if nums[m] < nums[r] {
            r = m;
        } else {
            l = m + 1;
        }
    }


    nums[l]
}

pub fn main() {
    let nums = [1,2].to_vec();
    println!("{}", find_min(nums));
}
