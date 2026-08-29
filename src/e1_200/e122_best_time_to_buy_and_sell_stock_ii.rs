fn max_profit(prices: Vec<i32>) -> i32 {
    let mut sum = 0;
    for i in 1..prices.len() {
        if prices[i] > prices[i - 1] {
            sum += prices[i] - prices[i - 1];
        }
    }

    sum
}

pub fn main() {
    let prices = vec![7, 1, 5, 3, 6, 4];
    println!("{}", max_profit(prices));
}
