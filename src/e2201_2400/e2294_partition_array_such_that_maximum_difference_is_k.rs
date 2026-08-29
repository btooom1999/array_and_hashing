fn partition_array(mut nums: Vec<i32>, k: i32) -> i32 {
    nums.sort();

    let mut selected_num = nums[0];
    let mut count = 1;

    for &num in &nums {
        if num - selected_num > k {
            selected_num = num;
            count += 1;
        }
    }

    count
}

pub fn main() {
    let nums = [3,6,1,2,5].to_vec();
    let k = 2;
    println!("{}", partition_array(nums, k));
}
