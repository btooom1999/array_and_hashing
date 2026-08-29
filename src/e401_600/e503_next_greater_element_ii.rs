fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
    let mut res = vec![-1; nums.len()];
    let mut stack = Vec::<usize>::new();
    for k in 0..(2 * nums.len()) {
        let i = k % nums.len();
        while let Some(last) = stack.last() {
            if nums[i] > nums[*last] {
                res[*last] = nums[i];
                stack.pop();
            } else {
                break;
            }
        }

        if k < nums.len() {
            stack.push(i);
        }
    }
    res
}

pub fn main() {
    let nums = vec![1,2,3,2,1];
    println!("{:?}", next_greater_elements(nums));
}
