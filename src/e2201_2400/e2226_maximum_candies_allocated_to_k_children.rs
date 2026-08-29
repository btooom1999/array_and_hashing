fn maximum_candies(candies: Vec<i32>, k: i64) -> i32 {
    let mut l = 1;
    let mut r = candies.iter().max().unwrap().to_owned();
    while l <= r {
        let m = (l + r) / 2;
        let mut sum = 0;
        for &candy in &candies {
            sum += candy as i64 / m as i64;
        }

        if sum >= k {
            l = m + 1;
        } else {
            r = m - 1;
        }
    }

    r
}

pub fn main() {
    let candies = [1,2,3,4,10].to_vec();
    let k = 5;
    println!("{}", maximum_candies(candies, k));
}
