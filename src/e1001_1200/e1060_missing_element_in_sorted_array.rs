fn missing_element(nums: Vec<i32>, k: i32) -> i32 {
    let mut buffer = 0;
    let mut l = 0;
    let mut r = nums.len() - 1;
    let mut k = k;
    while l <= r {
        let m = (l + r) / 2;
        let true_val = nums[0] + m as i32 + buffer;
        let val = nums[m];
        if k - (val - true_val) > 0 {
            let step = val - true_val;
            buffer += step;
            k -= step;

            l = m + 1;
        } else if k - (val - true_val) <= 0 {
            r = m - 1;
        }
    }

    nums[r] + k
}

pub fn main() {
    let nums = [1,2,4].to_vec();
    let k = 3;
    println!("{}", missing_element(nums, k));
}
