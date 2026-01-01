fn max_profit(prices: Vec<i32>) -> i32 {
    let mut profit = 0;
    let mut buy = prices[0];

    for price in prices.iter().skip(1) {
        if buy > *price {
            buy = *price;
        } else {
            profit = std::cmp::max(profit, price - buy);
        }
    }

    profit
}

pub fn main() {
    let prices = vec![7, 1, 5, 3, 6, 4];
    println!("{}", max_profit(prices));
}
