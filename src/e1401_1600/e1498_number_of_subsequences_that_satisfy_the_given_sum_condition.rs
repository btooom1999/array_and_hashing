const MOD: i64 = 1_000_000_007;

fn num_subseq(mut nums: Vec<i32>, target: i32) -> i32 {
    nums.sort();

    let n = nums.len();
    let mut i = 0;
    let mut j = nums.len() as i32 - 1;
    let mut res = 0;

    let mut pow2 = vec![1; n+1];
    for i in 1..n+1 {
        pow2[i] = (pow2[i - 1] * 2) % MOD;
    }

    while i <= j {
        if nums[i as usize] + nums[j as usize] <= target {
            res = (res + pow2[(j - i) as usize]) % MOD;
            i += 1;
        } else {
            j -= 1;
        }
    }

    res as i32
}


pub fn main() {
    // let nums = vec![14,4,6,6,20,8,5,6,8,12,6,10,14,9,17,16,9,7,14,11,14,15,13,11,10,18,13,17,17,14,17,7,9,5,10,13,8,5,18,20,7,5,5,15,19,14];
    let nums = vec![5, 5, 5];
    let target = 10;
    println!("{}", num_subseq(nums, target));
}
