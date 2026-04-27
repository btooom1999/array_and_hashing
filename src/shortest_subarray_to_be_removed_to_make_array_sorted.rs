fn find_length_of_shortest_subarray(arr: Vec<i32>) -> i32 {
    let n = arr.len();
    let mut prefix_increasing = (i32::MIN, 0);
    let mut suffix_decreasing = (i32::MAX, n);
    for i in 0..n {
        if arr[i] >= prefix_increasing.0 {
            prefix_increasing.0 = arr[i];
            prefix_increasing.1 += 1_usize;
        } else {
            prefix_increasing.0 = i32::MAX;
        }

        if arr[n-i-1] <= suffix_decreasing.0 {
            suffix_decreasing.0 = arr[n-i-1];
            suffix_decreasing.1 -= 1_usize;
        } else {
            suffix_decreasing.0 = i32::MIN;
        }
    }

    let mut res = (n - prefix_increasing.1).min(suffix_decreasing.1);
    let mut x = suffix_decreasing.1;
    for i in 0..prefix_increasing.1 {
        for j in x..n {
            if arr[i] <= arr[j] {
                res = res.min(j-i-1);
                x = j;
                break;
            }
        }
    }

    res as i32
}

pub fn main() {
    // let arr = [1,2,3,3,2,1].to_vec();
    let arr = [1,2,3,10,4,2,3,5].to_vec();
    println!("{}", find_length_of_shortest_subarray(arr));
}
