fn max_coins(mut piles: Vec<i32>) -> i32 {
    piles.sort();

    let n = piles.len();
    let mut i = n - 2;
    let mut res = 0;
    for _ in 0..n/3 {
        res += piles[i];
        i -= 2;
    }

    res
}

pub fn main() {
    let piles = [2,4,1,2,7,8].to_vec();
    println!("{}", max_coins(piles));
}
