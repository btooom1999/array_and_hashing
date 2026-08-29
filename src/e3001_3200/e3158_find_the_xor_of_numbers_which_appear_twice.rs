fn duplicate_numbers_xor(nums: Vec<i32>) -> i32 {
    let mut appears = vec![0; 51];
    let mut res = 0;
    for num in nums {
        let k = num as usize;
        appears[k] += 1;
        if appears[k] == 2 {
            res ^= num;
        }
    }

    res
}

pub fn main() {
    let nums = [1,2,1,3].to_vec();
    println!("{}", duplicate_numbers_xor(nums));
}
