fn max_score_sightseeing_pair(values: Vec<i32>) -> i32 {
    let mut max = i32::MIN;
    let mut res = i32::MIN;

    for (i, num) in values.into_iter().enumerate() {
        let i = i as i32;
        res = res.max(num-i + max);
        if num+i > max {
            max = num+i;
        }
    }

    res
}

pub fn main() {
    let values = [8,1,5,2,6].to_vec();
    println!("{:?}", max_score_sightseeing_pair(values));
}
