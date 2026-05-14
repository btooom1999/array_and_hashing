fn max_profit(prices: Vec<i32>) -> i32 {
    let mut min = prices[0];
    let mut res = 0;
    for i in 1..prices.len() {
        if prices[i] < min {
            min = prices[i];
        }
        res = res.max(prices[i]-min);
    }

    res
}

pub fn main() {
    let prices = vec![7, 1, 5, 3, 6, 4];
    println!("{}", max_profit(prices));
}
