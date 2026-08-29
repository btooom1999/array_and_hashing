fn max_chunks_to_sorted(arr: Vec<i32>) -> i32 {
    let mut stack = Vec::new();
    for num in &arr {
        if stack.last().is_none_or(|v| v <= num) {
            stack.push(*num);
        } else {
            let last = stack.pop().unwrap();
            while stack.last().is_some_and(|v| v > num) {
                stack.pop();
            }
            stack.push(last);
        }
    }

    stack.len() as i32
}

pub fn main() {
    let arr = vec![5,4,3,2,1];
    println!("{}", max_chunks_to_sorted(arr));
}
