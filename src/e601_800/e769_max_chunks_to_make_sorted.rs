fn max_chunks_to_sorted(arr: Vec<i32>) -> i32 {
    let mut res = 0;
    let mut sum1 = 0;
    let mut sum2 = 0;
    for i in 0..arr.len() {
        sum1 += i as i32;
        sum2 += arr[i];

        if sum1 == sum2 {
            res += 1;
            sum1 = 0;
            sum2 = 0;
        }
    }

    res
}

pub fn main() {
    let arr = [4,3,2,1,0].to_vec();
    println!("{}", max_chunks_to_sorted(arr));
}
