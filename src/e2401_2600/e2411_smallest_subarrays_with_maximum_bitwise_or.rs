fn smallest_subarrays(nums: Vec<i32>) -> Vec<i32> {
    let mut hashmap = vec![0; 32];
    let n = nums.len();
    let mut res = vec![1; n];
    for i in (0..n).rev() {
        for at in 0..32 {
            if 1 << at & nums[i] == 0 && hashmap[at] > 0 {
                res[i] = res[i].max((hashmap[at] - i + 1) as i32);
            }
        }

        for at in 0..32 {
            if 1 << at & nums[i] > 0 {
                hashmap[at] = i;
            }
        }
    }

    res
}

fn generate_random_numbers() -> Vec<i32> {
    (0..100_001).map(|_| rand::random_range(0..100_000_001)).collect()
}

pub fn main() {
    // let nums = [1,0,2,1,3].to_vec();
    let nums = generate_random_numbers();
    println!("{:?}", smallest_subarrays(nums));
}
