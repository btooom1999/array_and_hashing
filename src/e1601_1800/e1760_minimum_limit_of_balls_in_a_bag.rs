fn minimum_size(nums: Vec<i32>, max_operations: i32) -> i32 {
    let mut l = 1;
    let mut r = *nums.iter().max().unwrap();

    while l <= r {
        let m = (l + r) / 2;
        let mut count_operations = max_operations;
        for &num in &nums {
            count_operations -= (num + m - 1) / m - 1;
            if count_operations < 0 {
                break;
            }
        }

        if count_operations >= 0 {
            r = m - 1;
        } else {
            l = m + 1;
        }
    }

    l
}

pub fn main() {
    let nums = [2,4,8,2].to_vec();
    let max_operations = 4;
    // let nums = [1000000000,1000000000,1000000000].to_vec();
    // let max_operations = 1000000000;
    println!("{}", minimum_size(nums, max_operations));
}

