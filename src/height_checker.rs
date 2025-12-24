use std::collections::HashMap;

fn height_checker(heights: Vec<i32>) -> i32 {
    let mut expected = heights.clone();
    expected.sort();

    let mut res = 0;
    for (val1, val2) in heights.iter().zip(expected.iter()) {
        if val1 != val2 {
            res += 1;
        }
    }

    res
}

pub fn main() {
    //let heights = vec![1, 2, 3, 4, 5];
    let heights = vec![
        23, 52, 46, 7, 50, 87, 20, 32, 85, 65, 62, 34, 8, 86, 15, 66, 66, 30, 11, 96, 18, 26, 24,
        10, 57, 13, 37, 69, 85, 6, 8, 17, 40, 88, 14, 72, 85, 51, 40, 38, 54, 65, 65, 27, 18, 59,
        77, 12, 25, 46, 10, 19, 10, 28, 64, 79, 5, 88, 2, 1, 14, 50, 91, 34, 58, 32, 90, 67, 28,
        81, 84, 76, 88, 45, 42, 54, 59, 56, 20, 6, 56, 51, 72, 69, 6, 48, 67, 68, 6, 10, 93, 69, 4,
        29, 28,
    ];
    println!("{}", height_checker(heights));
}
