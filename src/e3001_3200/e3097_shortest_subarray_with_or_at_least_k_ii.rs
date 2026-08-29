fn check(bits: &[i32], k: i32) -> bool {
    for i in (0..32).rev() {
        let val1 = (bits[i] > 0) as i32;
        let val2 = (k >> i) & 1;
        if val1 > val2 {
            return true;
        } else if val1 < val2 {
            return false;
        }
    }

    true
}

fn minimum_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
    let mut bits = [0; 32];
    let mut i = 0;
    let mut res = i32::MAX;

    for j in 0..nums.len() {
        for x in 0..32 {
            bits[x] += (nums[j] >> x) & 1;
        }

        while i <= j && check(&bits, k) {
            res = res.min((j-i+1) as i32);
            for x in 0..32 {
                bits[x] -= (nums[i] >> x) & 1;
            }
            i += 1;
        }
    }

    if res == i32::MAX { -1 } else { res }
}

pub fn main() {
    let nums = [536870912, 268435456, 134217728, 67108864, 33554432, 16777216, 8388608, 4194304, 2097152, 1048576, 524288, 262144, 131072, 65536, 32768, 16384, 8192, 4096, 2048, 1024, 512, 256, 128, 64, 32, 16, 8, 4, 2, 1, 0, 0].to_vec();
    let k = 1000000000;
    println!("{}", minimum_subarray_length(nums, k));
}
