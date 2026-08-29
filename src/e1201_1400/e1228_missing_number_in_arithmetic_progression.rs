fn missing_number(arr: Vec<i32>) -> i32 {
    let d = (arr.last().unwrap() - arr.first().unwrap()) / arr.len() as i32;
    let mut l = 0;
    let mut r = arr.len() as i32 - 1;

    while l <= r {
        let m = (l + r) / 2;
        let val = arr[0] + m * d;

        if d > 0 {
            if val < arr[m as usize] {
                r = m - 1;
            } else {
                l = m + 1;
            }
        } else if val <= arr[m as usize] {
            l = m + 1;
        } else {
            r = m - 1;
        }
    }

    arr[0] + d * l
}

pub fn main() {
    // let arr = vec![44680,53392,57748,62104,66460,70816,75172,79528,83884];
    let arr = vec![1,1,1,1,1];
    println!("{}", missing_number(arr));
}
