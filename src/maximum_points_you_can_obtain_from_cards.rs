fn max_score(card_points: Vec<i32>, k: i32) -> i32 {
    let n = card_points.len();
    let k = k as usize;
    let mut prefix = vec![0; card_points.len()+1];
    for i in 0..card_points.len() {
        prefix[i+1] = card_points[i] + prefix[i];
    }

    let mut res = 0;
    for (j, i) in (1..=k).rev().enumerate() {
        let val1 = prefix[i]+prefix[n]-prefix[n-j];
        let val2 = prefix[n]-prefix[n-i]+prefix[j];
        res = res.max(val1.max(val2));
    }

    res
}

pub fn main() {
    let card_points = [1,2,3,4,5,6,1].to_vec();
    let k = 3;
    println!("{}", max_score(card_points, k))
}
