use std::collections::HashMap;

fn find_lucky(mut arr: Vec<i32>) -> i32 {
    arr.sort();

    let mut checking_number = arr.first().unwrap_or(&-1);
    let mut count = 0;
    let mut res = -1;
    for num in &arr {
        if checking_number == num {
            count += 1;
        } else {
            if count == *checking_number {
                res = *checking_number;
            }
            count = 1;
            checking_number = num;
        }
    }

    if *checking_number == count {
        *checking_number
    } else {
        res
    }
}

pub fn main() {
    let arr = vec![
        1, 13, 1, 5, 14, 1, 18, 20, 20, 15, 2, 1, 3, 6, 2, 19, 13, 3, 18, 16, 18, 2, 1, 10, 9, 2,
        19, 12, 5, 19, 7, 4, 4, 6, 19, 2, 3, 13, 18, 18, 16, 18, 16, 16, 9, 2, 17, 11, 2, 4, 7, 18,
        13, 14, 4, 15, 14, 10, 17, 11, 14, 1, 7, 10, 12, 10, 9, 1, 11,
    ];
    println!("{}", find_lucky(arr));
}
