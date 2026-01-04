fn sum_odd_length_subarrays(arr: Vec<i32>) -> i32 {
    let mut sum = 0;
    for (j, num) in arr.iter().enumerate() {
        let mut i = if j % 2 == 0 { 0 } else { 1 };
        while i <= j {
            sum += arr[i..(j + 1)].iter().sum::<i32>();
            i += 2;
        }
    }

    sum
}

pub fn main() {
    let arr = vec![10, 11, 12];
    println!("{}", sum_odd_length_subarrays(arr));
}
