use std::collections::HashSet;

fn longest_common_prefix(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
    let mut hashset = HashSet::new();
    for mut num in arr1 {
        while num > 0 {
            hashset.insert(num);
            num /= 10;
        }
    }

    let mut max = 0;
    for mut num in arr2 {
        while num > 0 {
            if hashset.contains(&num) {
                max = std::cmp::max(max, num.to_string().len());
                break;
            }
            num /= 10;
        }
    }

    max as _
}

pub fn main() {
    let arr1 = [1,10,100];
    let arr2 = [1000];

    println!("{:?}", longest_common_prefix(arr1.into(), arr2.into()));
}
