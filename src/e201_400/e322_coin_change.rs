use std::collections::HashMap;

fn dfs(
    coins: &Vec<i32>,
    dp: &mut HashMap<i32, i32>,
    amount: i32,
) -> i32 {
    if amount == 0 {
        return 0;
    }

    if let Some(&min) = dp.get(&amount) {
        return min;
    }

    let mut res = 1e9 as i32;
    for i in 0..coins.len() {
        if amount - coins[i] >= 0 {
            res = res.min(1+dfs(coins, dp, amount - coins[i]));
        }
    }
    dp.insert(amount, res);
    res
}

fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    let res = dfs(&coins, &mut HashMap::new(), amount);
    if res == 1e9 as i32 {
        -1
    } else {
        res
    }
}

pub fn main() {
    let coins = [1,2,5];
    let amount = 11;
    println!("{}", coin_change(coins.into(), amount));
}
