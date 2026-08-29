fn prefixes_div_by5(nums: Vec<i32>) -> Vec<bool> {
    let n = nums.len();
    let mut num = 0;
    let mut res = vec![false; n];
    for i in 0..n {
        num = (num << 1) | nums[i];
        num %= 5;
        if num % 5 == 0 {
            res[i] = true;
        }
    }

    res
}

pub fn main() {
    // let nums = [0,1,1].to_vec();
    let nums = [1,0,0,1,0,1,0,0,1,0,1,1,1,1,1,1,1,1,1,1,0,0,0,0,1,0,1,0,0,0,0,1,1,0,1,0,0,0,1].to_vec();
    println!("{:?}", prefixes_div_by5(nums));
}
