fn find_length_of_lcis(nums: Vec<i32>) -> i32 {
    let mut res = 1;
    let mut len = 1;
    for i in 1..nums.len() {
        if nums[i] > nums[i-1] {
            len += 1;
            res = res.max(len);
        } else {
            len = 1;
        }
    }

    res
}

pub fn main() {
    let nums = [1,3,5,4,7].to_vec();
    println!("{}", find_length_of_lcis(nums))
}
