fn maximum_swap(num: i32) -> i32 {
    let mut nums = num.to_string().into_bytes();
    let mut sorted_nums = nums.clone();
    sorted_nums.sort_by(|a, b| b.cmp(a));

    let mut res = 0;
    let n = nums.len();
    let mut swap = false;
    for i in 0..n {
        let mut value = nums[i];
        if !swap && nums[i] != sorted_nums[i] {
            value = sorted_nums[i];
            let mut j = n-1;
            while nums[j] != sorted_nums[i] {
                j -= 1;
            }
            nums[j] = nums[i];
            swap = true;
        }

        res = res * 10 + (value - b'0') as i32;
    }

    res
}

pub fn main() {
    let num = 9937;
    println!("{}", maximum_swap(num));
}
