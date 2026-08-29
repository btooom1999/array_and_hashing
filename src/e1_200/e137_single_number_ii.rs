fn single_number(nums: Vec<i32>) -> i32 {
    let mut res = 0;
    for i in (0..32).rev() {
        let mut count = 0;
        for &num in &nums {
            if (num >> i) & 1 == 1 {
                count += 1;
            }
        }

        let mut bit = 1;
        if count % 3 == 0 {
            bit = 0;
        }
        res = (res << 1) | bit;
    }

    res
}

pub fn main() {
    let nums = [2,2,3,2].to_vec();
    println!("{}", single_number(nums));
}
