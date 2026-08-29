use std::collections::HashMap;

fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut hashmap = HashMap::<i32, i32>::new();
    let mut stack = Vec::new();
    for num1 in nums2.iter().rev() {
        while let Some(num2) = stack.last() {
            if *num2 <= *num1 {
                stack.pop();
            } else {
                break;
            }
        }

        hashmap.insert(*num1, *stack.last().unwrap_or(&-1));
        stack.push(*num1);
    }

    nums1.into_iter().map(|v| *hashmap.get(&v).unwrap()).collect::<Vec<_>>()
}

pub fn main() {
    let nums1 = vec![4,1,2];
    let nums2 = vec![1,3,4,2];
    println!("{:?}", next_greater_element(nums1, nums2));
}
