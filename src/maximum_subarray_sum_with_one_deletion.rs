fn maximum_sum(arr: Vec<i32>) -> i32 {
    let mut sum_with_skip = arr[0];
    let mut sum_no_skip = arr[0];
    let mut res = arr[0];

    for i in 1..arr.len() {
        if sum_with_skip < 0 {
            sum_with_skip = 0;
        }

        if arr[i] >= 0 {
            sum_with_skip += arr[i];
        } else {
            sum_with_skip = sum_no_skip.max(sum_with_skip + arr[i]);
        }

        if sum_no_skip < 0 {
            sum_no_skip = 0;
        }

        sum_no_skip += arr[i];

        res = res.max(sum_no_skip).max(sum_with_skip);
    }

    res
}

pub fn main() {
    let arr = [-1,-1,-1,-1].to_vec();
    println!("{}", maximum_sum(arr));
}
