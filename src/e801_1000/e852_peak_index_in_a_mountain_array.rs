fn peak_index_in_mountain_array(arr: Vec<i32>) -> i32 {
    let n = arr.len();
    let mut l = 1;
    let mut r = n-1;
    while l<r {
        let m = (l+r)/2;
        if m>0 && arr[m-1] > arr[m] {
            r = m-1;
        } else if m+1<n && arr[m] < arr[m+1] {
            l = m+1;
        } else {
            return m as i32;
        }
    }

    l as i32
}

pub fn main() {
    let arr = [18,29,38,59,98,100,99,98,90].to_vec();
    println!("{}", peak_index_in_mountain_array(arr));
}
