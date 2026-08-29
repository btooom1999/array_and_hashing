fn sub_array_ranges(nums: Vec<i32>) -> i64 {
    let mut max_stack = Vec::<(i64, usize)>::new();
    let mut min_stack = Vec::<(i64, usize)>::new();
    let mut summed_max_stack = 0i64;
    let mut summed_min_stack = 0i64;
    let mut max_sum = 0i64;
    let mut min_sum = 0i64;

    for num in &nums {
        let num = *num as i64;
        let mut count = 1;
        while let Some(&(v, c)) = max_stack.last() {
            if v > num {
                break;
            }
            count += c;
            summed_max_stack -= v * c as i64;
            max_stack.pop();
        }
        max_stack.push((num, count));
        max_sum += num + (num * (count as i64 - 1)) + summed_max_stack;
        summed_max_stack += num * count as i64;

        count = 1;
        while let Some(&(v, c)) = min_stack.last() {
            if v < num {
                break;
            }
            count += c;
            summed_min_stack -= v * c as i64;
            min_stack.pop();
        }
        min_stack.push((num, count));
        min_sum += num + (num * (count as i64 - 1)) + summed_min_stack;
        summed_min_stack += num * count as i64;
    }

    max_sum - min_sum
}

pub fn main() {
    let nums = vec![4,-2,-3,4,1];
    println!("{}", sub_array_ranges(nums));
}
