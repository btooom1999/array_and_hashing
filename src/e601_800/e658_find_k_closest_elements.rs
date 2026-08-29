fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
    let k = k as usize;
    let mut l = 0;
    let mut r = arr.len() - k;

    while l < r {
        let m = (l + r) / 2;
        if x - arr[m] > arr[m+k] - x {
            l = m + 1;
        } else {
            r = m;
        }
    }

    arr[l..l+k].to_vec()
}

pub fn main() {
    let arr = [1,2,3,4,5].to_vec();
    let k = 4;
    let x = 3;
    println!("{:?}", find_closest_elements(arr, k, x));
}
