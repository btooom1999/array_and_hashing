fn find_min(nums: Vec<i32>) -> i32 {
    let mut l = 0;
    let mut r = nums.len()-1;
    while l < r {
        let m = (l+r)/2;
        if nums[m] > nums[r] {
            l = m + 1;
        } else if nums[m] < nums[r] {
            r = m;
        } else {
            r -= 1;
        }
    }

    nums[l]
}

pub fn main() {
    // let nums = [10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,1,10,10,10].to_vec();
    let nums = [2,0,0,0,1,2].to_vec();
    println!("{}", find_min(nums));
}
