fn buy_choco(mut prices: Vec<i32>, money: i32) -> i32 {
    prices.sort();

    let mut res = money;
    res -= prices[0];
    res -= prices.get(1).unwrap_or(&i32::MAX);

    if res < 0 {
        return money;
    }

    res
}

pub fn main() {
    let prices = [1,1,2].to_vec();
    let money = 3;
    println!("{}", buy_choco(prices, money));
}
