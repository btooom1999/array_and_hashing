fn repair_cars(ranks: Vec<i32>, cars: i32) -> i64 {
    let mut l = 1;
    let mut r = ranks[0] as i64 * cars as i64 * cars as i64;

    let mut res = -1;

    while l <= r {
        let m = (l + r) / 2;
        let mut count_repaired = 0;
        for &rank in &ranks {
            count_repaired += (m as f64 / rank as f64).sqrt().floor() as i64;
        }

        if count_repaired >= cars as i64 {
            res = m;
            r = m - 1;
        } else {
            l = m + 1;
        }
    }

    res
}

pub fn main() {
    let ranks = [4,2,3,1].to_vec();
    let cars = 10;
    println!("{}", repair_cars(ranks, cars));
}
