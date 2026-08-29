
fn find132pattern(nums: Vec<i32>) -> bool {
    let mut num3 = i32::MIN;
    let mut stack = Vec::<i32>::new();
    for num in nums.iter().rev() {
        if *num < num3 {
            return true;
        }

        while stack.last().is_some_and(|v| v < num) {
            num3 = std::cmp::max(num3, stack.pop().unwrap());
        }

        stack.push(*num);
    }

    false
}

pub fn main() {
    let nums = vec![1,4,0,-1,-2,-3,0,-1,-2,-4,9];
    println!("{}", find132pattern(nums));
}
