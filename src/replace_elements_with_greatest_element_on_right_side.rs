fn replace_elements(arr: Vec<i32>) -> Vec<i32> {
    let mut res = vec![-1; arr.len()];
    let mut max_val = arr[arr.len() - 1];

    for i in (0..arr.len() - 1).rev() {
        res[i] = max_val;
        max_val = std::cmp::max(max_val, arr[i]);
    }

    res
}

pub fn main() {
    let arr = vec![17,18,5,4,6,1];
    println!("{:?}", replace_elements(arr));
}
