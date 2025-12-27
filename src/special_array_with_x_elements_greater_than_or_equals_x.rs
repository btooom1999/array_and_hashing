fn special_array(mut sums: Vec<i32>) -> i32 {
    sums.sort();

    let mut i = 0;
    let mut x = 0;
    loop {
        if x > sums.len() as i32 || i > sums.len() - 1 {
            return -1;
        }

        if sums[i] >= x {
            if x == (sums.len() - i) as i32 {
                return x;
            } else {
                x += 1;
            }
        } else {
            i += 1;
        }
    }
}

pub fn main() {
    let nums = vec![3, 5];
    println!("{}", special_array(nums));
}
