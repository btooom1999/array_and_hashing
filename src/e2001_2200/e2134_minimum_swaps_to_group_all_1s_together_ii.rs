fn min_swaps(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut ones = 0;
    for &num in &nums {
        ones += (num == 1) as usize;
    }

    let mut zeroes = 0;
    let mut i = 0;
    let mut res = i32::MAX;
    for j in 0..n+ones {
        let num = nums[j%n];
        zeroes += (num == 0) as i32;

        if j-i+1 > ones {
            zeroes -= (nums[i%n] == 0) as i32;
            i += 1;
        }

        if j-i+1 >= ones {
            res = res.min(zeroes);
        }
    }

    res
}

pub fn main() {
    let nums = [0,1,0,1,1,0,0].to_vec();
    println!("{:?}", min_swaps(nums));
}
