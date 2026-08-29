fn final_prices(prices: Vec<i32>) -> Vec<i32> {
    let mut res = vec![-1; prices.len()];
    let mut stack = Vec::<i32>::new();
    for (i, price) in prices.iter().enumerate().rev() {
        while let Some(val) = stack.last() {
            if val > price {
                stack.pop();
            } else {
                break;
            }
        }

        let discount = *stack.last().unwrap_or(&0);
        res[i] = *price - discount;
        stack.push(*price);
    }

    res
}

pub fn main() {
    let prices = vec![8,4,6,2,3];
    println!("{:?}", final_prices(prices));
}
