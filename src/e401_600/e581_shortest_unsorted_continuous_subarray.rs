fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
    let mut cur_max = nums[0];
    let mut i = 1;

    while i < nums.len() {
        if cur_max > nums[i] {
            break;
        }
        cur_max = nums[i];
        i += 1;
    }

    if i == nums.len() {
        return 0;
    }

    let begin = i - 1;
    let mut end = i;
    while i < nums.len() {
        if cur_max > nums[i] {
            end = i;
        }

        cur_max = std::cmp::max(cur_max, nums[i]);
        i += 1;
    }

    let (min, max) = nums[begin..=end].iter().fold(
        (i32::MAX, i32::MIN),
        |acc, value| {
            (acc.0.min(*value), acc.1.max(*value))
        });

    let mut begin = 0;
    let mut end = nums.len() - 1;
    while nums[begin] <= min { begin += 1; }
    while nums[end] >= max { end -= 1; }

    (end - begin + 1) as i32
}

pub fn main() {
    let nums = vec![2,2,6,4,8,10,9,15];
    println!("{}", find_unsorted_subarray(nums));
}
