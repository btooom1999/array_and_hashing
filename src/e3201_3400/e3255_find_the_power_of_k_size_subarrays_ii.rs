fn results_array(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let k = k as usize;
    let mut error = -1;
    let mut i = 0;
    let mut res = Vec::new();
    for j in 0..nums.len() {
        if j > 0 && nums[j-1]+1 != nums[j] {
            error = (j-1) as i32;
        }

        if j >= k {
            if i == error {
                error = -1;
            }
            i += 1;
        }

        if j >= k-1 {
            if error == -1 {
                res.push(nums[j]);
            } else {
                res.push(-1);
            }
        }
    }

    res
}

pub fn main() {
    let nums = [3,2,3,2,3,2].to_vec();
    let k = 2;
    println!("{:?}", results_array(nums, k));
}
