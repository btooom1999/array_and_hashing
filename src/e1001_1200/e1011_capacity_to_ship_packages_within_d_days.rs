fn ship_within_days(weights: Vec<i32>, days: i32) -> i32 {
    let mut l = *weights.iter().max().unwrap();
    let mut r = weights.iter().sum::<i32>();

    while l <= r {
        let m = (l + r) / 2;
        let mut d = 1;
        let mut capacity = 0;
        for &weight in &weights {
            if capacity + weight > m {
                capacity = weight;
                d += 1;
            } else {
                capacity += weight;
            }
        }

        if d <= days {
            r = m - 1;
        } else {
            l = m + 1;
        }
    }

    l
}

pub fn main() {
    // let weights = [1,2,3,4,5,6,7,8,9,10].to_vec();
    // let days = 5;
    let weights = [1,2,3,1,1].to_vec();
    let days = 4;
    println!("{}", ship_within_days(weights, days));
}


