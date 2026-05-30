fn good_days_to_rob_bank(security: Vec<i32>, time: i32) -> Vec<i32> {
    let n = security.len();
    let mut left = vec![0; n+1];

    for i in 1..n {
        if security[i-1] >= security[i] {
            left[i] = left[i-1] + 1;
        }
    }

    let mut res = Vec::new();
    let mut right = 0;
    for i in (0..n).rev() {
        if right.min(left[i]) >= time {
            res.push(i as i32);
        }
        if i > 0 && security[i] >= security[i-1] {
            right += 1;
        } else {
            right = 0;
        }
    }

    res
}

pub fn main() {
    let security = [1,1,1,1,1].to_vec();
    let time = 0;
    println!("{:?}", good_days_to_rob_bank(security, time));
}
