fn minimum_sum_subarray(nums: Vec<i32>, l: i32, r: i32) -> i32 {
    let l = l as usize;
    let r = r as usize;
    let mut prefix = Vec::new();
    let mut res = i32::MAX;
    for &num in &nums {
        prefix.push(num + prefix.last().unwrap_or(&0));
    }

    let mut i = 0;
    for (j, &num) in nums.iter().enumerate() {
        if j >= r {
            i += 1;
        }

        if j >= l-1 {
            let mut temp_i = i;
            while j - temp_i + 1 >= l {
                let sum = prefix[j] - (if temp_i == 0 { 0 } else { prefix[temp_i-1] });
                if sum > 0 {
                    res = res.min(sum);
                }
                temp_i += 1;
            }
        }
    }

    if res == i32::MAX { -1 } else { res }
}

pub fn main() {
    let nums = [3, -2, 1, 4].to_vec();
    let l = 2;
    let r = 3;
    println!("{}", minimum_sum_subarray(nums, l, r));
}
