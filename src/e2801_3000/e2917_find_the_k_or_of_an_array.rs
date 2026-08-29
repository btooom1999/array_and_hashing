fn find_k_or(nums: Vec<i32>, k: i32) -> i32 {
    let mut ones = vec![0; 31];
    for mut num in nums {
        let mut i = 0;
        while num > 0 {
            if 1 & num == 1 {
                ones[i] += 1;
            }
            num >>= 1;
            i += 1;
        }
    }

    let mut mask = 0;
    for count in ones.into_iter().rev() {
        let mut bit = 0;
        if count >= k {
            bit = 1;
        }

        mask = (mask << 1) | bit;
    }

    mask
}

pub fn main() {
    let nums = [22,7,27,30,15,30,28].to_vec();
    let k = 4;
    println!("{:?}", find_k_or(nums, k));
}
