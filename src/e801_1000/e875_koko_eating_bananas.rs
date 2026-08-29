fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
    let mut l = 1;
    let mut r = *piles.iter().max().unwrap();

    while l <= r {
        let m = (l + r) / 2;
        let mut hours = 0i64;
        for pile in &piles {
            let h = pile / m;
            let m = if pile % m == 0 { 0 } else { 1 };
            hours += h as i64 + m as i64;
        }

        if hours <= h as i64 {
            r = m - 1;
        } else {
            l = m + 1;
        }
    }

    l
}


pub fn main() {
    let piles = [805306368,805306368,805306368].to_vec();
    let h = 1000000000;
    // let piles = [312884470].to_vec();
    // let h = 312884469;
    println!("{}", min_eating_speed(piles, h));
}
