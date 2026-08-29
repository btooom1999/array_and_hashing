fn shuffle(mut nums: Vec<i32>, n: i32) -> Vec<i32> {
    let n = n as usize;
    for i in 0..n {
        nums[i] = nums[i] << 10 | nums[i+n];
    }

    let mut j = nums.len()-1;
    for i in (0..n).rev() {
        let y = nums[i] & ((1 << 10)-1);
        let x = nums[i] >> 10;
        nums[j] = y;
        nums[j-1] = x;
        j = j.wrapping_sub(2);
    }

    nums
}

pub fn main() {
    let nums = [2,5,1,3,4,7].to_vec();
    let n = 3;
    println!("{:?}", shuffle(nums, n));
}
