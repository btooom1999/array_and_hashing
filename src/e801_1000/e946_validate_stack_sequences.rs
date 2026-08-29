fn validate_stack_sequences(pushed: Vec<i32>, popped: Vec<i32>) -> bool {
    let mut stack = Vec::new();
    let mut i = 0;
    for num in &pushed {
        if *num == popped[i] {
            i += 1;
            continue;
        }

        while stack.last().is_some_and(|v| *v == popped[i]) {
            i += 1;
            stack.pop();
        }
        stack.push(*num);
    }

    while let Some(val) = stack.pop() {
        if val != popped[i] {
            return false;
        }

        i += 1;
    }

    true
}

pub fn main() {
    let pushed = vec![2, 1, 0];
    let popped = vec![1, 2, 0];
    println!("{}", validate_stack_sequences(pushed, popped));
}
