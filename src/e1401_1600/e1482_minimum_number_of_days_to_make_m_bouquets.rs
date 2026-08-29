fn min_days(bloom_day: Vec<i32>, m: i32, k: i32) -> i32 {
    if (bloom_day.len() as i64) < m as i64 * k as i64 {
        return -1;
    }

    let mut l = 1;
    let mut r = *bloom_day.iter().max().unwrap();

    while l < r {
        let mid = (l + r) / 2;
        let mut temp_m = 0;
        let mut temp_k = 0;
        for &day in &bloom_day {
            if day <= mid {
                temp_k += 1;
            } else {
                temp_k = 0;
            }

            if temp_k == k {
                temp_m += 1;
                temp_k = 0;
            }
        }

        if temp_m < m {
            l = mid + 1;
        } else {
            r = mid;
        }
    }

    l
}

pub fn main() {
    let bloom_day = [1,10,3,10,2].to_vec();
    // let bloom_day = [7,7,7,7,12,7,7].to_vec();
    let m = 3;
    let k = 2;
    println!("{}", min_days(bloom_day, m, k));
}
