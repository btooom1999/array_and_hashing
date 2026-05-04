fn largest_number(mut nums: Vec<i32>) -> String {
    nums.sort_by(|a, b| {
        let mut a = a.to_string().into_bytes();
        let mut b = b.to_string().into_bytes();

        loop {
            let n1 = a.len();
            let n2 = b.len();
            let mut i = 0;
            let mut j = 0;
            while i < n1 && j < n2 && a[i] == b[j] {
                i += 1;
                j += 1;
            }
            if i == n1 && j == n2 {
                return std::cmp::Ordering::Equal;
            } else if i == n1 {
                b = b[n1..].to_vec();
            } else if j == n2 {
                a = a[n2..].to_vec();
            } else {
                return b[j].cmp(&a[i]);
            }
        }
    });

    if nums[0] == 0 {
        return "0".to_string();
    }

    nums
        .into_iter()
        .fold(String::new(), |mut acc, num| {
            acc.push_str(&num.to_string());
            acc
        })
}

pub fn main() {
    // let nums = [34232, 3432].to_vec();
    let nums = vec![3,30,34,5,9];
    println!("{:?}", largest_number(nums));
}
